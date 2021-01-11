use crate::domain::{UserId, ReservationId};
use std::collections::{HashSet};
use crate::domain::storage::data::reservation::Reservation;
use anyhow::Result;

pub struct User {
    user_id: UserId,
    reservations: HashSet<ReservationId>
}

impl User {
    /// Links an item with the user.
    ///
    /// # Error
    /// If the id is already in the user's profile, returns `Err`.
    ///
    /// It is very dangerous when this function returns `Err`. There may be potential reservation-id
    /// conflicts then.
    pub fn link(&mut self, id: ReservationId) -> Result<()> {
       if self.reservations.insert(id) {
           Ok(())
       } else {
           Err(anyhow::anyhow!("Reservation already in the list"))
       }
    }
}