use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::path::Path;

/// Simple helper writer that opens a file and overwrites into it.
pub struct SimpleWriter {
    buf: BufWriter<File>,
}

impl SimpleWriter {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect("Failed to open file");
        SimpleWriter {
            buf: BufWriter::new(f),
        }
    }

    pub fn writer(&mut self) -> &mut BufWriter<File> {
        &mut self.buf
    }
}
