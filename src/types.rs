use crate::messages;

#[derive(Debug)]
pub enum Messages {
    Skip(messages::Header),

    ServerDetails(messages::ServerDetails),

    PlayerUpdate(messages::PlayersUpdate),
    PlayerAdd(messages::PlayersAdd),
    PlayerRemove(messages::PlayerRemove),

    VehicleUpdate(messages::VehiclesUpdate),
    VehicleAdd(messages::VehiclesAdd),
    VehicleDestroyed(messages::VehicleDestroyed),

    FobAdd(messages::FobsAdd),
    FobRemove(messages::FobsRemove),

    RallyAdd(messages::RallyAdd),
    RallyRemove(messages::RallyRemove),

    TicketsTeam1(messages::Tickets),
    TicketsTeam2(messages::Tickets),

    SquadName(messages::SquadName),
    SLOrder(messages::SLOrders),

    Kill(messages::Kill),
    Chat(messages::Chat),

    Revive(messages::Revive),
    KitAllocated(messages::KitAllocated),

    ProjAdd(messages::ProjAdd),
    ProjUpdate(messages::ProjUpdate),
    ProjRemove(messages::ProjRemove),

    CacheAdd(messages::CachesAdd),
    CacheRemove(messages::CacheRemove),
    CacheReveal(messages::CachesReveal),
    IntelChange(messages::IntelChange),

    DoDList(messages::DoDLists),
    Flags(messages::Flags),
}
