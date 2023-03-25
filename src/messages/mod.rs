use bincode::de::Decoder;
use bincode::error::DecodeError;
use bincode::{impl_borrow_decode, Decode};

pub mod map;
pub use map::*;

pub mod players;
pub use players::*;

pub mod teams;
pub use teams::*;

pub mod vehicles;
pub use vehicles::*;

#[derive(Debug)]
pub struct Header {
    pub length: u16,
    pub typ: u8,
}

impl Decode for Header {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        Ok(Self {
            length: u16::decode(decoder)? - 1, // substract the type size decoded bellow
            typ: u8::decode(decoder)?,
        })
    }
}

impl_borrow_decode!(Header);
