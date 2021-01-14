use crate::domain::payment::Refund;
use crate::domain::storage::data::reservation::Reservation;
use crate::domain::{ReservationId, UserId};
use crate::foundation::errors::{rsv_conflict, rsv_not_found};
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Serialize, Deserialize)]
pub struct User {
    user_id: UserId,
    undone: ReservationMap,
    withdrawn: ReservationMap,
}

type ReservationMap = HashMap<ReservationId, Reservation>;

impl User {
    /// Links a reservation with the user.
    ///
    /// # Panic
    /// if the reservation id is already in the user's profile. It is very dangerous because there
    /// may be potential reservation-id conflicts.
    pub fn link(&mut self, r: Reservation) {
        if !self.undone.insert(r.id(), r) {
            panic!(rsv_conflict());
        }
    }

    /// # Return
    /// serde report of undone reservations.
    pub fn undone_reservations_serde(&mut self) -> Vec<String> {
        self.undone
            .values()
            .map(|r| serde_json::to_string(r).expect("Serde Err for Reservation"))
            .collect()
    }

    /// Refund the reservation.
    ///
    /// Note that the user will not check if the reservation is already done. The logic is up to
    /// the passed [Refund]. In this way I could achieve higher flexibility.
    ///
    /// # Error
    /// - if the reservation is not in the user's profile.
    pub fn refund(
        &mut self,
        r_id: &ReservationId,
        method: &dyn Refund,
    ) -> Result<steel_cent::Money> {
        method.refund(
            self.undone
                .get(r_id)
                .or(self.withdrawn.get(r_id))
                .ok_or(rsv_not_found())?,
        )
    }
}
