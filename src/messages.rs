use crate::fields::{Timestamp, ZeroEndedString};
use bincode::Decode;

#[derive(Decode)]
struct MessageLength(u16);

#[derive(Decode)]
struct MessageType(u8);

#[derive(Decode)]
struct Map {
    name: ZeroEndedString,
    gamemode: ZeroEndedString,
    layer: u8,
}

#[derive(Decode)]
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
