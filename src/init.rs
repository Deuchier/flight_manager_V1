use crate::domain::sessions::{refund, reserve_tickets, view};
use crate::domain::storage::data::reservation::ReservationFactoryV1;
use crate::domain::storage::{flights, reservations, users, RsvMap};
use crate::domain::{ReservableItemId, ReservationId};
use crate::foundation::file_reader::SimpleReader;
use crate::foundation::serde::deserialize_from;
use anyhow::{Context, Error, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::{Arc, RwLock};

/// A pack containing long-term structs that have been initialized.
pub struct GlobalPack<'a> {
    reservation_factory: ReservationFactoryV1,

    users: Arc<users::StorageV1>,
    flights: Arc<flights::StorageV1>,
    reservations: reservations::StorageV1,

    reserve_tickets: reserve_tickets::SessionV1,
}

/// Initialize the program.
///
/// # Return
/// Sessions of the domain layer.
pub fn init() {
    // 0. init storages.
    let users = Arc::new(from(USER_STORAGE));
    let flights = Arc::new(from(FLIGHT_STORAGE));

    // The reservation storage needs a bit special care.
    let reservations: RsvMap = from(RSV_STORAGE);

    //     first, init [ReservationFactory]
    let stats = program_stats().unwrap_or({
        eprintln!("{}", PROGRAM_STATS_NOT_FOUND_MSG);
        ProgramStats {
            id_pool: full_search_next_id(&reservations),
        }
    });
    let reservation_factory = unsafe { Arc::new(ReservationFactoryV1::new(stats.id_pool)) };

    //     then, init [reservations::StorageV1]
    let reservations = unsafe {
        reservations::StorageV1::from_components(reservations, reservation_factory.clone())
    };

    // 1. init sessions
    let empty_reservation_storage = unsafe {
        Box::new(reservations::StorageV1::from_components(
            RsvMap::new(),
            reservation_factory.clone(),
        ))
    };

    let refund = unsafe { refund::SessionV1::from_components(users.clone()) };
    let reserve_tickets = unsafe {
        reserve_tickets::SessionV1::from_components(
            RwLock::new(vec![]),
            empty_reservation_storage,
            users.clone(),
            flights.clone(),
        )
    };
    let view = unsafe { view::SessionV1::from_components(flights.clone()) };


    unimplemented!()

    // helpers

    fn from<T: DeserializeOwned>(filename: &'static str) -> T {
        deserialize_from(filename).context(filename).unwrap()
    }

    fn full_search_next_id(reservations: &RsvMap) -> ReservationId {
        reservations
            .iter()
            .map(|i| i.value().id())
            .max()
            .unwrap_or(0)
            + 1
    }
}

/// Get the program statistics.
///
/// # Error
/// - if the file [PROGRAM_STATS_FILENAME] is not found.
/// - if it cannot be parsed correctly.
fn program_stats() -> Result<ProgramStats> {
    Ok(deserialize_from(PROGRAM_STATS_FILENAME)?)
}

const RSV_STORAGE: &str = "reservation_storage.data";
const USER_STORAGE: &str = "user_storage.data";
const FLIGHT_STORAGE: &str = "flight_storage.data";
const PROGRAM_STATS_FILENAME: &str = "program_stats.data";
const PROGRAM_STATS_NOT_FOUND_MSG: &str = r"WARNING: Program-statistics file not found.
    Running total search of storages to identify the proper reservation id.
    - The Error: ";

/// Statistics of historical runs.
///
/// Currently, the stats consist of conly
#[derive(Serialize, Deserialize)]
struct ProgramStats {
    id_pool: ReservationId, // Next id to use
}
