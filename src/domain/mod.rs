//! Domain Layer.
//!
//! The Domain Layer contains the logic of the program. It implements classes that correspond to the
//! domain model.
//!
//! This file also contains some type definitions (shorthands) used commonly by the sub-modules.
pub mod payment;
pub mod sessions;
pub mod storage;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

pub type UserId = String; // UserId is the internal id of a user. They can also have nicknames.
pub type ReservationId = u64; // This id must be `Copy`.
pub type FlightId = String;

#[derive(Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct ReservableItemId {
    flight_id: FlightId,
    inner_id: String,
}

impl ReservableItemId {
    pub fn independent(inner: String) -> Self {
        Self {
            flight_id: "unknown".to_string(),
            inner_id: inner,
        }
    }

    pub fn flight_id(&self) -> &FlightId {
        &self.flight_id
    }
}

/// Helper types for simplifying the signature of the sessions.
///
/// The two tokens are `Copy`.
pub type UserToken<'a> = (&'a UserId, &'a ReservationId);
pub type ItemToken<'a> = (&'a UserId, &'a ReservationId, &'a ReservableItemId);

fn make_user_token<'a>(tok: &'a ItemToken) -> UserToken<'a> {
    (tok.0, tok.1)
}
