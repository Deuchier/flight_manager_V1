use crate::domain::ReservableItemId;
use anyhow::{anyhow, Result};

/// Reservable-Item Storage.
///
/// This storage records the states of items. If an item is to be occupied by a reservation, the
/// session controller must call corresponding methods on the storage to update the item's state,
/// and *in careful sequence*.
pub trait Storage {
    /// Atomically occupies an item.
    ///
    /// # Error
    /// - if the item was not `Available`.
    /// - if the item is not found.
    fn occupy(&self, item_id: ReservableItemId) -> Result<()>;

    /// Releases an item.
    ///
    /// No need for atomicity, since it does not matter if another user found that the item was
    /// occupied when it was being released.
    ///
    /// Others similar to [occupy].
    fn release(&self, item_id: ReservableItemId) -> Result<()>;
}

#[derive(Serialize, Deserialize)]
pub struct SimpleStorage {}
