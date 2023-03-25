use bincode::de::Decoder;
use bincode::error::DecodeError;
use bincode::{impl_borrow_decode, Decode};

pub mod map;
pub use map::*;

pub mod players;
pub use players::*;

pub mod teams;
pub use teams::*;

pub mod vehicles;
pub use vehicles::*;

macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<u8> for $name {
            type Error = ();

            fn try_from(v: u8) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as u8 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

back_to_enum! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum MessageType {
        ServerDetails = 0x00,
        DodList = 0x01,

        PlayerUpdate = 0x10,
        PlayerAdd = 0x11,
        PlayerRemove = 0x12,

        VehicleUpdate = 0x20,
        VehicleAdd = 0x21,
        VehicleDestroyed = 0x22,

        FobAdd = 0x30,
        FobRemove = 0x31,

        FlagUpdate = 0x40,
        FlagList = 0x41,

        Kill = 0x50,
        Chat = 0x51,

        TicketsTeam1 = 0x52,
        TicketsTeam2 = 0x53,

        RallyAdd = 0x60,
        RallyRemove = 0x61,

        CacheAdd = 0x70,
        CacheRemove = 0x71,
        CacheReveal = 0x72,
        IntelChange = 0x73,

        MarkerAdd = 0x80,
        MarkerRemove = 0x81,

        ProjUpdate = 0x90,
        ProjAdd = 0x91,
        ProjRemove = 0x92,

        Revive = 0xA0,
        KitAllocated = 0xA1,
        SquadName = 0xA2,
        SlOrders = 0xA3,

        RoundEnd = 0xF0,
        Ticks = 0xF1,

        PrivateMessage = 0xFD,
        ErrorMessage = 0xFE,
    }
}

pub type LengthSize = u16;

#[derive(Debug)]
pub struct Header {
    pub length: LengthSize,
    pub typ: MessageType,
}

impl Decode for Header {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let length = u16::decode(decoder)?;
        let typ_u8 = u8::decode(decoder)?;

        Ok(Self {
            length: length - 1, // substract the type size decoded bellow
            typ: typ_u8.try_into().unwrap(),
        })
    }
}

impl_borrow_decode!(Header);
