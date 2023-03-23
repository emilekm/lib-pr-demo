use bincode::{de::Decoder, error::DecodeError};
use bincode::{impl_borrow_decode, Decode};
use derive::DecodeWithFlags;

use crate::fields::ZeroEndedString;

use super::Position;

#[derive(Decode, Debug)]
pub struct PlayerAdd {
    id: u8,
    ign: ZeroEndedString,
    hash: ZeroEndedString,
    ip: ZeroEndedString,
}

#[derive(Decode, Debug)]
pub struct PlayerRemove {
    id: u8,
}

#[derive(Debug)]
pub struct PlayerVehicle {
    id: i16,
    seat_name: Option<ZeroEndedString>,
    seat_number: Option<i8>,
}

impl Decode for PlayerVehicle {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let id = i16::decode(decoder)?;

        Ok(Self {
            id,
            seat_name: {
                if id >= 0 {
                    Some(ZeroEndedString::decode(decoder)?)
                } else {
                    None
                }
            },
            seat_number: {
                if id >= 0 {
                    Some(i8::decode(decoder)?)
                } else {
                    None
                }
            },
        })
    }
}

impl_borrow_decode!(PlayerVehicle);

#[derive(Debug, DecodeWithFlags)]
pub struct PlayerUpdate {
    #[flags_field]
    flags: u16,
    id: u8,
    #[flag = 1]
    team: Option<i8>,
    #[flag = 2]
    squad: Option<u8>,
    #[flag = 4]
    vehicle: Option<PlayerVehicle>,
    #[flag = 8]
    health: Option<i8>,
    #[flag = 16]
    score: Option<i16>,
    #[flag = 32]
    teamwork_score: Option<i16>,
    #[flag = 64]
    kills: Option<i16>,
    #[flag = 256]
    deaths: Option<i16>,
    #[flag = 512]
    ping: Option<i16>,
    #[flag = 2048]
    is_alive: Option<bool>,
    #[flag = 4096]
    is_joining: Option<bool>,
    #[flag = 8192]
    position: Option<Position>,
    #[flag = 16384]
    rotation: Option<i16>,
    #[flag = 32768]
    kit_name: Option<ZeroEndedString>,
}

#[derive(Debug, Decode)]
pub struct Kill {
    attacker_id: u8,
    victim_id: u8,
    weapon: ZeroEndedString,
}

#[derive(Debug, Decode)]
pub struct Chat {
    channel: u8,
    player_id: u8,
    message: ZeroEndedString,
}

#[derive(Debug, Decode)]
pub struct Revive {
    medic_id: u8,
    revived_id: u8,
}

#[derive(Debug, Decode)]
pub struct KitAllocated {
    player_id: u8,
    kit_name: ZeroEndedString,
}
