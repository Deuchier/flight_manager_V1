use std::borrow::{Borrow, BorrowMut};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::{anyhow, Error, Result};
use dashmap::mapref::one::{Ref, RefMut};
use dashmap::DashMap;

use crate::domain::storage::data::reservation::{
    Reservation, ReservationFactory, ReservationFactoryV1,
};
use crate::domain::{make_user_token, ItemToken, ReservationId, UserId, UserToken};
use crate::foundation::errors::{rsv_conflict, rsv_not_found, user_not_conformant};

/// Reservation Storage.
///
/// Different from other storages, such as `UserStorage`, which might be singletons, there may be
/// multiple instances of reservation storages present. For example, we may need temporary storage
/// for active reservations which are not confirmed.
pub trait Storage: Sync {
    fn store(&self, r: Reservation);

    /// Add the item to the reservation list, does not check if the item exists (it can't anyway).
    ///
    /// # Error
    /// - if user id not conformant
    /// - if reservation not found.
    fn authenticated_add_item(&self, tok: ItemToken) -> Result<()>;

    fn authenticated_remove_item(&self, tok: ItemToken) -> Result<()>;

    /// Generate a summary of the reservation.
    ///
    /// # Error
    /// similar.
    fn authenticated_summary(&self, tok: UserToken) -> Result<String>;

    /// Extract a reservation from the storage.
    ///
    /// # Error
    /// similar.
    fn authenticated_extract(&self, tok: UserToken) -> Result<Reservation>;
}

/// Provide both creating & storing abilities.
pub trait CreativeStorage: Storage {
    fn new_reservation(&self, user_id: UserId) -> ReservationId;
}

pub struct StorageV1<'f> {
    // Initially, I used a `RwLock<HashMap<...>>` to implement this. Later, I found this nice
    // lib called `DashMap` which is exactly what I needed.
    reservations: DashMap<ReservationId, Reservation>,
    factory: &'f ReservationFactoryV1,
}

impl<'f> CreativeStorage for StorageV1<'f> {
    /// Create a new reservation for the user. Does not check if the user id is valid.
    ///
    /// # Panic
    /// if the id of the created reservation conflicts with any one else in the storage.
    fn new_reservation(&self, user_id: UserId) -> ReservationId {
        let reservation = self.factory.with_user_id(user_id);

        let id = reservation.id();
        if self.reservations.insert(id, reservation).is_none() {
            panic!(rsv_conflict());
        }

        id
    }
}

impl<'f> Storage for StorageV1<'f> {
    fn store(&self, r: Reservation) {
        assert!(
            self.reservations.insert(r.id(), r).is_none(),
            rsv_conflict()
        );
    }

    fn authenticated_add_item(&self, tok: ItemToken) -> Result<()> {
        let mut reservation = self.checked_rsv_mut(make_user_token(&tok))?;
        reservation.add(tok.2.clone())
    }

    fn authenticated_remove_item(&self, tok: ItemToken) -> Result<()> {
        let mut reservation = self.checked_rsv_mut(make_user_token(&tok))?;
        reservation.remove(tok.2)
    }

    fn authenticated_summary(&self, tok: UserToken) -> Result<String> {
        let reservation = self.checked_rsv(tok)?;
        Ok(reservation.summary())
    }

    fn authenticated_extract(&self, tok: UserToken) -> Result<Reservation> {
        let (_, reservation) = self.reservations.remove(tok.1).ok_or(rsv_not_found())?;

        if reservation.user_id() != tok.0 {
            self.reservations
                .insert(tok.1.clone(), reservation)
                .expect("SEVERE! Reservation data lost due to internal error of DashMap");
            return Err(user_not_conformant());
        }

        Ok(reservation)
    }
}

/// How DRY should we be?
///
/// When I was writing this block the other time, I wrote a macro consisting of the logic, and
/// then called it with different method name. When I finished, I reviewed the block and asked
/// myself: "Is it all worth it?"
///
/// No. It might be hard to admit, but the truth is that it is deterministic that we only need two
/// functions, one mut and the other not. We not gonna reuse the logic anywhere else, and copying
/// or pasting the code snippet is not that great an effort.
///
/// It was not until then that I started to suspect on the DRY principle. DRY does not come with
/// no cost. When we say "we should be DRY" we mean that the cost of staying DRY is worth it. When
/// the premise is no longer held, we have no incentive any more to keep the code DRY.
impl<'f> StorageV1<'f> {
    // rsv == reservation
    fn checked_rsv_mut(&self, tok: UserToken) -> Result<RefMut<ReservationId, Reservation>> {
        let reservation = self
            .reservations
            .get_mut(tok.1)
            .ok_or(anyhow!("Reservation not found"))?;
        if reservation.user_id() != tok.0 {
            Err(user_not_conformant())
        } else {
            Ok(reservation)
        }
    }

    fn checked_rsv(&self, tok: UserToken) -> Result<Ref<ReservationId, Reservation>> {
        let reservation = self
            .reservations
            .get(tok.1)
            .ok_or(anyhow!("Reservation not found"))?;
        if reservation.user_id() != tok.0 {
            Err(user_not_conformant())
        } else {
            Ok(reservation)
        }
    }
}
