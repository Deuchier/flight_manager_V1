/// Item Storage
///
/// Contains definition of the [items::Storage] trait. Also provides a low-level implementation of
/// it. Note that [flights::StorageV1] implements this trait too.
use crate::domain::storage::data::item;
use crate::domain::storage::data::item::ReservableItem;
use crate::domain::storage::data::item::State::{Available, Occupied};
use crate::domain::ReservableItemId;
use crate::foundation::errors::{item_not_available, item_not_found};
use anyhow::{anyhow, Result};
use boolinator::Boolinator;
use dashmap::{mapref, DashMap};
use serde::{Deserialize, Serialize};
use crate::domain::storage::flights;

/// Reservable-Item Storage.
///
/// It is up to the caller to ensure that external references into this storage are correctly
/// handled.
///
/// # Serde
/// The storage may need to be persistently stored on disk.
pub trait Storage: Sync + Serialize + Deserialize {
    /// Atomically occupies an item.
    ///
    /// # Error
    /// - if the item was not `Available`. This is common when many users are contending.
    /// - if the item is not found.
    fn occupy(&self, item_id: &ReservableItemId) -> Result<()>;

    /// Releases an item.
    ///
    /// No need for atomicity, since it does not matter if another user found that the item was
    /// occupied when it was being released.
    ///
    /// # Error
    /// - if the item is not found.
    ///
    /// # Panic
    /// if the item to be released is not occupied. In this case, the item has been wrongly released
    /// by someone else, which should never happen!
    fn release(&self, item_id: &ReservableItemId) -> Result<()>;
}


/// Leaf storage in the composite pattern.
#[derive(Serialize, Deserialize)]
pub struct SimpleStorage {
    items: DashMap<ReservableItemId, Box<dyn ReservableItem>>,
}

impl Storage for SimpleStorage {
    fn occupy(&self, id: &ReservableItemId) -> Result<()> {
        self.item(id)?.occupy().ok_or(item_not_available())
    }

    fn release(&self, id: &ReservableItemId) -> Result<()> {
        Ok(self.item(id)?.release().expect("Ownership spoiled"))
    }
}

/// Reference to the map's internal tuple
type Ref<'a> = mapref::one::Ref<'a, ReservableItemId, Box<dyn ReservableItem>>;

impl SimpleStorage {
    fn item(&self, id: &ReservableItemId) -> Result<Ref> {
        self.items.get(id).ok_or(item_not_found())
    }
}
