use serde::{Deserialize, Deserializer};

use clap::ValueEnum;
//use strum::{EnumIter, IntoEnumIterator};

#[repr(u8)]
//#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, EnumIter)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Handshake {
    None = 0,
    XOnXOff = 1,
    RequestToSend = 2,
    RequestToSendXOnXOff = 3,
}

impl Default for Handshake {
    fn default() -> Self {
        Handshake::None
    }
}

impl<'a> Deserialize<'a> for Handshake {
    fn deserialize<T>(deserializer: T) -> Result<Self, T::Error>
    where T: Deserializer<'a> {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(Handshake::None),
            1 => Ok(Handshake::XOnXOff),
            2 => Ok(Handshake::RequestToSend),
            3 => Ok(Handshake::RequestToSendXOnXOff),
            _ => Err(serde::de::Error::custom(format!("invalid handshake value: {}", value))),
        }
    }
}
