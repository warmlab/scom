use scom::SerialConnection;
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};
use std::fs::File;

use clap::{Parser, ValueEnum};

mod cli;
use cli::CommandLine;


fn main() -> Result<(), io::Error> {
    let cli = CommandLine::parse();

    // Establish a serial connection
    let mut connection: SerialConnection = SerialConnection::new(&cli.port, cli.baud.value())?;

    let mut output: Option<File> = if let Some(output_path) = cli.output {
        Some(File::create(output_path)?)
    } else {
        None
    };

    let mut input_data: Vec<u8> = if let Some(input_path) = cli.input {
        let mut file = File::open(input_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        buffer
    } else {
        Vec::new()
    };

    // Run loop for sending/receiving data
    let mut transmissions = 0;
    loop {
        // Input handling
        if input_data.is_empty() {
            print!("Enter data to send: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input_data = input.into_bytes();
        }

        // Send data
        connection.write_data(&input_data)?;

        // Receive data
        let mut buffer = [0; 1024];
        match connection.read_data(&mut buffer) {
            Ok(bytes_read) => {
                let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received: {}", response);

                if let Some(ref mut file) = output {
                    file.write_all(response.as_bytes())?;
                }
            }
            Err(e) => eprintln!("Error reading from serial port: {:?}", e),
        }

        // Handle count and loop options
        transmissions += 1;
        //if let Some(count_limit) = cli.count {
            if transmissions >= cli.count {
                break;
            }
        //}

        if !cli.to_loop {
            break;
        }

        // Handle interval between transmissions
        //if let Some(interval_duration) = cli.interval {
            std::thread::sleep(Duration::from_millis(cli.interval as u64));
        //}

        // Reset input for next loop if looping
        input_data.clear();
    }

    Ok(())
}
