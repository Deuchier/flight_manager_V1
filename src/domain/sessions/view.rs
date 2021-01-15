//! View various data.
//!
//! This session handles queries of user profiles, and flights, etc.
use crate::domain::sessions::Query;
use crate::domain::storage::flights;
use anyhow::Result;
use std::sync::Arc;

/// View Tickets Session.
///
/// Should implement various querying methods.
pub trait Session {
    /// Query about a flight. Returns serialized report.
    fn query_flights(&self, q: Query) -> Result<String>;
}

pub struct SessionV1 {
    flights: Arc<dyn flights::Storage>,
}

impl Session for SessionV1 {
    fn query_flights(&self, q: Query) -> Result<String> {
        self.flights.query(q.src(), q.dest())
    }
}

impl SessionV1 {
    pub unsafe fn from_components(flights: Arc<dyn flights::Storage>) -> Self {
        Self { flights }
    }
}
