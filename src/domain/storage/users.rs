use crate::domain::payment::Refund;
use crate::domain::storage::data::reservation::{Reservation, ReservationFactoryV1};
use crate::domain::storage::data::user::User;
use crate::domain::{ReservationId, UserId, UserToken};
use crate::foundation::errors::{user_not_conformant, user_not_found};
use anyhow::{anyhow, Result};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

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
///
/// > My brain must be stuck with shit when I wrote the previous paragraphs. How on earth could I
/// > ended up designing a user storage that is so coupled with reservations?
/// >
/// > Someone help kill the me of yesterday!
pub trait Storage: Sync + Send {
    fn user_exists(&self, user_id: &UserId) -> bool;

    /// Links a reservation to the user's profile.
    ///
    /// # Panic
    /// if the user is not in the storage.
    ///
    /// When we decide to call this function, it means that we already have a reservation linked
    /// with a certain user in the storage, and we only have to inform the storage of it now. So, it
    /// is unlikely that the user could not be found.
    fn link(&self, token: UserToken);

    /// Generate a list of reservation related with the user.
    fn undone_reservations(&self, user_id: &UserId) -> Result<Vec<ReservationId>>;

    /// Similar to [undone_reservations]
    fn done_reservations(&self, user_id: &UserId) -> Result<Vec<ReservationId>>;

    fn withdraw(&self, token: UserToken) -> Result<()>;
}

#[derive(Serialize, Deserialize)]
pub struct StorageV1 {
    users: DashMap<UserId, User>,
}

impl Storage for StorageV1 {
    fn user_exists(&self, user_id: &UserId) -> bool {
        self.users.contains_key(user_id)
    }

    fn link(&self, token: UserToken) {
        self.users
            .get_mut(token.0)
            .expect("User should be in the storage")
            .link(token.1.clone());
    }

    fn undone_reservations(&self, user_id: &UserId) -> Result<Vec<ReservationId>> {
        Ok(self
            .users
            .get(user_id)
            .ok_or(user_not_found())?
            .undone_reservations())
    }

    fn done_reservations(&self, user_id: &UserId) -> Result<Vec<ReservationId>> {
        Ok(self
            .users
            .get(user_id)
            .ok_or(user_not_found())?
            .done_reservations())
    }

    fn withdraw(&self, token: UserToken) -> Result<()> {
        self.users
            .get_mut(token.0)
            .ok_or(user_not_found())?
            .withdraw(token.1)
    }
}
