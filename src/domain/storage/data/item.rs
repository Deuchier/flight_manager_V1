use crate::domain::ReservableItemId;

/// Reservable Item. Examples of such items are passenger tickets, pick-up tickets, luggage-checking
/// tickets etc.
///
/// Such items are not `Send` nor `Sync`.
#[typetag::serde]
pub trait ReservableItem {
    fn id(&self) -> ReservableItemId;

    fn state(&self) -> State;

    fn state_mut(&mut self) -> &mut State;
}

/// Reservable-Item State.
pub enum State {
    Available,
    Occupied,
    Expired,
}
