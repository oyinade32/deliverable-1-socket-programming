use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    let address = "100.114.182.117:5050";
    let client_name = "Client of Oyinade Olukoya";

    println!("Enter an integer between 1 and 100:");

    let mut input = String::new();

    if let Err(error) = io::stdin().read_line(&mut input) {
        eprintln!("Failed to read input: {error}");
        return;
    }

    let number: i32 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid input. Please enter an integer.");
            return;
        }
    };

    if !(1..=100).contains(&number) {
        eprintln!("Please enter an integer between 1 and 100.");
        return;
    }

    println!("Client name: {client_name}");
    println!("Client number: {number}");
    println!("Connecting to server at {address}...");

    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("TCP connection established.");

            let message = format!("{client_name}|{number}\n");

            if let Err(error) = stream.write_all(message.as_bytes()) {
                eprintln!("Failed to send message: {error}");
                return;
            }

            println!("Sent client name and number to server.");

            let mut reader = BufReader::new(&mut stream);
            let mut reply = String::new();

            match reader.read_line(&mut reply) {
                Ok(0) => {
                    println!("Server closed the connection without replying.");
                }
                Ok(_) => {
                    let reply = reply.trim();

                    match reply.split_once('|') {
                        Some((server_name, server_number_text)) => {
                            match server_number_text.parse::<i32>() {
                                Ok(server_number) => {
                                    
                                    let sum = number + server_number;
                                    println!("Server name: {server_name}");
                                    println!("Client name: {client_name}");
                                    println!("Client number: {number}");
                                    println!("Server number: {server_number}");
                                    println!("Sum: {number} + {server_number} = {sum}");
                                }

                                Err(_) => {
                                    eprintln!("Invalid server number received.");
                                }
                            }
                        }
                        None => {
                            eprintln!("Invalid reply format received from server.");
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Failed to read server reply: {error}");
                }
            }
        }
        Err(error) => {
            eprintln!("Failed to connect to server: {error}");
        }
    }

    println!("Client has finished.");
}