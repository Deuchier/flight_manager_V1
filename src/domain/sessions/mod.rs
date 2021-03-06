//! Sessions
//!
//! Each session corresponds to a use case in the analyses.
//!
//! # Sync
//! Sessions should not be unique to only one thread. Therefore, each session should satisfy `Sync`.

use crate::domain::storage::data::flight::Address;
use crate::domain::{ReservableItemId, ReservationId, UserId};

pub mod refund;
pub mod reserve_tickets;
pub mod view;
