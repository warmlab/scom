mod config;
mod hexstring;

pub use config::{Config, BitMode, BaudRate, DataFormat};
pub use hexstring::HexString;

use serial2::SerialPort;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::time::Duration;


pub struct SerialConnection {
    port: SerialPort,
}

impl SerialConnection {
    pub fn list_ports() -> std::io::Result<Vec<PathBuf>> {
        SerialPort::available_ports()
    }

    pub fn new(port_name: &str, baud_rate: u32) -> Result<Self, io::Error> {
        let mut port = SerialPort::open(port_name, baud_rate)?;
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
