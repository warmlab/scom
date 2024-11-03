use clap::{Command, Parser, ValueEnum};
//use clap::{Arg, Command};

use std::any::Any;
use std::env;
use std::path::Path;
use std::{path::PathBuf, time::Duration};

use scom::{baud_rate::BaudRate, data_format::DataFormat};

use scom::stop_bits::StopBits;
use scom::parity::Parity;
use scom::handshake::Handshake;
use scom::data_bits::BitMode;
use scom::config::Config;

const DEFAULT_CONFIG_NAME: &str = ".scom.config";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLine {
     /// if present, the tool will continue sending data in a loop.
    #[arg(short = 'l', long="list")]
    pub to_list: bool,

    /// configuration file to use (NOT IMPLEMENT!!! TODO)
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// specifies the serial port (e.g., /dev/ttyS1 or COM1), (ignore the value from config).
    #[arg(short, long)]
    pub port: Option<String>,

    /// specifies the baud rate for the serial communication.
    #[arg(short, long, value_enum)]
    pub baud: Option<BaudRate>,

    /// specifies the parity for the serial communication.
    #[arg(short = 'P', long, value_enum)]
    pub parity: Option<Parity>,

    /// specifies the data bit for the serial communication.
    #[arg(short = 'D', long, value_enum)]
    pub data_bits: Option<BitMode>,

    /// specifies the stop bit for the serial communication.
    #[arg(short, long, value_enum)]
    pub stop_bits: Option<StopBits>,

    /// specifies the handshake for the serial communication.
    #[arg(short = 'H', long, value_enum)]
    pub handshake: Option<Handshake>,

    /// interval: Sets an interval between transmissions in milliseconds.
    #[arg(short = 't', long)]
    pub interval: Option<u64>,

    /// specifies data to send
    #[arg(short, long)]
    pub data: Option<String>,

    // specifies input data format, (ascii/utf8/hex)
    #[arg(short='I', long, value_enum)]
    pub input_format: Option<DataFormat>,

    // specifies output data format, (ascii/utf8/hex)
    #[arg(short='O', long, value_enum)]
    pub output_format: Option<DataFormat>,

    /// specifies the number of transmissions to send before stopping.
    #[arg(short='n', long)]
    pub count: Option<usize>,

     /// if present, the tool will continue sending data in a loop.
    #[arg(short = 'L', long="loop")]
    pub to_loop: Option<bool>,

    /// specifies a file to read input data from for sending through the serial port.
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// specifies a file to write received data to.
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    //#[arg(short='x', long)]
    //pub hex: bool,

     /// turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// data is encoded for transmission over the serial line (NOT IMPLEMENT!!! TODO)
    #[arg(short, long, value_enum, default_value_t=BitMode::Bit8)]
    pub mode: BitMode
}

impl CommandLine {
    fn option_to_value<T>(&self, option_value: Option<T>) -> T where T: Default {
        match option_value {
            Some(value) => value,
            None => T::default(),
        }
    }

    pub fn to_config(&self) -> Config {
        let mut config: Config;

        // Load configuration file
        if self.config == None {
            let path = match env::current_dir() {
                Ok(p) => {
                    eprintln!("Failed to load configuration file: {}/{}", p.display(), DEFAULT_CONFIG_NAME);
                    p.join(DEFAULT_CONFIG_NAME)
                },
                Err(_) => {
                    eprintln!("Failed to get current directory");
                    PathBuf::new()
                }
            };
            let option_config: Option<Config> = Config::load(&path);
            config = match option_config {
                Some(c) => c,
                None => {
                    let path = env::var_os("HOME");
                    let path = match path {
                        Some(path) => {
                            let mut pathbuf = PathBuf::new();
                            pathbuf.push(path);
                            let path = pathbuf.join(DEFAULT_CONFIG_NAME);
                            //let path = PathBuf::new(path.into_string()).join(DEFAULT_CONFIG_NAME);
                            println!("Home Directory: {:?}", path);
                            path
                        },
                        None => {
                            eprintln!("Failed to get home directory");
                            PathBuf::new()
                        }
                    };
                    let option_config = Config::load(&path);
                    match option_config {
                        Some(c) => c,
                        None => {
                            //eprintln!("Failed to load configuration file: {}", path.display());
                            Config::new()
                        }
                    }
                }
            };
        } else {
            if let Some(path) = &self.config {
                config = match Config::load(&path) {
                    Some(c) => c,
                    None => {
                        //eprintln!("Failed to load configuration file: {}", path.display());
                        Config::new()
                    }
                }
            } else {
                eprintln!("Configuration file not found");
                config = Config::new()
            }
        }
        // apply configuration
        //println!("Using configuration: {:?}", config);
        if let Some(_) = self.port {
            config.port.port = self.port.clone();
        }

        if let Some(_) = self.baud {
            config.port.baud = self.baud;
        }

        if let Some(_) = self.data_bits {
            config.port.data_bits = self.data_bits;
        }

        if let Some(_) = self.stop_bits {
            config.port.stop_bits = self.stop_bits;
        }

        if let Some(_) = self.parity {
            config.port.parity = self.parity;
        }

        if let Some(_) = self.handshake {
            config.port.handshake = self.handshake;
        }

        if let Some(_) = self.to_loop {
            config.loops.to_loop = self.to_loop;
        }

        if let Some(_) = self.interval {
            config.loops.interval = self.interval;
        }

        if let Some(_) = self.count {
            config.loops.count = self.count;
        }

                //config.timeout = self.option_to_value(self.timeout);
        if let Some(_) = self.input_format {
            config.dataformat.input = self.input_format;
        }

        if let Some(_) = self.output_format {
            config.dataformat.output = self.output_format;
        }

        config
    }
}
