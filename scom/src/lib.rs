pub mod config;
pub mod hexstring;
pub mod baud_rate;
pub mod data_bits;
pub mod parity;
pub mod stop_bits;
pub mod handshake;
pub mod data_format;

pub use config::Config;
use data_bits::BitMode;
use handshake::Handshake;
pub use hexstring::HexString;

use parity::Parity;
use serial2::{SerialPort, Settings};
use stop_bits::StopBits;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::time::Duration;

//extern crate libc;

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::fs;
//use std::io::{self, Write};

trait SerialPortSettings {
    fn as_serial_value<T>(&self) -> T;
}

pub struct SerialConnection {
    port: SerialPort,
}

impl SerialConnection {
    pub fn list_ports() -> std::io::Result<Vec<PathBuf>> {
        SerialPort::available_ports()
    }

    pub fn new(port_name: &str, baud_rate: u32, data_bit: BitMode, stop_bit: StopBits, parity: Parity, handshake: Handshake) -> Result<Self, io::Error> {

        let mut port = SerialPort::open(port_name, |mut settings: Settings | {
            settings.set_raw();
            settings.set_baud_rate(baud_rate)?;
            settings.set_char_size(BitMode::Bit8.as_serial_value());
            settings.set_stop_bits(StopBits::None.as_serial_value());
            settings.set_parity(Parity::None.as_serial_value());
            //settings.set_flow_control(FlowControl::RtsCts);
            Ok(settings)
        })?;
        port.set_read_timeout(Duration::from_secs(1))?;
        port.set_write_timeout(Duration::from_secs(1))?;
        Ok(SerialConnection { port })
    }

    pub fn write_lines(&mut self, lines: &Vec<String>) -> Result<usize, io::Error> {
        let mut total: usize = 0;

        for line in lines {
            match self.write_data(&line.as_bytes()) { // into_bytes
                Ok(c) => total += c,
                Err(e) => return Err(e)
            }
        }

        Ok(total)
    }

    pub fn write_data(&mut self, data: &[u8]) -> Result<usize, io::Error> {
        self.port.write(data)
    }

    pub fn read_data(&mut self, buffer: &mut [u8]) -> Result<usize, io::Error> {
        self.port.read(buffer)
    }
}
