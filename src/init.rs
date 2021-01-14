use crate::domain::{ReservationId, ReservableItemId};
use anyhow::{Result, Error, Context};
use std::io::{Read, BufReader}
use serde::{Deserialize, Serialize};
use std::fs::File;
use crate::foundation::file_reader::SimpleReader;
use crate::foundation::serde::deserialize_from;
use crate::domain::storage::{users, flights, reservations};


/// Initialize the program.
///
/// # Return
/// Sessions of the domain layer.
pub fn init() {
    // init storages
    let users: users::StorageV1 = from(USER_STORAGE);
    let flights: flights::StorageV1 = from(FLIGHT_STORAGE);
    let reservations: reservations::StorageV1 = from(RSV_STORAGE);

    // init [ReservationFactory]
    let stats = match program_stats() {
        Ok(stats) => stats,
        Err(err) => {
            eprintln!(PROGRAM_STATS_NOT_FOUND_MSG, err);
        }
    };

    fn from<T>(filename: &'static str) -> T {
        deserialize_from(filename).context(filename).unwrap()
    }

    fn full_search_next_id() -> ReservationId {
        let ret = ReservationId::default();

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
const PROGRAM_STATS_NOT_FOUND_MSG: &str =
r"WARNING: Program-statistics file not found.
    Running total search of storages to identify the proper reservation id.
    - The Error: {}";


/// Statistics of historical runs.
///
/// Currently, the stats consist of conly
#[derive(Serialize, Deserialize)]
struct ProgramStats {
    id_pool: ReservationId, // Next id to use
}