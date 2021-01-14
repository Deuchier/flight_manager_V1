//! Convenient errors.

use anyhow::{anyhow, Error};

pub fn rsv_not_found() -> Error {
    anyhow!("Reservation not found")
}

pub fn rsv_conflict() -> Error {
    anyhow!("Reservation Id conflicted")
}

pub fn item_not_found() -> Error {
    anyhow!("Item not found")
}

pub fn item_not_available() -> Error {
    anyhow!("Item not available")
}

pub fn user_not_found() -> Error {
    anyhow!("User not found")
}

pub fn user_not_conformant() -> Error {
    anyhow!("User id not conformant with the reservation")
}

pub fn flight_not_found() -> Error {
    anyhow!("Flight not found")
}
