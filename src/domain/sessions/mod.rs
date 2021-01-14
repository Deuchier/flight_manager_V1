//! Sessions
//!
//! Each session corresponds to a use case in the analyses.
//!
//! # Sync
//! Sessions should not be unique to only one thread. Therefore, each session should satisfy `Sync`.

use crate::domain::storage::data::flight::Address;
use crate::domain::{ReservableItemId, ReservationId, UserId};

pub mod reserve_tickets;
pub mod view;
pub mod refund;

/// Querying configuration.
pub struct Query(Address, Address);

impl Query {
    pub fn src(&self) -> &Address {
        &self.0
    }

    pub fn dest(&self) -> &Address {
        &self.1
    }
}
