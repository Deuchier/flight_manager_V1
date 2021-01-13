use crate::domain::storage::data::reservation::{Reservation, ReservationFactory};
use crate::domain::storage::data::user::User;
use crate::domain::{ReservationId, UserId, RSV_CONFLICT, USER_NOT_FOUND};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

/// User Storage
///
/// Stores user information.
///
/// # What are in "User Information"
/// Since reservations are involved with money, and refunding is required, we need to record that
/// "who did what reservations". Reservations are no longer stored as an independent state as in
/// previous tries, since it makes little sense for a reservation to live with no owner, and we
/// can easily get around the limitation by providing a void owner.
pub trait Storage: Sync {
    /// Returns true if the user exists in the storage.
    ///
    /// This function can be used to check if the user provided a correct user id.
    fn user_exists(&self, user_id: &UserId) -> bool;

    /// Starts a new reservation for the user.
    ///
    /// # Temporary Reservation
    /// See [start_reservation](crate::domain::sessions::reserve_tickets::Session::start_reservation).
    ///
    /// # No Error
    /// This function has no reason to fail. However, if with any prospect there will be errors in
    /// the future, it can return `ReservationId::max()`.
    ///
    /// TODO: delete this function. Starting a reservation is not the responsibility of a storage.
    fn start_reservation(&self, user_id: UserId) -> Result<ReservationId>;
}

/// # Lifetime
/// The struct dies before the reservation factory is dropped.
#[derive(Serialize, Deserialize)]
pub struct StorageV1 {
    users: RwLock<HashMap<UserId, User>>,
    reservations: RwLock<HashMap<ReservationId, Reservation>>,
    factory: ReservationFactory,
}

impl Storage for StorageV1 {
    fn user_exists(&self, user_id: &UserId) -> bool {
        self.users
            .read()
            .expect("Storage thread poisoned")
            .contains_key(user_id)
    }

    fn start_reservation(&self, user_id: UserId) -> Result<ReservationId> {
        if !self.user_exists(&user_id) {
            return Err(USER_NOT_FOUND);
        }

        let new = self.factory.with_user_id(user_id);
        let id = new.id();
        self.users
            .write()
            .expect("Storage thread poisoned")
            .get_mut(new.user_id())
            .expect("I should've checked that the user is in the list??")
            .link(id)
            .expect(RSV_CONFLICT.to_string().as_str());

        assert!(
            self.reservations
                .write()
                .expect("Storage thread poisoned")
                .insert(id, new)
                .is_none(),
            "Reservation Id should be unique"
        );

        Ok(id)
    }
}
