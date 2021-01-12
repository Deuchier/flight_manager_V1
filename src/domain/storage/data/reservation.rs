use crate::domain::{ReservableItemId, ReservationId, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};

/// Internal representation of a reservation.
///
/// Does not own any reservable items, but contain ids of them. We need to ask the Reservable-Item
/// Storages for their existence.
///
/// TODO: serde
#[derive(Serialize, Deserialize)]
pub struct Reservation {
    id: ReservationId,
    user: UserId,
    items: HashSet<ReservableItemId>,
}

/// Factory building reservations. It stores an internal state of the next id, which will be
/// stored persistently so that even after restarts the ids are still consistent.
pub struct ReservationFactory {
    // TODO: finish initialization.
    next_id: AtomicU64
}

impl ReservationFactory {
    /// Atomically get the next reservation id.
    ///
    /// Each reservation should have a unique id. They get the id from this function.
    ///
    /// # Persistent storage
    /// After a restart, the function should still be able to retrieve the previous state of the
    /// id pool. It lazy-initializes that value from [init].
    pub fn with_user_id(&mut self, user_id: UserId) -> Reservation {
        Reservation {
            id: self.next_id.fetch_add(1, Ordering::Relaxed),
            user: user_id,
            items: Default::default()
        }
    }
}


#[cfg(test)]
mod test {
    use crate::domain::storage::data::reservation::Reservation;
    use crate::foundation::file_writer::SimpleWriter;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use std::io::Write;

    #[test]
    fn serde() {
        let reservation = test_reservation();
        let mut writer = SimpleWriter::new("./tmp/test_reservation.json");
        serde_json::to_writer(writer.writer(), &reservation).unwrap();
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
                    .map(|s| s.to_string()),
            ),
        }
    }
}
