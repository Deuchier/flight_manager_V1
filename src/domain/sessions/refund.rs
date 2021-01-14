use crate::domain::payment::Refund;
use crate::domain::storage::users;
use crate::domain::{ReservationId, UserId};
use anyhow::Result;

/// Refund Session
pub trait Session {
    /// Returns serialized representation of refundable reservations.
    fn refundable_reservations(&self, user_id: &UserId) -> Result<Vec<String>>; //TODO

    fn refund(
        &self,
        user_id: &UserId,
        reservation_id: &ReservationId,
        method: &dyn Refund,
    ) -> Result<steel_cent::Money>;
}

pub struct SessionV1<'a> {
    users: &'a dyn users::Storage,
}

impl<'a> Session for SessionV1<'a> {
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
