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

impl Parity {
    pub fn as_serial_value(&self) -> serial2::Parity {
        match self { // TODO
            //Parity::None => serial2::Parity::None,
            Parity::Odd => serial2::Parity::Odd,
            Parity::Even => serial2::Parity::Even,
            //Parity::Mark => serial2::Parity::Mark,
            //Parity::Space => serial2::Parity::Space,
            _ => serial2::Parity::None, // Default to Even if not specified in the configuration file
        }
    }
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
