use bincode::{de::Decoder, error::DecodeError, Decode};
use derive::DecodeWithFlags;

use crate::fields::ZeroEndedString;

use super::Position;

#[derive(Debug, DecodeWithFlags)]
pub struct VehicleUpdate {
    #[flags_field]
    flags: u8,
    id: i16,
    #[flag = 1]
    team: Option<i8>,
    #[flag = 2]
    position: Option<Position>,
    #[flag = 4]
    rotation: Option<i16>,
    #[flag = 8]
    health: Option<i8>,
}

#[derive(Debug, Decode)]
pub struct VehicleAdd {
    id: i16,
    name: ZeroEndedString,
    max_health: u16,
}
