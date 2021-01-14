use crate::domain::storage::data::reservation::Reservation;
use anyhow::Result;

/// Interface for payments. Implementors can adopt different strategies.
///
/// TODO: implement payment
pub trait Payment {
    /// # Returns
    /// actual amount of money paid
    fn pay(&self, r: &Reservation) -> Result<steel_cent::Money>;
}

pub trait Refund {
    /// # Returns
    /// actual amount of money refunded.
    fn refund(&self, r: &Reservation) -> Result<steel_cent::Money>;
}
