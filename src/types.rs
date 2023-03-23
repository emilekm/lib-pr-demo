use crate::messages;

pub const SERVER_DETAILS: u8 = 0x00;
pub const DOD_LIST: u8 = 0x01;

pub const PLAYER_UPDATE: u8 = 0x10;
pub const PLAYER_ADD: u8 = 0x11;
pub const PLAYER_REMOVE: u8 = 0x12;

pub const VEHICLE_UPDATE: u8 = 0x20;
pub const VEHICLE_ADD: u8 = 0x21;
pub const VEHICLE_DESTROYED: u8 = 0x22;

pub const FOB_ADD: u8 = 0x30;
pub const FOB_REMOVE: u8 = 0x31;

pub const FLAG_UPDATE: u8 = 0x40;
pub const FLAG_LIST: u8 = 0x41;

pub const KILL: u8 = 0x50;
pub const CHAT: u8 = 0x51;

pub const TICKETS_TEAM1: u8 = 0x52;
pub const TICKETS_TEAM2: u8 = 0x53;

pub const RALLY_ADD: u8 = 0x60;
pub const RALLY_REMOVE: u8 = 0x61;

pub const CACHE_ADD: u8 = 0x70;
pub const CACHE_REMOVE: u8 = 0x71;
pub const CACHE_REVEAL: u8 = 0x72;
pub const INTEL_CHANGE: u8 = 0x73;

pub const MARKER_ADD: u8 = 0x80;
pub const MARKER_REMOVE: u8 = 0x81;

pub const PROJ_UPDATE: u8 = 0x90;
pub const PROJ_ADD: u8 = 0x91;
pub const PROJ_REMOVE: u8 = 0x92;

pub const REVIVE: u8 = 0xA0;
pub const KITALLOCATED: u8 = 0xA1;
pub const SQUADNAME: u8 = 0xA2;
pub const SLORDERS: u8 = 0xA3;

pub const ROUNDEND: u8 = 0xF0;
pub const TICKS: u8 = 0xF1;

pub const PRIVATEMESSAGE: u8 = 0xFD;
pub const ERRORMESSAGE: u8 = 0xFE;

#[derive(Debug)]
pub enum Messages {
    Invalid(messages::Header),

    ServerDetails(messages::ServerDetails),

    PlayerUpdate(messages::PlayerUpdate),
    PlayerAdd(messages::PlayerAdd),
    PlayerRemove(messages::PlayerRemove),

    VehicleUpdate(messages::VehicleUpdate),
}
