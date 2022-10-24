use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
};

use bincode::{de::read::Reader, error::DecodeError};
use compress::zlib;

pub struct DemoReader {
    cursor: Cursor<Vec<u8>>,
}

impl DemoReader {
    pub fn from_file(file: File) -> Self {
        let mut decompressed = Vec::new();
        _ = zlib::Decoder::new(file).read_to_end(&mut decompressed);

        Self {
            cursor: Cursor::new(decompressed),
        }
    }

    pub fn skip_message(&mut self, size: u16) -> u64 {
        self.cursor.seek(SeekFrom::Current(size as i64)).unwrap()
    }
}

impl Reader for DemoReader {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        self.cursor
            .read_exact(bytes)
            .map_err(|inner| DecodeError::Io {
                inner,
                additional: bytes.len(),
            })
    }
}
