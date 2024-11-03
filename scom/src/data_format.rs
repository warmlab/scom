use serde::{Deserialize, Deserializer};

use clap::ValueEnum;
//use strum::{EnumIter, IntoEnumIterator};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Deserialize)]
pub enum DataFormat {
    ASCII = 1,
    UTF8 = 2,
    HEX = 4,
}

impl Default for DataFormat {
    fn default() -> Self {
        DataFormat::ASCII
    }
}

/*
impl<'a> Deserialize<'a> for DataFormat {
    fn deserialize<T>(deserializer: T) -> Result<Self, T::Error>
    where T: Deserializer<'a> {
        let value = u8::deserialize(deserializer)?;
        match value {
            1 => Ok(DataFormat::ASCII),
            2 => Ok(DataFormat::UTF8),
            4 => Ok(DataFormat::HEX),
            _ => Err(serde::de::Error::custom(format!("invalid data format value: {}", value))),
        }
    }
} */
