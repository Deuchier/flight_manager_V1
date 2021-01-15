use crate::domain::payment::Refund;
use crate::domain::storage::data::reservation::Reservation;
use crate::domain::{ReservationId, UserId};
use crate::foundation::errors::{rsv_conflict, rsv_not_found};
use anyhow::Result;
use boolinator::Boolinator;
use serde::Deserialize;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Serialize, Deserialize)]
pub struct User {
    user_id: UserId,
    undone: ReservationSet,
    withdrawn: ReservationSet,
}

type ReservationSet = HashSet<ReservationId>;

impl User {
    /// Links a reservation with the user.
    ///
    /// # Panic
    /// if the reservation id is already in the user's profile. It is very dangerous because there
    /// may be potential reservation-id conflicts.
    pub fn link(&mut self, r: ReservationId) {
        assert!(self.undone.insert(r), rsv_conflict());
    }

    /// Get a list of undone reservations of the user.
    pub fn undone_reservations(&self) -> Vec<ReservationId> {
        self.undone.iter().cloned().collect()
    }

    /// Similar to [undone_reservations]
    pub fn done_reservations(&self) -> Vec<ReservationId> {
        self.withdrawn.iter().cloned().collect()
    }

    /// Put the reservation from the `undone` list to the `withdrawn` list.
    ///
    /// # Error
    /// if the reservation is not in the undone list.
    pub fn withdraw(&mut self, r_id: &ReservationId) -> Result<()> {
        self.undone.remove(r_id).ok_or(rsv_not_found())?;
        Ok(assert!(self.withdrawn.insert(r_id.clone()), rsv_conflict()))
    }
}
