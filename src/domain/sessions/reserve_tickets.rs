use anyhow::Result;

use crate::domain::sessions::{ItemToken, UserToken};
use crate::domain::ReservationId;

/// Reserve-Tickets Session.
pub trait Session {
    /// Start a new reservation for the user. Returns a unique id for identifying the reservation.
    ///
    /// This function should always succeed, or panic.
    ///
    /// The function receives the user's id as a `String` and stores it in the reservation.
    ///
    /// # Temporary Reservation
    /// This call starts a temporary reservation. Until confirmed, the reservation may not be
    /// stored persistently.
    fn start_reservation(&mut self, user_id: String) -> ReservationId;

    /// Adds a reservable item to the reservation list.
    ///
    /// Will exclusively occupy the item.
    ///
    /// # Error
    /// Returns `Err` if the reservation is confirmed or not valid (i.e. aborted).
    fn add(&mut self, token: ItemToken) -> Result<()>;

    /// Removes an item from the list.
    ///
    /// Similar to `add`.
    fn remove(&mut self, token: ItemToken) -> Result<()>;

    /// Gets a summary of the current state of the reservation.
    ///
    /// # Return
    /// Returns a serialized form of the reservation.
    ///
    /// <del>AFA (Jun. 11 '20), I'm using the YAML form instead of the more famous JSON, since YAML is
    /// more concise and readable than JSON.</del><br>
    ///     I've switched back to JSON since it has the best library support.
    fn summary(&self, token: ItemToken) -> Result<String>;

    /// Confirms an reservation.
    ///
    /// Calling this function will terminate modifying process of the reservation (i.e. `add` or
    /// `remove`). Any subsequent calls to the functions or this function will result in an error.
    ///
    /// # Persistent Storage
    /// Calling this function will cause the reservation to be persistently stored into the user's
    /// profile. This function must be called before ending a reservation, unless errors occur, or
    /// the user wish to abort the reservation.
    fn confirm(&mut self, token: UserToken) -> Result<()>;

    /// Aborts a reservation that's not paid yet.
    ///
    /// The aborted reservation will not be stored in the system.
    ///
    /// # Error
    /// If the reservation has already been paid, then the function will return `Err`. You should
    /// use the `refund` functionality instead of `abort`.
    fn abort(&mut self, token: UserToken) -> Result<()>;

    /// Pays for a reservaiton.
    ///
    /// # Error
    /// Returns `Err` if the payment failed, or the reservation is already paid.
    fn pay(&mut self, token: UserToken) -> Result<()>;
}
