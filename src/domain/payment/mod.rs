use crate::domain::storage::data::reservation::Reservation;
use anyhow::Result;

/// Interface for payments. Implementors can adopt different strategies.
///
/// TODO: implement payment
pub trait Payment {
    fn pay(&self, r: &Reservation) -> Result<steel_cent::Money>;
}
