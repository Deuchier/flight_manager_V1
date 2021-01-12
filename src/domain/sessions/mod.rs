//! Sessions
//!
//! Each session corresponds to a use case in the analyses.
//!
//! # Sync
//! Sessions should not be unique to only one thread. Therefore, each session should satisfy `Sync`.

use crate::domain::{ReservableItemId, ReservationId, UserId};

pub mod reserve_tickets;

/// Helper types for simplifying the signature of the sessions.
pub type UserToken<'a> = (&'a UserId, &'a ReservationId);
pub type ItemToken<'a> = (&'a UserId, &'a ReservationId, &'a ReservableItemId);

fn make_user_token(tok: &ItemToken) -> UserToken {
    UserToken { 0: tok.0, 1: tok.1 }
}
