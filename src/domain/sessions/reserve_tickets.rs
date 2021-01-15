use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Error, Result};
use dashmap::DashMap;

use crate::domain::payment::Payment;
use crate::domain::storage::data::reservation::{Reservation, ReservationFactoryV1};
use crate::domain::storage::data::user::User;
use crate::domain::storage::reservations::{CreativeStorage, Storage, StorageV1};
use crate::domain::storage::{items, reservations, users};
use crate::domain::{ItemToken, ReservationId, UserId, UserToken};
use crate::foundation::errors::{user_not_conformant, user_not_found};
use std::ops::Add;
use steel_cent::Money;

/// Reserve-Tickets Session.
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
    /// # Set `due` time (todo)
    /// When confirming a reservation, the due time of it is updated.
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
    /// # Expects
    /// the reservation is confirmed.
    ///
    /// # Ensures
    /// Unpaid reservations that exceed the timeout will be discarded, and will not be recorded in
    /// the users' profile. (won't do)
    ///
    /// # Returns
    /// money actually paid.
    ///
    /// # Error
    /// - if the payment failed
    /// - if the reservation is already paid.
    /// - if the reservation is not found.
    fn pay(&self, token: UserToken, p: Box<dyn Payment>) -> Result<steel_cent::Money>;
}

/// # Won't fix
/// 1. New design of the `payment` is recorded in the doc. If I had the time and energy I might
///    refactor the case.
/// 2. Implement timing facilities of pending reservations.
pub struct SessionV1 {
    pending_reservations: Box<dyn reservations::Storage>,
    active_reservations: Box<dyn reservations::CreativeStorage>,
    users: Arc<dyn users::Storage>,
    items: Arc<dyn items::Storage>,
}

impl SessionV1 {
    pub unsafe fn from_components(
        pending_reservations: Box<dyn reservations::Storage>,
        active_reservations: Box<dyn reservations::CreativeStorage>,
        users: Arc<dyn users::Storage>,
        items: Arc<dyn items::Storage>,
    ) -> Self {
        Self {
            pending_reservations,
            active_reservations,
            users,
            items,
        }
    }
}

impl Session for SessionV1 {
    fn start_reservation(&self, user_id: UserId) -> Result<ReservationId> {
        if !self.users.user_exists(&user_id) {
            return Err(user_not_found());
        }

        Ok(self.active_reservations.new_reservation(user_id))
    }

    fn add_item(&self, token: ItemToken) -> Result<()> {
        // Occupy the item first in case it is preempted by others.
        self.items.occupy(token.2)?;

        self.active_reservations.add_item(token).or_else(|e| {
            self.items.release(token.2);
            Err(e)
        })
    }

    fn remove_item(&self, token: ItemToken) -> Result<()> {
        self.active_reservations.remove_item(token)?;
        self.items.release(token.2)
    }

    fn summary(&self, token: UserToken) -> Result<String> {
        self.active_reservations.summary(token)
    }

    fn confirm(&self, token: UserToken) -> Result<()> {
        self.active_reservations
            .transfer_to(token, self.pending_reservations.borrow())
    }

    fn abort(&self, token: UserToken) -> Result<()> {
        unsafe { self.active_reservations.extract(token)? }; // drop
        Ok(())
    }

    fn pay(&self, token: UserToken, p: Box<dyn Payment>) -> Result<steel_cent::Money> {
        let ret = self
            .pending_reservations
            .process(token, Box::new(move |rsv| p.pay(rsv)))?;
        self.users.link(token);
        Ok(ret)
    }
}
