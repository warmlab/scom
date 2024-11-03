use serde::{Deserialize, Deserializer};

use clap::ValueEnum;
//use strum::{EnumIter, IntoEnumIterator};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum BitMode {
    /// transmits 7 data bits, useful for ASCII text but limited in terms of data range.
    Bit7,

    /// transmits 8 data bits, the more common mode today, allowing full byte transmission and more flexibility for binary and non-ASCII data.
    Bit8,
}

impl BitMode {
    pub fn as_serial_value(&self) -> serial2::CharSize {
        match self {
            BitMode::Bit7 => serial2::CharSize::Bits7,
            BitMode::Bit8 => serial2::CharSize::Bits8,
        }
    }
}

impl Default for BitMode {
    fn default() -> Self {
        BitMode::Bit8
    }
}

impl<'a> Deserialize<'a> for BitMode {
    fn deserialize<T>(deserializer: T) -> Result<Self, T::Error>
    where T: Deserializer<'a> {
        let value = u8::deserialize(deserializer)?;
        match value {
            7 => Ok(BitMode::Bit7),
            8 => Ok(BitMode::Bit8),
            _ => Err(serde::de::Error::custom(format!("invalid data bit value: {}", value))),
        }
    }
}
