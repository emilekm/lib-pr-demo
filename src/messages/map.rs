use bincode::Decode;

use crate::fields::{Timestamp, ZeroEndedString};

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

#[derive(Decode, Debug)]
pub struct Position {
    x: i16,
    y: i16,
    z: i16,
}

#[derive(Debug, Decode)]
pub struct CacheAdd {
    id: u8,
    position: Position,
}

#[derive(Debug, Decode)]
pub struct CacheRemove {
    id: u8,
}

#[derive(Debug, Decode)]
pub struct CacheReveal {
    id: u8,
}

#[derive(Debug, Decode)]
pub struct IntelChange {
    intel_count: i8,
}

#[derive(Debug, Decode)]
pub struct DoDList {
    team: u8,
    inverted: u8,
    dod_type: u8,
    num_points: u8,
    points_array: Vec<(f32, f32)>,
}

#[derive(Debug, Decode)]
pub struct Flags {
    cp_id: i16,
    owning_team: u8,
    position: Position,
    radius: u16,
}
