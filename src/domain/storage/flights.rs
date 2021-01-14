use crate::domain::storage::data::flight::{Address, Flight};
use crate::foundation::errors::flight_not_found;
use anyhow::Result;
use dashmap::DashMap;
use std::borrow::Borrow;

pub trait Storage: Sync {
    /// Generate a report of flights from `src` to `dest`.
    fn query(&self, src: &Address, dest: &Address) -> Result<String>;
}

/// Outer: Src, Inner: Dest
type FlightMap = DashMap<Address, DashMap<Address, Vec<Flight>>>;

pub struct StorageV1 {
    flights: FlightMap,
}

impl Storage for StorageV1 {
    fn query(&self, src: &Address, dest: &Address) -> Result<String> {
        Ok(serde_json::to_string(
            self.flights
                .get(src)
                .ok_or(flight_not_found())?
                .get(dest)
                .ok_or(flight_not_found())?
                .value(),
        )?)
    }
}
