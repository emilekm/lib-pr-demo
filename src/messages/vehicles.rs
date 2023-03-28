use bincode::{de::Decoder, error::DecodeError, Decode};
use derive::{DecodeMultiple, DecodeWithFlags};

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

#[derive(Debug, DecodeMultiple)]
pub struct VehiclesUpdate {
    vehicles: Vec<VehicleUpdate>,
}

#[derive(Debug, Decode)]
pub struct VehicleAdd {
    id: i16,
    name: ZeroEndedString,
    max_health: u16,
}

#[derive(Debug, DecodeMultiple)]
pub struct VehiclesAdd {
    vehicles: Vec<VehicleAdd>,
}

#[derive(Debug, Decode)]
pub struct VehicleDestroyed {
    id: i16,
    is_killer_known: bool,
    killer_id: u8,
}
