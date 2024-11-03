use serde::{Deserialize, Deserializer};

use clap::ValueEnum;
//use strum::{EnumIter, IntoEnumIterator};

#[repr(u8)]
//#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Deserialize, EnumIter)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum StopBit {
    None = 0,
    One = 1,
    Two = 2,
    OnePointFive = 3,
}

impl Default for StopBit {
    fn default() -> Self {
        StopBit::None
    }
}


impl<'a> Deserialize<'a> for StopBit {
    fn deserialize<T>(deserializer: T) -> Result<Self, T::Error>
    where T: Deserializer<'a> {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(StopBit::None),
            1 => Ok(StopBit::One),
            2 => Ok(StopBit::Two),
            3 => Ok(StopBit::OnePointFive),
            _ => Err(serde::de::Error::custom(format!("invalid stop bit value: {}", value))),
        }
    }
}
