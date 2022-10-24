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
            _ => MessageType::Invalid,
        };
        Ok(Header {
            typ,
            length: length - 1,
        })
    }
}

impl_borrow_decode!(Header);

#[derive(Decode, Debug)]
struct Map {
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
