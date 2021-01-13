use crate::domain::sessions::Query;
use anyhow::Result;

/// View Tickets Session.
///
/// Should implement various querying methods.
pub trait Session {
    fn query(&self, q: Query) -> Result<String>;
}

pub struct SessionV1 {}

impl Session for SessionV1 {
    fn query(&self, q: Query) -> Result<String> {
        unimplemented!()
    }
}
