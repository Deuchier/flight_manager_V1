use crate::domain::payment::Refund;
use crate::domain::storage::data::reservation::Reservation;
use crate::domain::{ReservationId, UserId};
use crate::foundation::errors::{rsv_conflict, rsv_not_found};
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use boolinator::Boolinator;

#[derive(Serialize, Deserialize)]
pub struct User {
    user_id: UserId,
    undone: ReservationMap,
    withdrawn: ReservationMap,
}

type ReservationMap = HashSet<ReservationId>;

impl User {
    /// Links a reservation with the user.
    ///
    /// # Panic
    /// if the reservation id is already in the user's profile. It is very dangerous because there
    /// may be potential reservation-id conflicts.
    pub fn link(&mut self, r: ReservationId) {
        assert!(self.undone.insert(r), rsv_conflict());

    }

    /// # Return
    /// serde report of undone reservations.
    pub fn undone_reservations(&mut self) -> Vec<ReservationId> {
        self.undone.iter().collect()
    }

    /// # Error
    /// if the reservation is not in the undone list.
    pub fn withdrawn(&mut self, r_id: &ReservationId) -> Result<()> {
        self.undone.remove(r_id).ok_or(rsv_not_found())?;
        Ok(assert!(self.withdrawn.insert(r_id.clone()), rsv_conflict()))
    }
}
