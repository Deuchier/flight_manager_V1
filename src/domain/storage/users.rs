use crate::domain::storage::data::reservation::{Reservation, ReservationFactoryV1};
use crate::domain::storage::data::user::User;
use crate::domain::{ReservationId, UserId, };
use anyhow::{anyhow, Result};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use crate::foundation::errors::user_not_found;

/// User Storage
///
/// # Reservations Stored in User Storage
/// Since reservations are involved with money, and refunding is required, we need to record that
/// "who did what reservations". Reservations are no longer stored as an independent state as in
/// previous tries, since it makes little sense for a reservation to live with no owner, and we
/// can easily get around the limitation by providing a void owner.
///
/// Nonetheless, the program still has some reservation storages. But they are mainly used to store
/// temporary reservations (undone, unpaid etc).
pub trait Storage: Sync {
    fn user_exists(&self, user_id: &UserId) -> bool;

    /// Adds a reservation to the user's profile.
    ///
    /// # Panic
    /// if the user is not in the storage.
    ///
    /// When we decide to call this function, it means that we already have a reservation linked
    /// with a certain user in the storage, so, it is unlikely that the user could not be found.
    fn add_reservation(&self, r: Reservation);
}

#[derive(Serialize, Deserialize)]
pub struct StorageV1 {
    users: DashMap<UserId, User>,
    reservations: DashMap<ReservationId, Reservation>,
}

impl Storage for StorageV1 {
    fn user_exists(&self, user_id: &UserId) -> bool {
        self.users.contains_key(user_id)
    }

    fn add_reservation(&self, r: Reservation) {
        self.users
            .get_mut(r.user_id())
            .expect(&user_not_found().to_string())
            .link(r.id());

        self.reservations.insert(r.id(), r);
    }
}
