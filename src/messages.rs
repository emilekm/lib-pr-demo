use crate::fields::{Timestamp, ZeroEndedString};
use crate::types::MessageType;
use bincode::de::Decoder;
use bincode::error::DecodeError;
use bincode::{impl_borrow_decode, Decode};

#[derive(Debug)]
pub struct Header {
    pub length: u16,
    pub typ: MessageType,
}

impl Decode for Header {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let length = u16::decode(decoder)?;
        let _typ = u8::decode(decoder)?;
        let typ = match _typ {
            0 => MessageType::ServerDetails,
            10 => MessageType::PlayerUpdate,
            11 => MessageType::PlayerAdd,
            12 => MessageType::PlayerRemove,
            _ => MessageType::Invalid,
        };
        Ok(Self {
            typ,
            length: length - 1,
        })
    }
}

impl_borrow_decode!(Header);

#[derive(Decode, Debug)]
pub struct Map {
    name: ZeroEndedString,
    gamemode: ZeroEndedString,
    layer: u8,
}

#[derive(Decode, Debug)]
pub struct ServerDetails {
    version: i32,
    demo_time_per_tick: f32,
    ip_port: ZeroEndedString,
    server_name: ZeroEndedString,
    max_players: u8,
    round_length: u16,
    briefing_time: u16,
    map: Map,
    blufor_team: ZeroEndedString,
    opfor_team: ZeroEndedString,
    start_time: Timestamp<u32>,
    tickets1: u16,
    tickets2: u16,
}

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

#[derive(Decode, Debug)]
pub struct Position {
    x: i16,
    y: i16,
    z: i16,
}

#[derive(Debug)]
pub struct PlayerUpdate {
    flags: u16,
    id: u8,
    team: i8,
    squad: u8,
    vehicle: PlayerVehicle,
    health: i8,
    score: i16,
    teamwork_score: i16,
    kills: i16,
    deaths: i16,
    ping: i16,
    is_alive: bool,
    is_joining: bool,
    position: Position,
    rotation: i16,
    kit_name: ZeroEndedString,
}

// TODO: add decode method for PlayerUpdate
