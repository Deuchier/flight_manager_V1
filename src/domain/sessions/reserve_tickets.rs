use anyhow::{Context, Result};

use crate::domain::sessions::{ItemToken, UserToken};
use crate::domain::storage::data::reservation::{Reservation, ReservationFactory};
use crate::domain::storage::data::user::User;
use crate::domain::storage::users;
use crate::domain::{ReservationId, UserId};
use std::collections::HashMap;
use std::sync::{Mutex, RwLock, RwLockWriteGuard};
use std::time::{Duration, Instant};

/// Reserve-Tickets Session.
pub trait Session {
    /// Start a new reservation for the user. Returns a unique id for identifying the reservation.
    ///
    /// The function receives the user's id as a `String` and stores it in the reservation.
    ///
    /// # Temporary Reservation
    /// This call starts a temporary reservation. Until confirmed, the reservation may not be
    /// stored persistently.
    ///
    /// # Error
    /// if the user is not found.
    fn start_reservation(&self, user_id: String) -> Result<ReservationId>;

    /// Adds a reservable item to the reservation list.
    ///
    /// Will exclusively occupy the item.
    ///
    /// # Error
    /// - if any of the user, the reservation, or the item is not found;
    /// - if the reservation is already confirmed or not valid (e.g. aborted).
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
/// ReserveTicketsSessions live no longer than either ('a) the User Storage it refers to, or ('b)
/// the factory that produces reservations.
pub struct SessionV1<'a, 'b> {
    // Outer lock for inserting new elements into the vec.
    // Inner lock for visiting a certain reservation.
    active_reservations: RwLock<HashMap<ReservationId, RwLock<Reservation>>>,
    // Might not be too many temps, so a Vec will do.
    // Confirmed reservations waiting to be paid.
    // We need only one modification to them when they are popped from the Vec, so no need to
    // add locks.
    pending_reservations: Vec<TempReservation>,
    users: &'a dyn users::Storage,
    factory: &'b ReservationFactory,
}

struct TempReservation {
    deadline: Instant,
    inner: Reservation,
}

/// Timeout for a new reservation.
///
/// This is the max elapse from when a reservation has been confirmed to when it is paid.
const TEMP_RESERVATION_TIMEOUT: Duration = Duration::from_secs(5 * 60);

impl<'a, 'b> Session for SessionV1<'a, 'b> {
    fn start_reservation(&self, user_id: UserId) -> Result<ReservationId> {
        if !self.users.user_exists(&user_id) {
            return Err(anyhow!("User not found"));
        }

        let reservation = self.factory.with_user_id(user_id);
        let reservation_id = reservation.id();

        self.store_active(reservation);

        Ok(reservation_id)
    }

    fn add(&self, token: ItemToken) -> Result<()> {
        unimplemented!()
    }

    fn remove(&self, token: ItemToken) -> Result<()> {
        unimplemented!()
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

impl<'a, 'b> SessionV1<'a, 'b> {
    fn store_active(&self, r: Reservation) {
        assert!(
            self.active_reservations
                .write()
                .expect("Reserve Tickets Session thread poisoned")
                .insert(r.id(), RwLock::new(r))
                .is_none(),
            "Reservation Id conflicted"
        );
    }

    /// Check if the provided token conforms with the internal records. If so, returns a write
    /// guard to the reservation.
    fn checked_write(&self, tok: UserToken) -> Result<RwLockWriteGuard<Reservation>> {
        let ret = self
            .active_reservations
            .read()
            .expect("Reserve Tickets Session thread poisoned")
            .get(tok.1)
            .ok_or(anyhow!("Reservation not active"))?
            .write()
            .expect("Reserve Tickets Session thread poisoned");
        if ret.user_id() != tok.0 {
            Err(anyhow!("User id not conformant with the reservation"))
        } else {
            Ok(ret)
        }
    }
}
