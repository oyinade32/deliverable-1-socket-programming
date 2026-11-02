use std::io::Write;
use std::net::TcpStream;

fn main() {
    let address = "127.0.0.1:5050";

    println!("Connecting to server at {address}...");

    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Connected to server.");

            let test_message = "Client of Shutdown Test|0\n";

            match stream.write_all(test_message.as_bytes()) {
                Ok(()) => {
                    println!("Sent test number 0 to the server.");
                    println!("The server should shut down.");
                }
                Err(error) => {
                    eprintln!("Failed to send test message: {error}");
                }
            }
        }
        Err(error) => {
            eprintln!("Could not connect to server: {error}");
            eprintln!("Make sure the server is running first.");
        }
    }
}