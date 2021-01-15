//! Storage.
//!
//! - Provides interface for storages.
//! - Provides concurrent accessibility.
//! - Some are persistent, others are temporary
//! - Does NOT guarantee any check on links. E.g. it is up to the session controllers to ensure that
//! the `Users` don't store `ReservationId`s of which reservation that do not belong to them.

use crate::domain::storage::data::reservation::Reservation;
use crate::domain::ReservationId;
use dashmap::DashMap;

pub mod data;
pub mod flights;
pub mod items;
pub mod reservations;
pub mod users;

pub(crate) type RsvMap = DashMap<ReservationId, Reservation>;
