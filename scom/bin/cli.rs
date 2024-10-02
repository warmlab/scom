use clap::{Parser, ValueEnum};
//use clap::{Arg, Command};

use std::path::PathBuf;

use scom::{BaudRate, BitMode, DataFormat};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLine {
     /// if present, the tool will continue sending data in a loop.
    #[arg(short = 'l', long="list")]
    pub to_list: bool,

    /// configuration file to use (NOT IMPLEMENT!!! TODO)
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// specifies the baud rate for the serial communication.
    #[arg(short, long, value_enum, default_value_t=BaudRate::b115200)]
    pub baud: BaudRate,

    /// specifies the serial port (e.g., /dev/ttyS1 or COM1), (ignore the value from config).
    #[arg(short, long)]
    pub port: Option<String>,

    /// interval: Sets an interval between transmissions in milliseconds.
    #[arg(short = 't', long, default_value_t=1000)]
    pub interval: u32,

    /// specifies data to send
    #[arg(short, long)]
    pub data: Option<String>,

    // specifies input data format, (ascii/utf8/hex)
    #[arg(short='I', long, value_enum, default_value_t=DataFormat::ASCII)]
    pub input_format: DataFormat,

    // specifies output data format, (ascii/utf8/hex)
    #[arg(short='O', long, value_enum, default_value_t=DataFormat::ASCII)]
    pub output_format: DataFormat,

    /// specifies the number of transmissions to send before stopping.
    #[arg(short='n', long)]
    pub count: Option<u32>,

     /// if present, the tool will continue sending data in a loop.
    #[arg(short = 'L', long="loop")]
    pub to_loop: bool,

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
    #[arg(short, long, value_enum, default_value_t=BitMode::bit8)]
    pub mode: BitMode
}
