use bincode::de::Decoder;
use bincode::error::DecodeError;
use bincode::{impl_borrow_decode, Decode};

pub mod headers;
pub use headers::{Map, ServerDetails};

pub mod players;
pub use players::{PlayerAdd, PlayerRemove, PlayerUpdate};

pub mod vehicles;
pub use vehicles::{VehicleAdd, VehicleUpdate};

// pub struct Message<T: Decode> {
//     header: Header,
//     pos: u64,
// }

// impl Message {
//     pub fn read()
// }

#[derive(Debug)]
pub struct Header {
    pub length: u16,
    pub typ: u8,
}

impl Decode for Header {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        Ok(Self {
            length: u16::decode(decoder)? - 1,
            typ: u8::decode(decoder)?,
        })
    }
}

impl_borrow_decode!(Header);

#[derive(Decode, Debug)]
pub struct Position {
    x: i16,
    y: i16,
    z: i16,
}
