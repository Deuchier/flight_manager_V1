use crate::domain::sessions::ItemToken;
use crate::domain::storage::data::reservation::{Reservation, ReservationFactory};
use crate::domain::{ReservationId, UserId};
use anyhow::{anyhow, Result};
use dashmap::mapref::one::RefMut;
use dashmap::DashMap;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Reservation Storage.
///
/// Different from other storages, such as `UserStorage`, which might be singletons, there may be
/// multiple instances of reservation storages present. For example, we may need temporary storage
/// for active reservations which are not confirmed.
pub trait Storage {
    fn store(&self, r: Reservation);

    /// Asks for write handles on a reservation.
    ///
    /// # May Deadlock
    /// if called when holding any other references into the storage.
    fn write(&self, r_id: ReservationId) -> Option<RefMut<ReservationId, Reservation>>;
}

/// In-memory reservation storage for storing active reservations, i.e. those that are being made
/// by the user.
///
///
/// TODO: Kill the todo in User Storage. By the principle of Information Expert, I should make the
///       reservation storage responsible for creating reservations, so the session need not know
///       the existence of the factory.
///         A storage such as this creates new reservations. The session then move the reservation
///       to other storages or the void. This is far more ideal.
///
/// See [../sessions/reserve_tickets.html] for more.
///
/// # Lifetime
/// This storage
pub struct ActiveReservations<'f> {
    // Initially, I used a `RwLock<HashMap<...>>` to implement this. Later, I found this nice
    // lib called `DashMap` which is exactly what I needed.
    reservations: DashMap<ReservationId, Reservation>,
    factory: &'f ReservationFactory,
}

const CONFLICT: &str = "Reservation Id conflicted";

impl<'f> ActiveReservations<'f> {
    /// Create a new reservation for the user. Does not check if the user id is valid.
    pub fn new_reservation(&self, user_id: UserId) -> ReservationId {
        let reservation = self.factory.with_user_id(user_id);
        let id = reservation.id();
        assert!(self.reservations.insert(id, reservation).is_none(), CONFLICT);
        id
    }
}

impl<'f> Storage for ActiveReservations<'f> {
    fn store(&self, r: Reservation) {
        assert!(self.reservations.insert(r.id(), r).is_none(), CONFLICT);
    }

    fn write(&self, r_id: &ReservationId) -> Option<RefMut<ReservationId, Reservation>> {
        self.reservations.get_mut(r_id)
    }
}
