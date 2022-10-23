use bincode::de::{Decode, Decoder};
use bincode::error::DecodeError;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::marker::PhantomData;

pub struct ZeroEndedString(pub String);

impl Decode for ZeroEndedString {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let c = u8::decode(decoder)?;
        Ok(ZeroEndedString(String::from("")))
    }
}

pub struct Timestamp<T>(pub DateTime<Utc>, PhantomData<T>);

impl Decode for Timestamp<u32> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(1)?;
        let ui = u32::decode(decoder)?;

        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ui as i64, 0), Utc);
        Ok(Self(dt, PhantomData))
    }
}
