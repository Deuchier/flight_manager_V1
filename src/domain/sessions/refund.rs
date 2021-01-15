use crate::domain::payment::Refund;
use crate::domain::storage::{users, reservations};
use crate::domain::{ReservationId, UserId, UserToken};
use anyhow::Result;
use std::sync::Arc;

/// Refund Session
pub trait Session {
    /// Returns serialized representation of refundable reservations.
    ///
    /// It's a vec so you can easily count the number.
    fn refundable_reservations(&self, user_id: &UserId) -> Result<Vec<String>>; //TODO

    fn refund(&self, tok: UserToken, method: &dyn Refund) -> Result<steel_cent::Money>;
}

pub struct SessionV1 {
    users: Arc<dyn users::Storage>,
    reservations: Arc<dyn reservations::Storage> // todo: user do not hold reservations.
}

impl SessionV1 {
    pub unsafe fn from_components(users: Arc<dyn users::Storage>) -> Self {
        Self { users }
    }
}

impl Session for SessionV1 {
    fn refundable_reservations(&self, user_id: &UserId) -> Result<Vec<String>> {
        self.users.refundable_reservations_serde(user_id)
    }

    fn refund(
        &self,
        user_id: &UserId,
        r_id: &ReservationId,
        method: &dyn Refund,
    ) -> Result<steel_cent::Money> {
        self.users.refund(user_id, r_id, method)
    }
}
