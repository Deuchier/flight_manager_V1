use crate::domain::storage::users;
use crate::domain::UserId;
use anyhow::Result;

/// Refund Session
pub trait Session {
    /// Returns serialized representation of refundable reservations.
    fn refundable_reservations(&self, user_id: &UserId) -> Result<Vec<String>>; //TODO

    fn start_refund(&self);
}

pub struct SessionV1<'a> {
    users: &'a dyn users::Storage,
}

impl<'a> Session for SessionV1<'a> {
    fn refundable_reservations(&self, user_id: &UserId) -> Result<Vec<String>> {
        self.users.refundable_reservations_serde(user_id)
    }

    fn start_refund(&self) {
        unimplemented!()
    }
}
