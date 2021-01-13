use crate::domain::storage::data::item;
use crate::domain::storage::data::item::ReservableItem;
use crate::domain::storage::data::item::State::{Available, Occupied};
use crate::domain::ReservableItemId;
use anyhow::{anyhow, Result};
use dashmap::mapref::one::RefMut;
use dashmap::DashMap;
use std::intrinsics::unlikely;

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
    fn occupy(&self, item_id: &ReservableItemId) -> Result<()>;

    /// Releases an item.
    ///
    /// No need for atomicity, since it does not matter if another user found that the item was
    /// occupied when it was being released.
    ///
    /// # Panic
    /// if the item to be released is not occupied. In this case, the item has been wrongly released
    /// by someone else, which should never happen!
    fn release(&self, item_id: &ReservableItemId);
}

#[derive(Serialize, Deserialize)]
pub struct SimpleStorage {
    items: DashMap<ReservableItemId, Box<dyn ReservableItem>>,
}

impl Storage for SimpleStorage {
    fn occupy(&self, item_id: &ReservableItemId) -> Result<()> {
        let (mut state, _) = self.state_mut(item_id)?;
        if *state != Available {
            return Err(anyhow!("Item not available"));
        }
        Ok(*state = Occupied)
    }

    fn release(&self, item_id: &ReservableItemId) {
        let (mut state, _) = self.state_mut(item_id)?;
        if unsafe { unlikely(*state != Occupied) } {
            panic!("WTF the item released by someone else?");
        }
        *state = Available;
    }
}

impl SimpleStorage {
    fn state_mut(
        &self,
        item_id: &ReservableItemId,
    ) -> Result<(
        &mut item::State,
        RefMut<ReservableItemId, Box<dyn ReservableItem>>,
    )> {
        let mut guard = self
            .items
            .get_mut(item_id)
            .ok_or(anyhow!("Item not found"))?;
        let state = guard.state_mut();
        Ok((state, guard))
    }
}
