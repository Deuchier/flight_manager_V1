use crate::domain::payment::Refund;
use crate::domain::storage::data::reservation::Reservation;
use crate::domain::storage::{reservations, users};
use crate::domain::{ReservationId, UserId, UserToken};
use anyhow::{anyhow, Result};
use std::sync::Arc;

/// Refund Session
pub trait Session {
    fn refund(&self, tok: UserToken, method: Box<dyn Refund>) -> Result<steel_cent::Money>;
}

pub struct SessionV1 {
    users: Arc<dyn users::Storage>,
    reservations: Arc<dyn reservations::Storage>, // todo: user do not hold reservations.
}

impl SessionV1 {
    pub unsafe fn from_components(
        users: Arc<dyn users::Storage>,
        reservations: Arc<dyn reservations::Storage>,
    ) -> Self {
        Self {
            users,
            reservations,
        }
    }
}

impl Session for SessionV1 {
    // This implementation is very similar to that of [reserve_tickets::SessionV1::pay]
    fn refund(&self, tok: UserToken, method: Box<dyn Refund>) -> Result<steel_cent::Money> {
        let ret = self
            .reservations
            .process(tok, Box::new(move |rsv| method.refund(rsv)))?;
        self.users.withdraw(tok);
        Ok(ret)
    }
}
