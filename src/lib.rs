pub use bincode;
use bincode::config::Config;
use bincode::de::read::Reader;
use messages::Header;
use types::MessageType;

mod fields;
pub mod messages;
pub mod types;

pub fn next_message<R: Reader, C: Config>(
    reader: R,
    config: C,
) -> Result<Header, bincode::error::DecodeError> {
    let decoded: (u16, u8) = bincode::decode_from_reader(reader, config)?;
    let typ = match decoded.1 {
        0 => MessageType::ServerDetails,
        _ => MessageType::Invalid,
    };
    Ok(Header {
        typ,
        length: decoded.0 - 1,
    })
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
