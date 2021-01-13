use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Error, Result};
use dashmap::DashMap;

use crate::domain::payment::Payment;
use crate::domain::storage::data::reservation::{Reservation, ReservationFactoryV1};
use crate::domain::storage::data::user::User;
use crate::domain::storage::reservation::{CreativeStorage, Storage, StorageV1};
use crate::domain::storage::{items, users};
use crate::domain::{ItemToken, ReservationId, UserId, UserToken};
use crate::foundation::errors::{user_not_conformant, user_not_found};
use std::ops::Add;

/// Reserve-Tickets Session.
///
/// # Sync
/// Multiple threads can access the same session without contention.
pub trait Session: Sync {
    /// Start a new reservation for the user. Returns a unique id for identifying the reservation.
    ///
    /// # Temporary Reservation
    /// This call starts a temporary reservation. Until confirmed, the reservation may not be
    /// stored persistently.
    ///
    /// # Error
    /// if the user is not found.
    ///
    /// # Implementation
    /// Specifically, the method collaborates with others in this way:
    /// 1. check with the user storage to see if the user id is valid.
    /// 2. ask a reservation factory to produce a new reservation for the user.
    /// 3. store the new reservation to the active-reservation storage.
    /// 4. return the id of the reservation.
    fn start_reservation(&self, user_id: UserId) -> Result<ReservationId>;

    /// Adds a reservable item to the reservation list.
    ///
    /// Will exclusively occupy the item.
    ///
    /// # Error
    /// - if any of the user, the reservation, or the item is not found;
    /// - if the reservation is already confirmed or not valid (e.g. aborted).
    /// - if the item is occupied, which is possible with multiple users accessing the system.
    fn add_item(&self, token: ItemToken) -> Result<()>;

    /// Removes an item from the list.
    ///
    /// Similar to `add`.
    fn remove_item(&self, token: ItemToken) -> Result<()>;

    /// Gets a summary of the current state of the reservation.
    ///
    /// # Return
    /// a serialized form of the reservation.
    ///
    /// # Error
    /// if the user id is not conformant.
    ///
    /// > <del>
    /// > AFA (Jun. 11 '20), I'm using the YAML form instead of the more famous JSON, since
    /// > YAML is more concise and readable than JSON.
    /// > </del>
    /// >
    /// > I've switched back to JSON since it has the best library support.
    fn summary(&self, token: UserToken) -> Result<String>;

    /// Confirms an reservation.
    ///
    /// Calling this function will terminate the modifying process of the reservation (i.e. `add` or
    /// `remove`). Any subsequent calls to those functions or this one will result in an error.
    ///
    /// # Error
    /// if the reservation is not active.
    fn confirm(&self, token: UserToken) -> Result<()>;

    /// Aborts a reservation that's not paid yet.
    ///
    /// The aborted reservation will not be stored in the system.
    ///
    /// # Error
    /// if the reservation is not active.
    fn abort(&self, token: UserToken) -> Result<()>;

    /// Pays for a reservation.
    ///
    /// # After
    /// calling confirm. If the reservation is not confirmed yet, it will not be
    ///
    /// # Error
    /// - if the payment failed
    /// - if the reservation is already paid.
    /// - if the reservation is not found.
    ///
    /// # Persistent Storage
    /// On success, the function will store the reservation into the user's profile. It is not until
    /// this function is called that a reservation will finally be stored in the user's profile.
    fn pay(&self, token: UserToken, p: Box<dyn Payment>) -> Result<()>;
}

pub struct SessionV1<'a, 'b, 'c> {
    pending_reservations: RwLock<Vec<TempReservation>>,
    active_reservations: StorageV1<'a>,
    users: &'b dyn users::Storage,
    items: &'c dyn items::Storage,
}

impl<'a, 'b, 'c> Session for SessionV1<'a, 'b, 'c> {
    fn start_reservation(&self, user_id: UserId) -> Result<ReservationId> {
        if !self.users.user_exists(&user_id) {
            return Err(user_not_found());
        }

        Ok(self.active_reservations.new_reservation(user_id))
    }

    fn add_item(&self, token: ItemToken) -> Result<()> {
        // Occupy the item first in case it is preempted by others.
        self.items.occupy(token.2)?;

        self.active_reservations
            .authenticated_add_item(token)
            .or_else(|e| {
                self.items.release(token.2);
                Err(e)
            })
    }

    fn remove_item(&self, token: ItemToken) -> Result<()> {
        self.active_reservations.authenticated_remove_item(token)?;
        self.items.release(token.2)
    }

    fn summary(&self, token: UserToken) -> Result<String> {
        self.active_reservations.authenticated_summary(token)
    }

    fn confirm(&self, token: UserToken) -> Result<()> {
        let reservation = self.active_reservations.authenticated_extract(token)?;
        let mut guard = self.pending_reservations.write().unwrap();
        Ok(guard.push(TempReservation::with_wait_time(
            reservation,
            TMP_RSV_TIMEOUT,
        )))
    }

    fn abort(&self, token: UserToken) -> Result<()> {
        self.active_reservations.authenticated_extract(token)?; // drop
        Ok(())
    }

    fn pay(&self, token: UserToken, p: Box<dyn Payment>) -> Result<()> {
        let reservation = self.authenticated_extract_tmp(token)?;
        p.pay(&reservation)?;
        self.users.add_reservation(reservation);
        Ok(())
    }
}

/// Helpers
impl<'a, 'b, 'c> SessionV1<'a, 'b, 'c> {
    /// # Error
    /// - if user not found
    /// - if user not conformant with the reservation
    fn authenticated_extract_tmp(&self, token: UserToken) -> Result<Reservation> {
        let mut reservations = self.pending_reservations.write().unwrap();

        let last = reservations.len() - 1;
        let pos = reservations
            .iter()
            .position(|r| r.reservation.id() == *token.1)
            .ok_or(user_not_found())?;

        reservations.swap(pos, last);

        let ret = Reservation::from(reservations.pop().unwrap());
        if ret.user_id() != token.0 {
            return Err(user_not_conformant());
        }

        Ok(ret)
    }
}

// TODO: Implement wait-up mechanisms
struct TempReservation {
    deadline: Instant,
    reservation: Reservation,
}

impl From<TempReservation> for Reservation {
    /// **Does NOT check if the deadline is passed**
    ///
    /// I could have checked it here, but a problem occurs: What if when externally checking the
    /// deadline it was not passed, yet when extracting it, the reserve is true?
    fn from(t: TempReservation) -> Self {
        t.reservation
    }
}

impl TempReservation {
    /// Create a clock-counted reservation pack with the designated duration.
    fn with_wait_time(reservation: Reservation, wait_for: Duration) -> Self {
        Self {
            deadline: Instant::now().add(wait_for),
            reservation,
        }
    }
}

/// Timeout for a new reservation.
///
/// This is the max elapse from when a reservation has been confirmed to when it is paid.
const TMP_RSV_TIMEOUT: Duration = Duration::from_secs(5 * 60);
