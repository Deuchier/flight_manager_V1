use crate::domain::{ReservationId, UserId};
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct User {
    user_id: UserId,
    reservations: HashSet<ReservationId>,
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

#[cfg(test)]
mod test {
    use crate::domain::storage::data::user::User;

    #[test]
    fn serde() {}

    fn get_test_user() -> User {
        unimplemented!()
    }
}
