use crate::fields::{Timestamp, ZeroEndedString};
use bincode::Decode;

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
    mapsize: f32,
}
