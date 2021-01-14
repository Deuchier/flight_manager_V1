use crate::domain::storage::data::flight::{Address, Flight};
use crate::foundation::errors::flight_not_found;
use anyhow::Result;
use dashmap::DashMap;
use std::borrow::Borrow;
use serde::{Deserialize, Serialize};
use crate::domain::{FlightId, ReservableItemId};
use crate::domain::storage::items;

pub trait Storage: Sync {
    /// Generate a report of flights from `src` to `dest`.
    fn query(&self, src: &Address, dest: &Address) -> Result<String>;
}

/// Outer: Src, Inner: Dest
type FlightMap = DashMap<Address, DashMap<Address, Vec<FlightId>>>;

#[derive(Serialize, Deserialize)]
pub struct StorageV1 {
    data: DashMap<FlightId, Flight>,
    flights: FlightMap,
}

/// Let's mimic as an item storage (Composite)
impl items::Storage for StorageV1 {
    fn occupy(&self, item_id: &ReservableItemId) -> Result<()> {
        self.inner_item_storage(item_id)?.occupy(item_id)
    }

    fn release(&self, item_id: &ReservableItemId) -> Result<()> {
        self.inner_item_storage(item_id)?.release(item_id)
    }
}

impl Storage for StorageV1 {
    fn query(&self, src: &Address, dest: &Address) -> Result<String> {
        let refs: Vec<_> = self.flights
            .get(src)
            .ok_or(flight_not_found())?
            .get(dest)
            .ok_or(flight_not_found())?
            .value()
            .iter()
            .map(|i| {
                self.data.get(i).expect(FLIGHT_MISSING)
            }).collect();

        Ok(serde_json::to_string(&refs)?)
    }
}

const FLIGHT_MISSING: &str =
    r#"Flight missing which should be in the storage, since we already have ids of them stored.
Check if there are any logical errors."#;

impl StorageV1 {
    /// (Composite Pattern) Get inner storage the item is in.
    fn inner_item_storage(&self, item_id: &ReservableItemId) -> Result<&dyn items::Storage> {
        //! (Put in the report)
        //!
        //! I used a dyn trait [item::Storage] as the inner storage of [Flight]. Later when I was
        //! refactoring [flights::Storage], I was surprised and amused to find that I could
        //! directly designate the task to the inner storage!
        //!
        //! That is the charm of Software Engineering. High usability, low mental burden.
        Ok(self.data.get(item_id.flight_id()).ok_or(flight_not_found())?.items())
    }
}