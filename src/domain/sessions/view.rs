//! View various data.
//!
//! This session handles queries of user profiles, and flights, etc.
use crate::domain::sessions::Query;
use crate::domain::storage::flights;
use anyhow::Result;

/// View Tickets Session.
///
/// Should implement various querying methods.
pub trait Session {
    /// Query about a flight. Returns serialized report.
    fn query_flights(&self, q: Query) -> Result<String>;
}

pub struct SessionV1<'a> {
    flights: &'a dyn flights::Storage,
}

impl<'a> Session for SessionV1<'a> {
    fn query_flights(&self, q: Query) -> Result<String> {
        self.flights.query(q.src(), q.dest())
    }
}
