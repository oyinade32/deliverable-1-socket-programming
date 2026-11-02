use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() {
    let address = "0.0.0.0:5050";
    let server_name = "Server of Oyinade Olukoya";
    let server_number: i32 = 10;

    let listener = TcpListener::bind(address)
        .expect("Failed to bind server to address");

    println!("{server_name} is listening on {address}.");

    loop {
        println!("Waiting for a client connection...");

        let (mut stream, client_address) = match listener.accept() {
            Ok(connection) => connection,
            Err(error) => {
                eprintln!("Failed to accept connection: {error}");
                continue;
            }
        };

        println!("Connection accepted from {client_address}.");

        let mut message = String::new();

        let read_result = {
            let mut reader = BufReader::new(&mut stream);
            reader.read_line(&mut message)
        };

        match read_result {
            Ok(0) => {
                println!("Client disconnected without sending a message.");
                continue;
            }
            Ok(_) => {}
            Err(error) => {
                eprintln!("Failed to read client message: {error}");
                continue;
            }
        }

        let message = message.trim();
        println!("Received message: {message}");

        let (client_name, number_text) = match message.split_once('|') {
            Some(parts) => parts,
            None => {
                println!("Invalid message format; expected name|number.");
                continue;
            }
        };

        let client_number: i32 = match number_text.parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid integer received; connection will close.");
                continue;
            }
        };

        println!("Client name: {client_name}");
        println!("Server name: {server_name}");
        println!("Client number: {client_number}");

        if !(1..=100).contains(&client_number) {
            println!("Number is outside 1–100. Shutting down server.");
            break;
        }

        let sum = client_number + server_number;

        println!("Server number: {server_number}");
        println!("Sum: {client_number} + {server_number} = {sum}");

        let reply = format!("{server_name}|{server_number}\n");

        match stream.write_all(reply.as_bytes()) {
            Ok(()) => println!("Server reply sent to client."),
            Err(error) => eprintln!("Failed to send reply: {error}"),
        }

        println!("Client connection finished.");
    }

    println!("Server has shut down.");
}