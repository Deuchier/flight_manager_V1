use crate::domain::{UserId, ReservationId, ReservableItemId};
use std::collections::{HashSet};

/// User Storage
///
/// Stores user information.
///
/// # What are in "User Information"
/// Since reservations are involved with money, and refunding is required, we need to record that
/// "who did what reservations". Reservations are no longer stored as an independent state as in
/// previous tries, since it makes little sense for a reservation to live with no owner, and we
/// can easily get around the limitation by providing a void owner.
pub trait Storage {
    /// Returns true if the user exists in the storage.
    ///
    /// This function can be used to check if the user provided a correct user id.
    fn user_exists(&self, user_id: &UserId) -> bool;

    /// Starts a new reservation for the user.
    ///
    /// # Internal mutability
    /// The function receives `&self` instead of mutable references. It is because that when
    /// creating new reservations, it is okay for other threads reading user data to report that
    /// the user has not created a new reservation. It is actually the case. The reservation will
    /// only be added to the list of a reservation until
    ///
    /// # No Error
    /// This function has no reason to fail. However, if with any prospect there will be errors in
    /// the future, it can return `ReservationId::max()`.
    fn start_reservation(&self, user_id: &Userid) -> ReservationId;
}

/// Get an instance of a user storage. Hides the concrete type.
pub fn storage_instance() -> &'static dyn Storage {
    unimplemented!()
}

/// Internal representation of a reservation.
///
/// Does not own any reservable items, but contain ids of them. We need to ask the Reservable-Item
/// Storages for their existence.
struct Reservation {
    id: ReservationId,
    user: UserId,
    items: HashSet<ReservableItemId>
}