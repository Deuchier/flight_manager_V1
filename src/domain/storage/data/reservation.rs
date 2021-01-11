use crate::domain::{ReservationId, UserId, ReservableItemId};
use std::collections::HashSet;

/// Internal representation of a reservation.
///
/// Does not own any reservable items, but contain ids of them. We need to ask the Reservable-Item
/// Storages for their existence.
///
/// TODO: serde
pub(crate) struct Reservation {
    id: ReservationId,
    user: UserId,
    items: HashSet<ReservableItemId>
}

impl Reservation {
    pub fn with_user_id(user: UserId) -> Self {
        Self {
            id: self::next_id(),
            user,
            items: Default::default()
        }
    }

    /// Atomically get the next reservation id.
    ///
    /// Each reservation should have a unique id. They get the id from this function.
    ///
    /// # Persistent storage
    /// After a restart, the function should still be able to retrieve the previous state of the
    /// id pool. It lazy-initializes that value from [init].
    fn next_id() -> ReservationId {
        // TODO: finish initialization.
        unsafe {
            let ret = NEXT_ID.clone();
            NEXT_ID += 1;
            ret
        }
    }
}


static mut NEXT_ID: ReservationId = Default::default();

/// Called when initializing. If not properlly set, the id will start from 0.
pub(in crate::foundation) fn init_id_pool(next_id: ReservationId) {
    // We don't use multi-thread when initializing, right?
    unsafe { NEXT_ID = next_id };
}