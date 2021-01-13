use crate::domain::{ReservableItemId, ReservationId, UserId};
use anyhow::{anyhow, Result};
use boolinator::Boolinator;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};

/// Internal representation of a reservation.
///
/// Does not own any reservable items, but contain ids of them. We need to ask the Reservable-Item
/// Storages for their existence.
#[derive(Serialize, Deserialize)]
pub struct Reservation {
    id: ReservationId,
    user: UserId,
    items: HashSet<ReservableItemId>,
}

impl Reservation {
    pub fn id(&self) -> ReservationId {
        self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user
    }

    /// Add an item to the reservation list.
    ///
    /// # Error
    /// if an item with the same id is already in the list, since items should have unique ids.
    pub fn add(&mut self, item: ReservableItemId) -> Result<()> {
        self.items
            .insert(item)
            .ok_or(anyhow!("Reservable Item Id conflicted"))
    }

    pub fn remove(&mut self, item: &ReservableItemId) -> Result<()> {
        self.items
            .remove(item)
            .ok_or(anyhow!("Item not in the list"))
    }

    /// Generate a summary of the reservation.
    ///
    /// No error. Why would the function ever go wrong?
    pub fn summary(&self) -> String {
        serde_json::to_string(self).expect("Error when serializing the reservation")
    }
}

/// Trait representing a reservation factory.
///
/// # Initialization and Sync
/// This trait must be implemented with care. Reservations should have unique ids. Multiple
/// instances of factories should produce their products with conforming ids. Singletons could be
/// preferred, which should be taken care of in the initialization.
pub trait ReservationFactory {
    /// Creates a new reservation with the given user id.
    ///
    /// Each reservation should have a unique id. They get the id atomically.
    fn with_user_id(&self, user_id: UserId) -> Reservation;
}

/// Singleton factory building reservations. It stores an internal state of the next id, which will
/// be stored persistently so that even after restarts the ids are still consistent.
///
/// # Serde
/// Serde required because it needs to be restored after restart of the program.
#[derive(Serialize, Deserialize)]
pub struct ReservationFactoryV1 {
    // TODO: finish initialization.
    next_id: AtomicU64,
}

impl ReservationFactory for ReservationFactoryV1 {
    fn with_user_id(&self, user_id: UserId) -> Reservation {
        Reservation {
            // Relaxed because nobody else uses the atomic value.
            id: self.next_id.fetch_add(1, Ordering::Relaxed),
            user: user_id,
            items: Default::default(),
        }
    }
}

impl ReservationFactoryV1 {
    /// Creates a new reservation factory
    ///
    /// # Unsafe
    /// Normally the factory should be restored from persistent storage. Call this function only if
    /// no persistent storage is found, in which case an alert should be given.
    pub(crate) unsafe fn new(init_id: ReservationId) -> Self {
        Self {
            next_id: AtomicU64::new(init_id),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::domain::storage::data::reservation::{Reservation, ReservationFactoryV1};
    use crate::foundation::file_writer::SimpleWriter;
    use std::collections::HashSet;
    use std::io::Write;
    use std::iter::FromIterator;
    use crate::domain::ReservableItemId;

    #[test]
    fn serde() {
        let reservation = test_reservation();
        let factory = unsafe { ReservationFactoryV1::new(24) };
        let mut writer = SimpleWriter::new("./tmp/test_reservation.json");
        serde_json::to_writer(writer.writer(), &reservation).unwrap();
        // emm, json only allows one top-level object.
        serde_json::to_writer(writer.writer(), &factory).unwrap();
        writer.writer().flush();
    }

    fn test_reservation() -> Reservation {
        Reservation {
            id: 114514,
            user: "TestUser".to_string(),
            items: HashSet::from_iter(
                vec![
                    "TestItemId1",
                    "TestItemApple",
                    "TestItemBillGates",
                    "Test item id with spaces",
                ]
                .into_iter()
                .map(|s| ReservableItemId::independent(s.to_string())),
            ),
        }
    }
}
