mod cli;

use scom::{DataFormat, SerialConnection};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::time::{Duration, Instant};
use std::fs::File;

use clap::Parser;

use cli::CommandLine;
use scom::HexString;


fn main() -> Result<(), io::Error> {
    // parse command line arguments
    let cli = CommandLine::parse();

    println!("{}", cli.baud.value());
    // Establish a serial connection
    let mut connection: SerialConnection = SerialConnection::new(&cli.port, cli.baud.value())?;

    let mut lines: Vec<String> = Vec::new();

    // process data which would send
    if let Some(lns) = cli.data {
        for line in lns.lines() {
            lines.push(line.to_string());
        }
    };

    // output file, result usually write to this
    let mut output = if let Some(output_path) = cli.output {
        // Some(File::create(output_path)?)
        match File::create(output_path) {
            Ok(file) => Some(BufWriter::new(file)),
            Err(_) => None
        }
    } else {
        None
    };

    if let Some(input_path) = cli.input {
        let file = File::open(input_path)?;
        let reader = BufReader::new(file);
        //file.read_to_end(&mut buffer)?;
        //buffer
        for line in reader.lines() {
            match line {
                Ok(l) => lines.push(l),
                Err(_) => println!("Error occurred when reading input file")
            }
        }
    };

    // Run loop for sending/receiving data
    let mut transmissions: u32 = 0;
    //let mut input_lines: Vec<String> = Vec::new();


    loop {
        // Input handling
        if lines.is_empty() {
            print!("Enter data to send: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            //input_lines.push(input);
            handle_line(&mut connection, &input, &mut output, &cli.input_format, &cli.output_format);
        } else {
            for line in lines.iter() {
                handle_line(&mut connection, &line, &mut output, &cli.input_format, &cli.output_format);
            }
        }

        // Receive data
        //let mut buffer: [u8; 1024] = [0; 1024];
        //match connection.read_data(&mut buffer) {
        //    Ok(bytes_read) => {
        //        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        //        println!("Received: {}", response);

        //        if let Some(ref mut file) = output {
        //            file.write_all(response.as_bytes())?;
        //        }
        //    }
        //    Err(e) => eprintln!("Error reading from serial port: {:?}", e),
        //}

        // Handle count and loop options
        //}

        if !cli.to_loop {
            break;
        } else {
            (transmissions, _) = transmissions.overflowing_add(1);

            if transmissions & 0xF == 0 {
                println!("transmission times: [{}]", transmissions);
            }
            if let Some(count_limit) = cli.count {
                if transmissions >= count_limit {
                    break;
                }
            }
        }

        //if let Some(interval_duration) = cli.interval {
        //}

        // Reset input for next loop if looping
        //input_lines.clear();

        // Handle interval between transmissions
        std::thread::sleep(Duration::from_millis(cli.interval as u64));
    }

    Ok(())
}

fn handle_line(connection: &mut SerialConnection, line: &str, output: &mut Option<BufWriter<File>>, input_format: &DataFormat, output_format: &DataFormat) {
    let hex;
    let data_to_send;
    // send data
    if *input_format == DataFormat::HEX {
        data_to_send = match String::from_hex(line) {
            Ok(r) => {
                hex = r;
                hex.as_slice()
            },
            Err(err) => {
                eprintln!("error in parse vector bytes to array: {}", err);
                b""
            }
        };
    } else {
        data_to_send = line.as_bytes();
    }

    match connection.write_data(data_to_send) {
        Ok(bytes_write) => {
            println!("> [{}]: {}", bytes_write, line);
        }
        Err(e) => eprintln!("error sending data to serial port: {:?}", e),
    }

    // Buffer for receiving data from the serial connection
    let mut buffer = [0; 1024];
    match connection.read_data(&mut buffer) {
        Ok(bytes_read) => {
            let received_data = &buffer[..bytes_read];

            let output_str = if *output_format == DataFormat::HEX {
                // Convert received data to hex if output format is hex
                received_data.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>().join(" ")
            } else {
                // Otherwise treat the received data as plain text
                String::from_utf8_lossy(received_data).to_string()
            };

            println!("< [{}]: {}", bytes_read, output_str);

            // Optionally write the output to the file
            if let Some(ref mut writer) = output {
                if let Err(e) = writer.write_all(output_str.as_bytes()) {
                    eprintln!("Error writing to file: {}", e);
                }
            }
        }
        Err(e) => eprintln!("Error reading from serial port: {:?}", e),
    }
}
