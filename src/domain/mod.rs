//! Domain Layer.
//!
//! The Domain Layer contains the logic of the program. It implements classes that correspond to the
//! domain model.
//!
//! This file also contains some type definitions (shorthands) used commonly by the sub-modules.
pub mod sessions;
mod storage;

pub type UserId = String; // UserId is the internal id of a user. They can also have nicknames.
pub type ReservationId = u64; // This id must be `Copy`.
pub type ReservableItemId = String; // Is a String for it may be defined by external organizations.

/// Helper types for simplifying the signature of the sessions.
///
/// The two tokens are `Copy`.
pub type UserToken<'a> = (&'a UserId, &'a ReservationId);
pub type ItemToken<'a> = (&'a UserId, &'a ReservationId, &'a ReservableItemId);

fn make_user_token<'a>(tok: &'a ItemToken) -> UserToken<'a> {
    (tok.0, tok.1)
}
