use bincode::Decode;

#[derive(Decode, Debug)]
pub struct Position {
    x: i16,
    y: i16,
    z: i16,
}

#[derive(Debug, Decode)]
pub struct CacheAdd {
    cache_id: u8,
    cache_pos: Position,
}

#[derive(Debug, Decode)]
pub struct CacheRemove {
    cache_id: u8,
}

#[derive(Debug, Decode)]
pub struct CacheReveal {
    cache_id: u8,
}

#[derive(Debug, Decode)]
pub struct IntelChange {
    new_intel_count: i8,
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
