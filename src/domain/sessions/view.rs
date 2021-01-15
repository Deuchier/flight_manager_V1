//! View various data.
//!
//! This session handles queries of user profiles, and flights, etc.
use crate::domain::storage::data::flight::Address;
use crate::domain::storage::{flights, reservations, users};
use crate::domain::{UserId, UserToken};
use anyhow::Result;
use std::sync::Arc;

/// View Tickets Session.
///
/// Should implement various querying methods.
pub trait Session {
    /// Query about a flight. Returns serialized report.
    fn query_flights(&self, src: &Address, dest: &Address) -> Result<String>;

    /// View reservations of a user.
    ///
    /// # Returns
    /// 0. Undone reservations
    /// 1. Done reservations
    ///
    /// # Error
    /// if user not found
    fn view_reservations(&self, tok: UserToken) ->Result<(Vec<String>, Vec<String>)>;
}

pub struct SessionV1 {
    users: Arc<dyn users::Storage>,
    reservations: Arc<dyn reservations::Storage>,
    flights: Arc<dyn flights::Storage>,
}

impl Session for SessionV1 {
    fn query_flights(&self, src: &Address, dest: &Address) -> Result<String> {
        self.flights.query(src, dest)
    }

    fn view_reservations(&self, tok: UserToken) -> Result<(Vec<String>, Vec<String>)> {
        let a = self
            .users
            .undone_reservations(tok.0)?
            .into_iter()
            .map(|r_id| self.reservations.summary(tok));
        let b = self
            .users
            .done_reservations(tok.0).unwrap()
            .into_iter()
            .map(|r_id| self.reservations.summary(tok));

        let mut aa = Vec::new();
        let mut bb = Vec::new();
        for i in a {
            aa.push(i?)
        }
        for i in b {
            bb.push(i?)
        }

        Ok((aa, bb))
    }
}

impl SessionV1 {
    pub unsafe fn from_components(
        users: Arc<dyn users::Storage>,
        reservations: Arc<dyn reservations::Storage>,
        flights: Arc<dyn flights::Storage>,
    ) -> Self {
        Self {
            users,
            reservations,
            flights,
        }
    }
}
