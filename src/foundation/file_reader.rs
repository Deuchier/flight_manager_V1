use std::fs::{File, };
use std::io::{ BufReader};
use std::path::Path;

/// Simple helper writer that opens a file and overwrites into it.
pub struct SimpleReader {
    buf: BufReader<File>,
}

impl SimpleReader {
    pub fn new<P: AsRef<Path>>(filename: P) -> Self {
        SimpleReader {
            buf: BufReader::new(File::open(filename).expect("Failed to open file")),
        }
    }

    pub fn reader(&mut self) -> &mut BufReader<File> {
        &mut self.buf
    }
}
