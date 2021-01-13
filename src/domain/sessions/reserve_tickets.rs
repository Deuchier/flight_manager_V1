use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Error, Result};
use dashmap::DashMap;

use crate::domain::{ItemToken, ReservationId, UserId, UserToken};
use crate::domain::storage::{items, users};
use crate::domain::storage::data::reservation::{Reservation, ReservationFactory};
use crate::domain::storage::data::user::User;
use crate::domain::storage::reservation::{ActiveReservations, Storage};

/// Reserve-Tickets Session.
pub trait Session {
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
    fn add(&self, token: ItemToken) -> Result<()>;

    /// Removes an item from the list.
    ///
    /// Similar to `add`.
    fn remove(&self, token: ItemToken) -> Result<()>;

    /// Gets a summary of the current state of the reservation.
    ///
    /// # Return
    /// a serialized form of the reservation.
    ///
    /// > <del>
    /// > AFA (Jun. 11 '20), I'm using the YAML form instead of the more famous JSON, since
    /// > YAML is more concise and readable than JSON.
    /// > </del>
    /// >
    /// > I've switched back to JSON since it has the best library support.
    fn summary(&self, token: ItemToken) -> Result<String>;

    /// Confirms an reservation.
    ///
    /// Calling this function will terminate modifying process of the reservation (i.e. `add` or
    /// `remove`). Any subsequent calls to the functions or this function will result in an error.
    ///
    /// # Persistent Storage
    /// Calling this function will cause the reservation to be persistently stored into the user's
    /// profile. This function must be called before ending a reservation, unless errors occur, or
    /// the user wish to abort the reservation.
    fn confirm(&self, token: UserToken) -> Result<()>;

    /// Aborts a reservation that's not paid yet.
    ///
    /// The aborted reservation will not be stored in the system.
    ///
    /// # Error
    /// If the reservation has already been paid, then the function will return `Err`. You should
    /// use the `refund` functionality instead of `abort`.
    fn abort(&self, token: UserToken) -> Result<()>;

    /// Pays for a reservaiton.
    ///
    /// # Error
    /// Returns `Err` if the payment failed, or the reservation is already paid.
    fn pay(&self, token: UserToken) -> Result<()>;
}

/// # Lifetime
/// `ReserveTicketsSession`s live no longer than
/// - ('a) the active-reservation storage;
/// - ('b) the User Storage;
/// - ('c) the Item Storage.
pub struct SessionV1<'a, 'b, 'c> {
    // Might not be too many temps, so a Vec will do.
    // Confirmed reservations waiting to be paid.
    // We need only one modification to them when they are popped from the Vec, so no need to
    // add locks.
    pending_reservations: Vec<TempReservation>,
    active_reservations: ActiveReservations<'a>,
    users: &'b dyn users::Storage,
    items: &'c dyn items::Storage,
}

impl<'a, 'b, 'c> Session for SessionV1<'a, 'b, 'c> {
    fn start_reservation(&self, user_id: UserId) -> Result<ReservationId> {
        if !self.users.user_exists(&user_id) {
            return Err(anyhow!("User not found"));
        }

        Ok(self.active_reservations.new_reservation(user_id))
    }

    fn add(&self, token: ItemToken) -> Result<()> {
        // Occupy the item first in case it is preempted by others.
        self.items.occupy(token.2)?;

        self.active_reservations.authenticated_add(token)
            .or_else(|e| {
                self.items.release(token.2);
                Err(e)
            })
    }

    fn remove(&self, token: ItemToken) -> Result<()> {
        self.active_reservations.authenticated_remove(token)?;
        Ok(self.items.release(token.2)) // Ok for the tuple is `Copy`
    }

    fn summary(&self, token: ItemToken) -> Result<String> {
        unimplemented!()
    }

    fn confirm(&self, token: UserToken) -> Result<()> {
        unimplemented!()
    }

    fn abort(&self, token: UserToken) -> Result<()> {
        unimplemented!()
    }

    fn pay(&self, token: UserToken) -> Result<()> {
        unimplemented!()
    }
}

struct TempReservation {
    deadline: Instant,
    inner: Reservation,
}

/// Timeout for a new reservation.
///
/// This is the max elapse from when a reservation has been confirmed to when it is paid.
const TEMP_RESERVATION_TIMEOUT: Duration = Duration::from_secs(5 * 60);
