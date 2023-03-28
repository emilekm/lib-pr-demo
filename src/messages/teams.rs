use bincode::Decode;
use derive::DecodeMultiple;

use crate::fields::ZeroEndedString;

use super::Position;

#[derive(Debug, Decode)]
pub struct FobAdd {
    id: i32,
    team: u8,
    position: Position,
}

#[derive(Debug, DecodeMultiple)]
pub struct FobsAdd {
    fobs: Vec<FobAdd>,
}

#[derive(Debug, Decode)]
pub struct FobRemove {
    id: i32,
}

#[derive(Debug, DecodeMultiple)]
pub struct FobsRemove {
    fobs: Vec<FobRemove>,
}

#[derive(Debug, Decode)]
pub struct RallyAdd {
    team_squad: u8,
    position: Position,
}

#[derive(Debug, Decode)]
pub struct RallyRemove {
    team_squad: u8,
}

#[derive(Debug, Decode)]
pub struct Tickets {
    tickets: i16,
}

#[derive(Debug, Decode)]
pub struct SquadName {
    team_squad: u8,
    squad_name: ZeroEndedString,
}

#[derive(Debug, Decode)]
pub struct SLOrder {
    team_squad: u8,
    order_type: u8,
    position: Position,
}

#[derive(Debug, DecodeMultiple)]
pub struct SLOrders {
    orders: Vec<SLOrder>,
}
