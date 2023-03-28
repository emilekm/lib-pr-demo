use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
};

use bincode::{config, de::read::Reader, error::DecodeError};
use compress::zlib;

use crate::{
    messages::{self, LengthSize, MessageType},
    types::Messages,
};

// At least a length (u16) and type (u8) are required for a valid message to be read
const MIN_MESSAGE_LEN: u64 = 3;

type DemoConfig = config::Configuration<
    config::LittleEndian,
    config::Fixint,
    config::SkipFixedArrayLength,
    config::NoLimit,
>;

const CONFIG: DemoConfig = bincode::config::standard()
    .with_little_endian()
    .with_fixed_int_encoding()
    .skip_fixed_array_length();

pub struct DemoReader {
    config: DemoConfig,
    container: Container<Vec<u8>>,
    read: Vec<MessageType>,
}

impl DemoReader {
    pub fn new(file: File, read: Vec<MessageType>) -> Self {
        let mut decompressed = Vec::new();
        _ = zlib::Decoder::new(file).read_to_end(&mut decompressed);

        Self {
            container: Container {
                cursor: Cursor::new(decompressed),
            },
            config: CONFIG,
            read,
        }
    }

    pub fn skip_message(&mut self, size: LengthSize) -> u64 {
        self.container
            .cursor
            .seek(SeekFrom::Current(size as i64))
            .unwrap()
    }

    pub fn has_message(&mut self) -> bool {
        self.container.cursor.get_ref().len() as u64 - self.container.cursor.position()
            >= MIN_MESSAGE_LEN
    }

    pub fn read_message(&mut self) -> Result<Messages, DecodeError> {
        let config = self.config;
        let header: messages::Header = bincode::decode_from_reader(&mut self.container, config)?;

        // Skip message if it is not in self.read
        if self.read.contains(&header.typ) {
            let mut buf = vec![0u8; header.length as usize];
            self.container.read(&mut buf).unwrap();
            let cursor = Cursor::new(buf);
            let container: &mut Container<Vec<u8>> = &mut Container { cursor };
            match header.typ {
                MessageType::ServerDetails => Ok(Messages::ServerDetails(
                    bincode::decode_from_reader(container, config)?,
                )),
                MessageType::PlayerUpdate => Ok(Messages::PlayerUpdate(
                    bincode::decode_from_reader(container, config)?,
                )),
                MessageType::PlayerAdd => Ok(Messages::PlayerAdd(bincode::decode_from_reader(
                    container, config,
                )?)),
                MessageType::PlayerRemove => Ok(Messages::PlayerRemove(
                    bincode::decode_from_reader(container, config)?,
                )),
                MessageType::VehicleUpdate => Ok(Messages::VehicleUpdate(
                    bincode::decode_from_reader(container, config)?,
                )),
                _ => Ok(Messages::Skip(header)),
            }
        } else {
            Ok(Messages::Skip(header))
        }
    }
}

struct Container<T> {
    cursor: Cursor<T>,
}

impl Reader for Container<Vec<u8>> {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        self.cursor
            .read_exact(bytes)
            .map_err(|inner| DecodeError::Io {
                inner,
                additional: bytes.len(),
            })
    }
}

impl Reader for Container<&mut [u8]> {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        self.cursor
            .read_exact(bytes)
            .map_err(|inner| DecodeError::Io {
                inner,
                additional: bytes.len(),
            })
    }
}
