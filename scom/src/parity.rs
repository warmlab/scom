use serde::{Deserialize, Deserializer};

use clap::ValueEnum;
//use strum::{EnumIter, IntoEnumIterator};

#[repr(u32)]
//#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Deserialize, EnumIter)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Parity {
    None = 0,
    Odd = 1,
    Even = 2,
    Mark = 3,
    Space = 4
}

impl Default for Parity {
    fn default() -> Self {
        Parity::None
    }
}

impl<'a> Deserialize<'a> for Parity {
    fn deserialize<T>(deserializer: T) -> Result<Self, T::Error>
    where T: Deserializer<'a> {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(Parity::None),
            1 => Ok(Parity::Odd),
            2 => Ok(Parity::Even),
            3 => Ok(Parity::Mark),
            4 => Ok(Parity::Space),
            _ => Err(serde::de::Error::custom(format!("invalid parity value: {}", value))),
        }
    }
}
