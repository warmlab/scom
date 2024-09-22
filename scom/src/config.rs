use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;

use clap::ValueEnum;
use toml::{self, de};
use serde::Deserialize;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DataFormat {
    ASCII = 1,
    UTF8 = 2,
    HEX = 4,
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum BitMode {
    /// transmits 7 data bits, useful for ASCII text but limited in terms of data range.
    bit7,

    /// transmits 8 data bits, the more common mode today, allowing full byte transmission and more flexibility for binary and non-ASCII data.
    bit8,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Deserialize)]
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
}

#[derive(Deserialize)]
pub struct Config {
    port: String,
    baud_rate: BaudRate,
}

impl Config {
    pub fn new() -> Self {
        Config {
            port: "".to_string(),
            baud_rate: BaudRate::b115200
        }
    }

    pub fn load(filename: &Option<PathBuf>) -> Option<Self> {
        // read configure file
        if let Some(config_file) = filename {
            let file = File::open(config_file).expect("configure file was not found");
            //let mut buf = Vec::new();
            let reader = BufReader::new(file);
            //let _size = reader.buffer(); //.read_to_ing(&mut buf);
            let config: Result<Config, de::Error> = toml::from_slice(reader.buffer());
            match config {
                Ok(c) => Some(c),
                Err(_) => None
            };
        }

        None
    }
}
