use bincode::de::{Decode, Decoder};
use bincode::error::DecodeError;
use bincode::impl_borrow_decode;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::marker::PhantomData;

pub struct ZeroEndedString(pub String);

impl Decode for ZeroEndedString {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let mut s = String::new();

        loop {
            let c = u8::decode(decoder)?;
            if c == 0 {
                break;
            }

            s.push(c as char)
        }

        Ok(ZeroEndedString(s))
    }
}

impl_borrow_decode!(ZeroEndedString);

pub struct Timestamp<T>(pub DateTime<Utc>, PhantomData<T>);

impl Decode for Timestamp<u32> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let timestamp = u32::decode(decoder)?;

        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp as i64, 0), Utc);
        Ok(Self(dt, PhantomData))
    }
}

impl_borrow_decode!(Timestamp<u32>);
