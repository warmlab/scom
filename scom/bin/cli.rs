use clap::{Parser, ValueEnum};
//use clap::{Arg, Command};

use std::path::PathBuf;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum BitMode {
    /// force 7bit mode
    bit7,

    /// force 8bit mode
    bit8,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
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

    #[arg(short='x', long)]
    pub hex: bool,

     /// turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[arg(short, long, value_enum)]
    pub mode: BitMode
}

/*
impl CommandLine {
    // Define command-line options using `clap`
    /*
     * Explanation of the CLI Options

     * --baud/-b: Specifies the baud rate for the serial communication.
     * --help/-h: Displays the help message.
     */

    let matches = Command::new("Serial Communication Tool")
        .version("1.0")
        .author("Xusheng")
        .about("A tool for serial communication over serial port")
        .arg(
            Arg::new("baud")
                .short('b')
                .long("baud")
                .value_name("BAUD")
                .help("Sets the baud rate")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Sets the serial port (e.g., /dev/ttyS1)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("interval")
                .long("interval")
                .value_name("INTERVAL")
                .help("Sets the interval between transmissions in milliseconds")
                .takes_value(true),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .help("Sets the number of transmissions")
                .takes_value(true),
        )
        .arg(
            Arg::new("loop")
                .short('l')
                .long("loop")
                .help("Run in a continuous loop"),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("INPUT")
                .help("Input file for data to send")
                .takes_value(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("Output file to store received data")
                .takes_value(true),
        )
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .help("Prints help information"),
        )
        .get_matches();

    // Parse the command-line arguments
    let baud_rate: u32 = matches
        .value_of("baud")
        .unwrap()
        .parse()
        .expect("Invalid baud rate");

    let port_name = matches.value_of("port").unwrap();
    let interval = matches
        .value_of("interval")
        .map(|v| v.parse::<u64>().expect("Invalid interval"));
    let count = matches
        .value_of("count")
        .map(|v| v.parse::<usize>().expect("Invalid count"));

    let input_file = matches.value_of("input");
    let output_file = matches.value_of("output");

    let loop_mode = matches.is_present("loop");
}
*/