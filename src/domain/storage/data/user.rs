use crate::domain::{ReservationId, UserId};
use crate::foundation::errors::rsv_conflict;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct User {
    user_id: UserId,
    reservations: HashSet<ReservationId>,
}

impl User {
    /// Links a reservation with the user.
    ///
    /// # Panic
    /// if the reservation id is already in the user's profile. It is very dangerous because there
    /// may be potential reservation-id conflicts.
    pub fn link(&mut self, id: ReservationId) {
        if !self.reservations.insert(id) {
            panic!(rsv_conflict());
        }
    }
}
