use serde::{Deserialize, Deserializer};

use clap::ValueEnum;
//use strum::{EnumIter, IntoEnumIterator};

#[repr(u8)]
//#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Deserialize, EnumIter)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum StopBits {
    None = 0,
    One = 1,
    Two = 2,
    OnePointFive = 3,
}

impl StopBits {
    pub fn as_serial_value(&self) -> serial2::StopBits {
        match self { // TODO
            //StopBits::None => serial2::StopBits::None,
            StopBits::One => serial2::StopBits::One,
            //StopBits::Two => serial2::StopBits::Two,
            //StopBits::OnePointFive => serial2::StopBits::OnePointFive,
            _ => serial2::StopBits::Two,
        }
    }
}

impl Default for StopBits {
    fn default() -> Self {
        StopBits::None
    }
}


impl<'a> Deserialize<'a> for StopBits {
    fn deserialize<T>(deserializer: T) -> Result<Self, T::Error>
    where T: Deserializer<'a> {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(StopBits::None),
            1 => Ok(StopBits::One),
            2 => Ok(StopBits::Two),
            3 => Ok(StopBits::OnePointFive),
            _ => Err(serde::de::Error::custom(format!("invalid stop bit value: {}", value))),
        }
    }
}
