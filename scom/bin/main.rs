mod cli;

use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::fs::File;
use std::thread::{self, JoinHandle};
use std::sync::{mpsc, Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use scom::data_bits;
use signal_hook::flag;
use signal_hook::consts::SIGINT;

use clap::Parser;

use cli::CommandLine;
use scom::{HexString, data_format::DataFormat, SerialConnection};


fn main() -> Result<(), io::Error> {
    // parse command line arguments
    let cli = CommandLine::parse();

    if cli.to_list {
        let ll = SerialConnection::list_ports();
        match ll {
            Ok(ls) => {
                for pb in ls {
                    println!("{}", pb.display());
                }
            },
            Err(err) => {
                eprintln!("Error in getting available ports: {:?}", err);
            }
        }

        return Ok(());
    }

    let config = cli.to_config();
    // Establish a serial connection
    if config.port.port == None {
        eprintln!("Usage: {} --port <PORT>", env!("CARGO_PKG_NAME"));
        return Err(io::Error::new(io::ErrorKind::AddrNotAvailable, "do not supply the serial port"));
    }

    // Establish a serial connection
    let mut conn: SerialConnection = SerialConnection::new(&config.port.port.expect("Port is required!"),
                                                            config.port.baud.expect("No value").value(),
                                                            config.port.data_bits.expect("No value"),
                                                            config.port.stop_bits.expect("No value"),
                                                            config.port.parity.expect("No value"),
                                                            config.port.handshake.expect("No value")
                                                        )?; // TODO, need a default value for the baud rate
    let mut connection = Arc::new(Mutex::new(conn));
    // Create a shutdown flag
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    // Set up the signal handler for Ctrl+C
    flag::register(SIGINT, shutdown_flag.clone()).expect("Error setting signal handler");

    // start read thread
    let handler = read_thread(connection.clone(), &cli.output, &config.dataformat.output, shutdown_flag.clone());
    let mut lines: Vec<String> = Vec::new();

    // process data which would send
    if let Some(lns) = cli.data {
        for line in lns.lines() {
            lines.push(line.to_string());
        }
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
    let mut transmissions: usize = 0;
    //let mut input_lines: Vec<String> = Vec::new();

    while !shutdown_flag.load(Ordering::Relaxed) {
        // Input handling
        if lines.is_empty() {
            print!("Enter data to send: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            //input_lines.push(input);
            handle_line(connection.clone(), &input, &config.dataformat.input);
        } else {
            for line in lines.iter() {
                handle_line(connection.clone(), &line, &config.dataformat.input);
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

        if config.loops.to_loop != None && !config.loops.to_loop.expect("msg") {
            // Signal the read thread to stop
            shutdown_flag.store(true, Ordering::Relaxed);
            break;
        } else {
            (transmissions, _) = transmissions.overflowing_add(1);

            if transmissions & 0xF == 0 {
                println!("transmission times: [{}]", transmissions);
            }
            if let Some(count_limit) = config.loops.count {
                if transmissions >= count_limit {
                    break;
                }
            }
        }

        //if let Some(interval_duration) = config.interval {
        //}

        // Reset input for next loop if looping
        //input_lines.clear();

        if shutdown_flag.load(Ordering::Relaxed) {
            // Signal the read thread to stop
            shutdown_flag.store(true, Ordering::Relaxed);
        }
        // Handle interval between transmissions
        thread::sleep(Duration::from_millis(config.loops.interval.expect("")));
    }

    handler.join().unwrap();

    Ok(())
}

fn read_thread(connection: Arc<Mutex<SerialConnection>>,
               output_path: &Option<PathBuf>,
               output_format: &Option<DataFormat>,
               shutdown_flag: Arc<AtomicBool>, // Flag for stopping the thread
    ) -> JoinHandle<()> {

    // output file, result usually write to this
    let mut output = if let Some(output_path) = output_path {
        // Some(File::create(output_path)?)
        match File::create(output_path) {
            Ok(file) => Some(BufWriter::new(file)),
            Err(_) => None
        }
    } else {
        None
    };

    let fm = output_format.clone();
    let handler = thread::spawn(move || {
        // Buffer for receiving data from the serial connection
        let mut buffer = [0; 1024];
        // Loop until the shutdown_flag is set to true
        while !shutdown_flag.load(Ordering::Relaxed) {
            let mut conn = connection.lock().unwrap();
            match conn.read_data(&mut buffer) {
                Ok(bytes_read) => {
                    let received_data = &buffer[..bytes_read];

                    let output_str = match fm {
                        Some(DataFormat::HEX) => {
                            // Convert received data to hex if output format is hex
                            received_data.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>().join(" ")
                        },
                        _ => {
                            // Otherwise treat the received data as plain text
                            String::from_utf8_lossy(received_data).to_string()
                        }
                    };

                    // Optionally write the output to the file
                    if let Some(ref mut writer) = output {
                        if let Err(e) = writer.write_all(output_str.as_bytes()) {
                            eprintln!("Error writing to file: {}", e);
                        }
                        let _ = writer.flush(); // TODO
                    } else {
                        println!("< [{}]: {}", bytes_read, output_str);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from serial port: {:?}", e);
                    break;
                }
            }

            // Sleep for a small amount of time to avoid busy looping
            thread::sleep(Duration::from_millis(50));
        }

        //println!("Read thread shutting down gracefully.");
    });

    handler
}

fn handle_line(connection: Arc<Mutex<SerialConnection>>, line: &str, input_format: &Option<DataFormat>) {
    let hex;
    let data_to_send;
    // send data
    match input_format {
        //if *input_format == DataFormat::HEX {
        Some(DataFormat::HEX) => {
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
        },
        _ => {
            data_to_send = line.as_bytes();
        }
    }

    match connection.lock().unwrap().write_data(data_to_send) {
        Ok(bytes_write) => {
            if bytes_write > 0 {
                println!("> [{}]: {}", bytes_write, line);
            }
        }
        Err(e) => eprintln!("Error sending data to serial port: {:?}", e),
    }
}
