use crate::domain::ReservableItemId;
use anyhow::Result;

/// Reservable Item. Examples of such items are passenger tickets, pick-up tickets, luggage-checking
/// tickets etc.
#[typetag::serde]
pub trait ReservableItem: Sync + Send {
    fn id(&self) -> ReservableItemId;

    /// True if the item was available.
    fn occupy(&self) -> bool;

    /// # Error
    /// if trying to release an available item.
    ///
    /// It's dangerous since potential misbehavior might've occurred.
    fn release(&self) -> Result<()>;
}

/// Reservable-Item State.
pub enum State {
    Available,
    Occupied,
    Expired,
}
