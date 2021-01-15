//! # Unstable
//! This module was directly copied from previous implementations. The transplant may have a lot of
//! problems. The previous implementation did not support concurrent access either.
//!
//! - Some transplant rejections occurred, but no longer. It is now working rather stably, perhaps
//!   later sometime I should remove the "Unstable" note.
use crate::domain::storage::data::item::ReservableItem;
use crate::domain::storage::items;
use crate::domain::{FlightId, ReservableItemId};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Flight {
    id: FlightId,
    company_id: String,
    plane: PlaneInfo,
    src: AirportInfo,
    dest: AirportInfo,
    items: ItemMap,
}

pub type ItemMap = items::SimpleStorage;

impl Flight {
    pub fn flight_id(&self) -> &FlightId {
        &self.id
    }

    pub fn company_id(&self) -> &str {
        &self.company_id
    }

    pub fn items(&self) -> &ItemMap {
        &self.items
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaneInfo {
    id: String,
    r#type: String,
}

impl PlaneInfo {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct AirportInfo {
    name: String,
    addr: Address,
}

impl AirportInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn addr(&self) -> &Address {
        &self.addr
    }
}

/// Address arranged in hierarchical Segments. Addresses are used in describing the airport infos.
///
/// Generally the hierarchy is from larger regions to smaller ones.
/// - \[0]: Continent
///     - \[1]: Nation
///     - \[2]: Province / State
///     - ...
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Hash)]
pub struct Address {
    locations: Vec<String>,
}

impl Address {
    pub fn location(&self, level: usize) -> Option<&str> {
        self.locations.get(level).map(|i| i.as_str())
    }
}

#[cfg(test)]
mod old_tests {
    use super::*;
    use std::error::Error;
    use std::fs::{File, OpenOptions};
    use std::io::{BufReader, BufWriter};

    type Ret = Result<(), Box<dyn Error>>;

    #[test]
    fn serde_flight() -> Ret {
        let mut flight = Flight {
            id: "TEST1029021".to_string(),
            company_id: "TestCompany".to_string(),
            plane: PlaneInfo {
                id: "991203".to_string(),
                r#type: "C919".to_string(),
            },
            src: AirportInfo {
                name: "Homeland Town".to_string(),
                addr: address(),
            },
            dest: AirportInfo {
                name: "Merry Land".to_string(),
                addr: address(),
            },
            items: DashMap::new(),
        };
        flight.items = unimplemented!();

        const FILENAME: &str = "./tmp/test_flight.json";
        let mut out = output(FILENAME);
        serde_json::to_writer_pretty(&mut out, &flight)?;

        let mut input = input(FILENAME);
        let de_flight: Flight = serde_json::from_reader(&mut input)?;

        assert_eq!(flight.id, de_flight.id);
        assert_eq!(flight.company_id, de_flight.company_id);
        assert_eq!(flight.dest, de_flight.dest);
        assert_eq!(flight.plane.id, de_flight.plane.id);

        Ok(())
    }

    #[test]
    fn serde_airport() -> Ret {
        let info = AirportInfo {
            name: "TestAirport".to_string(),
            addr: address(),
        };
        let mut out = output("./tmp/airport_info.json");
        serde_json::to_writer_pretty(&mut out, &info)?;
        Ok(())
    }

    #[test]
    fn test_address() {
        let addr = address();
        assert_eq!(addr.location(0), Some("Europe"));
        assert_eq!(addr.location(1), Some("Germany"));
        assert_eq!(addr.location(2), Some("Berlin"));
        assert_eq!(addr.location(3), None);
    }

    fn address() -> Address {
        Address {
            locations: vec!["Europe", "Germany", "Berlin"]
                .into_iter()
                .map(|i| i.to_string())
                .collect(),
        }
    }

    fn input(filename: &str) -> BufReader<File> {
        let file = std::fs::File::open(filename).expect("Failed to open test file");
        BufReader::new(file)
    }

    fn output(filename: &str) -> BufWriter<File> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename)
            .expect("Failed to open test file");
        BufWriter::new(file)
    }
}
