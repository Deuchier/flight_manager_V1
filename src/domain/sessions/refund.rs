use crate::domain::UserId;

/// Refund Session
pub trait Session {
    fn start_refund(&self, user_id: &UserId) -> TransactionId; //TODO

    fn end_refund(&self, )
}