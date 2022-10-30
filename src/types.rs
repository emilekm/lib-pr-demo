use crate::messages;

pub const SERVER_DETAILS: u8 = 0;

pub const PLAYER_UPDATE: u8 = 16;
pub const PLAYER_ADD: u8 = 17;
pub const PLAYER_REMOVE: u8 = 18;

pub const VEHICLE_UPDATE: u8 = 32;

#[derive(Debug)]
pub enum Messages {
    Invalid(messages::Header),

    ServerDetails(messages::ServerDetails),

    PlayerUpdate(messages::PlayerUpdate),
    PlayerAdd(messages::PlayerAdd),
    PlayerRemove(messages::PlayerRemove),

    VehicleUpdate(messages::VehicleUpdate),
}

