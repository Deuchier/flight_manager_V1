use std::path::Path;
use anyhow::Result;
use crate::foundation::file_reader::SimpleReader;
use crate::foundation::file_writer::SimpleWriter;

pub fn deserialize_from<R, P: AsRef<Path>>(path: P) -> Result<R> {
    Ok(serde_json::from_reader(SimpleReader::new(path).reader())?)
}

pub fn serialize_into<T, P: AsRef<Path>>(path: P, value: &T) -> Result<()> {
    Ok(serde_json::to_writer(SimpleWriter::new(path).writer(), value)?)
}

pub fn serialize_into_pretty<T, P: AsRef<Path>>(path: P, value: &T) -> Result<()> {
    Ok(serde_json::to_writer_pretty(SimpleWriter::new(path).writer(), value)?)
}