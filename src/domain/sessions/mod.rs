//! Sessions
//!
//! Each session corresponds to a use case in the analyses

use crate::domain::{ReservableItemId, ReservationId, UserId};

pub mod reserve_tickets;

/// Helper types for simplifying the signature of the sessions.
pub type UserToken<'a> = (&'a UserId, &'a ReservationId);
pub type ItemToken<'a> = (&'a UserId, &'a ReservationId, &'a ReservableItemId);
