mod cli;

use scom::SerialConnection;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::time::{Duration, Instant};
use std::fs::File;

use clap::Parser;

use cli::CommandLine;


fn main() -> Result<(), io::Error> {
    // parse command line arguments
    let cli = CommandLine::parse();

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
            handle_line(&mut connection, &input, &mut output);
        } else {
            for line in lines.iter() {
                handle_line(&mut connection, &line, &mut output);
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
        transmissions += 1;
        //if let Some(count_limit) = cli.count {
            if transmissions >= cli.count {
                break;
            }
        //}

        if !cli.to_loop {
            break;
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

fn handle_line(connection: &mut SerialConnection, line: &str, output: &mut Option<BufWriter<File>>) {
    // send data
    match connection.write_data(line.as_bytes()) {
        Ok(bytes_write) => {
            println!("data[{}] has sent to the port", bytes_write);
        }
        Err(e) => eprintln!("error sending data to serial port: {:?}", e),
    }

    // Receive data
    let mut buffer = [0; 1024];
    match connection.read_data(&mut buffer) {
        Ok(bytes_read) => {
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("received: {}", response);

            if let Some(ref mut writer) = output {
                let _ = writer.write_all(response.as_bytes()); // TODO
            }
        }
        Err(e) => eprintln!("error reading from serial port: {:?}", e),
    }
}
