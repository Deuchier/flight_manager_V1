use crate::domain::ReservationId;
use std::io::Read;
use anyhow::Result;
use serde::Deserialize;

/// Get the program statistics.
pub fn program_stats() -> ProgramStats {

}

/// The statistics from previous runs. When the program restarts, it will read its storage from
/// this struct.
pub struct ProgramStats {
    id_pool: ReservationId, // Next id to use
}


impl ProgramStats {
    pub fn reservation_id_pool(&self) -> ReservationId {
        self.id_pool
    }

    fn from<R: Read>(src: &mut R) -> Result<Self> {
        unimplemented!()
    }
}