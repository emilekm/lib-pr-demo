use crate::messages;

#[derive(Debug)]
pub enum Messages {
    Invalid(messages::Header),

    ServerDetails(messages::ServerDetails),

    PlayerUpdate(messages::PlayerUpdate),
    PlayerAdd(messages::PlayerAdd),
    PlayerRemove(messages::PlayerRemove),

    VehicleUpdate(messages::VehicleUpdate),
    VehicleAdd(messages::VehicleAdd),
    VehicleDestroyed(messages::VehicleDestroyed),

    FobAdd(messages::FobAdd),
    FobRemove(messages::FobRemove),

    RallyAdd(messages::RallyAdd),
    RallyRemove(messages::RallyRemove),

    Tickets(messages::Tickets),

    SquadName(messages::SquadName),
    SLOrder(messages::SLOrder),

    Kill(messages::Kill),
    Chat(messages::Chat),

    Revive(messages::Revive),
    KitAllocated(messages::KitAllocated),

    ProjAdd(messages::ProjAdd),
    ProjUpdate(messages::ProjUpdate),
    ProjRemove(messages::ProjRemove),

    CacheAdd(messages::CacheAdd),
    CacheRemove(messages::CacheRemove),
    CacheReveal(messages::CacheReveal),
    IntelChange(messages::IntelChange),

    DoDList(messages::DoDList),
    Flags(messages::Flags),
}
