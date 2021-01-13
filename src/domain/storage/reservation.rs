use std::borrow::{Borrow, BorrowMut};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::{anyhow, Error, Result};
use dashmap::DashMap;
use dashmap::mapref::one::RefMut;

use crate::domain::{ItemToken, ReservationId, UserId, UserToken, make_user_token};
use crate::domain::storage::data::reservation::{Reservation, ReservationFactory};
use std::process::Termination;

/// Reservation Storage.
///
/// Different from other storages, such as `UserStorage`, which might be singletons, there may be
/// multiple instances of reservation storages present. For example, we may need temporary storage
/// for active reservations which are not confirmed.
pub trait Storage {
    fn store(&self, r: Reservation);

    /// Add the item to the reservation list, if it belongs to the user. Returns error otherwise.
    fn authenticated_add(&self, tok: ItemToken) -> Result<()>;

    fn authenticated_remove(&self, tok: ItemToken) -> Result<()>;
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
        assert!(
            self.reservations.insert(id, reservation).is_none(),
            CONFLICT
        );
        id
    }
}

impl<'f> Storage for ActiveReservations<'f> {
    fn store(&self, r: Reservation) {
        assert!(self.reservations.insert(r.id(), r).is_none(), CONFLICT);
    }

    fn authenticated_add(&self, tok: ItemToken) -> Result<()> {
        let mut reservation = self.checked_rsv_mut(make_user_token(&tok));
        Ok(reservation.add(tok.2))
    }

    fn authenticated_remove(&self, tok: ItemToken) -> Result<()> {
        let mut reservation = self.checked_rsv_mut(make_user_token(&tok));
        Ok(reservation.remove(tok.2))
    }
}

impl<'f> ActiveReservations<'f> {
    // rsv == reservation
    fn checked_rsv_mut(&self, tok: UserToken) -> Result<RefMut<ReservationId, Reservation>> {
        let reservation =
            self.reservations.get_mut(tok.1).ok_or(anyhow!("Reservation not found"))?;
        if reservation.user_id() != tok.0 {
            Err(anyhow!("User id not conformant with the reservation"))
        } else {
            Ok(reservation)
        }
    }
}