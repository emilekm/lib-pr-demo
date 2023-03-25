use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
};

use bincode::{config, de::read::Reader, error::DecodeError};
use compress::zlib;

use crate::{
    messages,
    types::{self, Messages},
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
    cursor: Cursor<Vec<u8>>,
}

impl DemoReader {
    pub fn new(file: File) -> Self {
        let mut decompressed = Vec::new();
        _ = zlib::Decoder::new(file).read_to_end(&mut decompressed);

        Self {
            cursor: Cursor::new(decompressed),
            config: CONFIG,
        }
    }

    pub fn skip_message(&mut self, size: u16) -> u64 {
        self.cursor.seek(SeekFrom::Current(size as i64)).unwrap()
    }

    pub fn has_message(&mut self) -> bool {
        self.cursor.get_ref().len() as u64 - self.cursor.position() >= MIN_MESSAGE_LEN
    }

    pub fn read_message(mut self) -> Result<Messages, DecodeError> {
        let config = self.config;
        let header: messages::Header = bincode::decode_from_reader(&mut self, config)?;
        match header.typ {
            types::SERVER_DETAILS => Ok(Messages::ServerDetails(bincode::decode_from_reader(
                &mut self, config,
            )?)),
            types::PLAYER_UPDATE => Ok(Messages::PlayerUpdate(bincode::decode_from_reader(
                &mut self, config,
            )?)),
            types::PLAYER_ADD => Ok(Messages::PlayerAdd(bincode::decode_from_reader(
                &mut self, config,
            )?)),
            types::PLAYER_REMOVE => Ok(Messages::PlayerRemove(bincode::decode_from_reader(
                &mut self, config,
            )?)),
            types::VEHICLE_UPDATE => Ok(Messages::VehicleUpdate(bincode::decode_from_reader(
                &mut self, config,
            )?)),
            _ => Ok(Messages::Invalid(header)),
        }
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
