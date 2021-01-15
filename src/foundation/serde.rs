use crate::foundation::file_reader::SimpleReader;
use crate::foundation::file_writer::SimpleWriter;
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub fn deserialize_from<R: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<R> {
    Ok(serde_json::from_reader(SimpleReader::new(path).reader())?)
}

pub fn serialize_into<T: Serialize, P: AsRef<Path>>(path: P, value: &T) -> Result<()> {
    Ok(serde_json::to_writer(
        SimpleWriter::new(path).writer(),
        value,
    )?)
}

pub fn serialize_into_pretty<T: Serialize, P: AsRef<Path>>(path: P, value: &T) -> Result<()> {
    Ok(serde_json::to_writer_pretty(
        SimpleWriter::new(path).writer(),
        value,
    )?)
}
