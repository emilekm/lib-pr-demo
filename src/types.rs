use crate::messages;

pub const SERVER_DETAILS: u8 = 0;
pub const DOD_LIST: u8 = 1;

pub const PLAYER_UPDATE: u8 = 16;
pub const PLAYER_ADD: u8 = 17;
pub const PLAYER_REMOVE: u8 = 18;

pub const VEHICLE_UPDATE: u8 = 32;
pub const VEHICLE_ADD: u8 = 33;
pub const VEHICLE_DESTROYED: u8 = 34;

pub const FOB_ADD: u8 = 48;
pub const FOB_REMOVE: u8 = 49;

pub const FLAG_UPDATE: u8 = 64;
pub const FLAG_LIST: u8 = 65;

pub const KILL: u8 = 80;
pub const CHAT: u8 = 81;

pub const TICKETS_TEAM1: u8 = 82;
pub const TICKETS_TEAM2: u8 = 83;

pub const RALLY_ADD: u8 = 96;
pub const RALLY_REMOVE: u8 = 97;

pub const CACHE_ADD: u8 = 112;
pub const CACHE_REMOVE: u8 = 113;
pub const CACHE_REVEAL: u8 = 114;
pub const INTEL_CHANGE: u8 = 115;

pub const PROJ_UPDATE: u8 = 144;
pub const PROJ_ADD: u8 = 145;
pub const PROJ_REMOVE: u8 = 146;

pub const REVIVE: u8 = 160;
pub const KITALLOCATED: u8 = 161;
pub const SQUADNAME: u8 = 162;
pub const SLORDERS: u8 = 163;

pub const ROUNDEND: u8 = 240;
pub const TICKS: u8 = 241;

#[derive(Debug)]
pub enum Messages {
    Invalid(messages::Header),

    ServerDetails(messages::ServerDetails),

    PlayerUpdate(messages::PlayerUpdate),
    PlayerAdd(messages::PlayerAdd),
    PlayerRemove(messages::PlayerRemove),

    VehicleUpdate(messages::VehicleUpdate),
}
