//! Storage.
//!
//! - Provides interface for storages.
//! - Provides concurrent accessibility.
//! - Some are persistent, others are temporary
//! - Does NOT guarantee any check on links. E.g. it is up to the session controllers to ensure that
//! the `Users` don't store `ReservationId`s of which reservation that do not belong to them.

pub mod data;
pub mod flights;
pub mod items;
pub mod reservation;
pub mod users;
