use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

//use strum::IntoEnumIterator;
//use strum::EnumIter;
//
//use clap::ValueEnum;
use serde::Deserialize;
use toml;

use crate::baud_rate::BaudRate;
use crate::data_bit::BitMode;
use crate::parity::Parity;
use crate::stop_bit::StopBit;
use crate::handshake::Handshake;
use crate::data_format::DataFormat;

#[derive(Debug, Deserialize)]
pub struct PortConfig {
    pub port: Option<String>,
    pub baud: Option<BaudRate>,
    pub data_bits: Option<BitMode>,
    pub parity: Option<Parity>,
    pub stop_bits: Option<StopBit>,
    pub handshake: Option<Handshake>,

    #[serde(flatten)]
    pub extra: HashMap<String, toml::Value>, // Capture unexpected fields
}

#[derive(Debug, Deserialize)]
pub struct LoopsConfig {
    pub to_loop: Option<bool>,
    pub interval: Option<u64>,
    pub count: Option<usize>,
    pub timeout: Option<u64>,

    #[serde(flatten)]
    pub extra: HashMap<String, toml::Value>, // Capture unexpected fields
}

#[derive(Debug, Deserialize)]
pub struct DataFormatConfig {
    pub input: Option<DataFormat>,
    pub output: Option<DataFormat>,

    #[serde(flatten)]
    pub extra: HashMap<String, toml::Value>, // Capture unexpected fields
}

#[derive(Debug, Deserialize)]
pub struct Config {
    /*pub port: String,
    pub baud: BaudRate,
    pub data_bits: BitMode,
    pub parity: Parity,
    pub stop_bits: StopBit,
    pub handshake: Handshake,
    pub interval: u64,
    pub timeout: u64,
    pub input_format: DataFormat,
    pub output_format: DataFormat,
    pub count: usize,
    */
    pub port: PortConfig,
    pub loops: LoopsConfig,
    pub dataformat: DataFormatConfig,

    #[serde(flatten)]
    pub extra_sections: HashMap<String, toml::Value>, // Capture unexpected sections
}

impl Config {
    pub fn new() -> Self {
        Config {
            port: PortConfig {
                port: Some("/dev/ttyUSB0".to_string()),
                baud: Some(BaudRate::b115200),
                data_bits: Some(BitMode::Bit8),
                parity: Some(Parity::None),
                stop_bits: Some(StopBit::None),
                handshake: Some(Handshake::None),
                extra: HashMap::new(),

            },
            loops: LoopsConfig {
                to_loop: Some(false),
                interval: Some(1000), // unit millisecond
                timeout: Some(1000), // unit second TODO 
                count: Some(0),
                extra: HashMap::new(),
            },

            dataformat: DataFormatConfig {
                input: Some(DataFormat::ASCII),
                output: Some(DataFormat::ASCII),
                extra: HashMap::new()
            },

            extra_sections: HashMap::new()
        }
    }

    pub fn load(config_file: &PathBuf) -> Option<Config> {
        // read configure file
        //if let Some(config_file) = filename {
            let mut file = File::open(config_file).expect("configure file was not found");
            let mut content: String = String::new();
            //let mut buf = Vec::new();
            //let reader = BufReader::new(file);
            //let _size = reader.buffer(); //.read_to_ing(&mut buf);
            return match file.read_to_string(&mut content) {
                Ok(_) => {
                    //let config: Result<HashMap<String, String>, toml::de::Error> = toml::from_str(&content);
                    let config: Result<Config, toml::de::Error> = toml::from_str(&content);
                    match config {
                        Ok(c) => {
                            println!("{:?}", c);
                            Some(Config::new())
                        },
                        Err(err) => {
                            println!("error loading config file: {}", err);
                            None
                        }
                    }
                },
                Err(_) => None
            };
        //}

        //None
    }
}
