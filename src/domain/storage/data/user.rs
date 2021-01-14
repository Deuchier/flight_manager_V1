use crate::domain::storage::data::reservation::Reservation;
use crate::domain::{ReservationId, UserId};
use crate::foundation::errors::rsv_conflict;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::time::Instant;

/// (todo)
/// # Lazy Sorting of Reservations
/// `User` stores the reservations with different states in different storages. However, when a
/// reservation is initially linked onto the user, the user will not sort it immediately. It will
/// do so when querying the reservations. In this way, we can easily update the user's info.
///
/// ## Unsorted Reservations
/// are those that have not expired or withdrawn.
///
/// ## Done Reservations
/// are those that's been withdrawn.
#[derive(Serialize, Deserialize)]
pub struct User {
    user_id: UserId,
    unsorted: ReservationSet,
    done: ReservationSet,
    expired: ReservationSet,
}

type ReservationSet = HashSet<Reservation>;

impl User {
    /// Links a reservation with the user.
    ///
    /// # Panic
    /// if the reservation id is already in the user's profile. It is very dangerous because there
    /// may be potential reservation-id conflicts.
    pub fn link(&mut self, r: Reservation) {
        if !self.unsorted.insert(r) {
            panic!(rsv_conflict());
        }
    }

    /// # Return
    /// serde report of undone reservations.
    pub fn undone_reservations_serde(&mut self) -> Vec<String> {
        let now = Instant::now();
        self.unsorted
            .into_iter()
            .filter_map(|r| {
                if r.due() < now {
                    assert!(self.expired.insert(r), rsv_conflict());
                    None
                } else {
                    let report = serde_json::to_string(&r).expect("Serde Err for Reservation");
                    assert!(self.unsorted.insert(r), rsv_conflict());
                    Some(report)
                }
            })
            .collect()
    }
}
