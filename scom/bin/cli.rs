use clap::{Parser, ValueEnum};
//use clap::{Arg, Command};

use std::path::PathBuf;

use scom::{BaudRate, BitMode, DataFormat};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLine {
    /// configuration file to use
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// specifies the baud rate for the serial communication.
    #[arg(short, long, value_enum, default_value_t=BaudRate::b115200)]
    pub baud: BaudRate,

    /// specifies the serial port (e.g., /dev/ttyS1 or COM1), (ignore the value from config).
    #[arg(short, long)]
    pub port: String,

    /// interval: Sets an interval between transmissions in milliseconds.
    #[arg(short = 't', long, default_value_t=1000)]
    pub interval: u32,

    /// specifies data to send
    #[arg(short, long)]
    pub data: Option<String>,

    // specifies input data format, (ascii/utf8/hex)
    #[arg(short='I', long)]
    pub input_format: DataFormat,

    // specifies output data format, (ascii/utf8/hex)
    #[arg(short='O', long)]
    pub output_format: DataFormat,

    /// specifies the number of transmissions to send before stopping.
    #[arg(short='n', long, default_value_t=1)]
    pub count: u32,

     /// if present, the tool will continue sending data in a loop.
    #[arg(short = 'l', long="loop")]
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

    #[arg(short, long, value_enum)]
    pub mode: BitMode
}
