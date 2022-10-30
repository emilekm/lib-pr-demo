#[derive(Debug)]
pub enum MessageType {
    Invalid = -1,

    ServerDetails = 0,
    PlayerUpdate = 10,
    PlayerAdd = 11,
    PlayerRemove = 12,
}
