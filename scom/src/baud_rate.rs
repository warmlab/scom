use serde::{Deserialize, Deserializer};

use clap::ValueEnum;
use strum::{EnumIter, IntoEnumIterator};

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, EnumIter)]
pub enum BaudRate {
    /// baud rate: 110
    b110 = 110,
    /// baud rate: 300
    b300 = 300,
    /// baud rate: 600
    b600 = 600,
    /// baud rate: 1200
    b1200 = 1200,
    /// baud rate: 2400
    b2400 = 2400,
    /// baud rate: 4800
    b4800 = 4800,
    /// baud rate: 9600
    b9600 = 9600,
    /// baud rate: 14400
    b14400 = 14400,
    /// baud rate: 19200
    b19200 = 19200,
    /// baud rate: 38400
    b38400 = 38400,
    /// baud rate: 57600
    b57600 = 57600,
    /// baud rate: 115200
    b115200 = 115200,
    /// baud rate: 128000
    b128000 = 128000,
    /// baud rate: 256000
    b256000 = 256000,
}

impl BaudRate {
    pub fn value(&self) -> u32 {
        *self as u32
    }

    pub fn values() -> Vec<(String, u32)> {
        let mut r = Vec::new();
        for rate in BaudRate::iter() {
            r.push((rate.value().to_string(), rate.value()));
        }

        r
    }
}

impl Default for BaudRate {
    fn default() -> Self {
        BaudRate::b115200
    }
}

impl<'de> Deserialize<'de> for BaudRate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let baud = u32::deserialize(deserializer)?;
        match baud {
            110 => Ok(BaudRate::b110),
            300 => Ok(BaudRate::b300),
            600 => Ok(BaudRate::b600),
            1200 => Ok(BaudRate::b1200),
            2400 => Ok(BaudRate::b2400),
            4800 => Ok(BaudRate::b4800),
            9600 => Ok(BaudRate::b9600),
            14400 => Ok(BaudRate::b14400),
            19200 => Ok(BaudRate::b19200),
            38400 => Ok(BaudRate::b38400),
            57600 => Ok(BaudRate::b57600),
            115200 => Ok(BaudRate::b115200),
            128000 => Ok(BaudRate::b128000),
            256000 => Ok(BaudRate::b256000),
            _ => Err(serde::de::Error::custom(format!("invalid baud rate: {}", baud))),
        }
    }
}
