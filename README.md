# TCP Socket Programming in Rust

## Project Overview

This project implements a TCP client-server communication system using Rust's standard networking library (`std::net`).

The client accepts an integer from the user, sends the client's name and number to the server, receives the server's response, and calculates the sum of both numbers.

The server listens for incoming TCP connections, receives client information, generates a response using a predefined integer, calculates the sum, and sends its name and number back to the client.

## Technologies Used

- Programming Language: Rust
- Networking Protocol: TCP
- Rust Libraries: `std::net`, `std::io`
- Development Environment: Visual Studio Code
- Operating System: macOS

## Project Structure

```text
deliverable-1-socket-programming/
│
├── Cargo.toml
├── Cargo.lock
├── README.md
│
└── src/
    ├── main.rs
    └── bin/
        ├── client.rs
        ├── server.rs
        └── shutdown_test.rs
```

## How the Application Works

### Client

The client program:

1. Prompts the user to enter an integer between 1 and 100.
2. Validates the entered integer.
3. Establishes a TCP connection with the server.
4. Sends its name and integer to the server.
5. Waits for the server's response.
6. Extracts the server's name and integer.
7. Calculates and displays the sum.
8. Closes the connection when execution finishes.

### Server

The server program:

1. Binds to port `5050` and listens for incoming TCP connections.
2. Accepts client connections.
3. Receives and extracts the client's name and integer.
4. Checks whether the received integer is within the range of 1–100.
5. Uses a predefined server integer of `10`.
6. Calculates and displays the sum.
7. Sends its name and integer back to the client.
8. Closes the client connection and waits for another connection.

If the server receives an integer outside the permitted range, it terminates.

## Communication Protocol

The client and server exchange newline-delimited messages using a pipe (`|`) as the separator.

**Client message:**

```text
Client of Oyinade Olukoya|25
```

**Server response:**

```text
Server of Oyinade Olukoya|10
```

The client and server use TCP communication on port `5050`.

## Configuration

The server listens on all available network interfaces:

```rust
let address = "0.0.0.0:5050";
```

The client connects using the server's Tailscale IP address:

```rust
let address = "100.114.182.117:5050";
```

For local testing, the client can instead use:

```rust
let address = "127.0.0.1:5050";
```

For remote testing, the client must use the server's reachable Tailscale IP address, and both devices must have an active Tailscale connection with appropriate network access.

## How to Run the Application

Make sure Rust and Cargo are installed.

### 1. Start the server

Open a terminal in the project directory and run:

```bash
cargo run --bin server
```

The server will display a listening message and wait for client connections.

### 2. Start the client

Open a second terminal in the same project directory and run:

```bash
cargo run --bin client
```

Enter an integer between 1 and 100 when prompted.

### 3. Example Output

**Client output:**

```text
Enter an integer between 1 and 100:
25
Client name: Client of Oyinade Olukoya
Client number: 25
Connecting to server...
TCP connection established.
Sent client name and number to server.
Server name: Server of Oyinade Olukoya
Client name: Client of Oyinade Olukoya
Client number: 25
Server number: 10
Sum: 25 + 10 = 35
Client has finished.
```

**Server output:**

```text
Server of Oyinade Olukoya is listening on 0.0.0.0:5050.
Waiting for a client connection...
Received message: Client of Oyinade Olukoya|25
Client name: Client of Oyinade Olukoya
Server name: Server of Oyinade Olukoya
Client number: 25
Server number: 10
Sum: 25 + 10 = 35
Server reply sent to client.
Client connection finished.
```

## Testing

The following tests have been performed:

- Successful TCP connection between the client and server.
- Successful transmission of the client's name and integer.
- Successful transmission of the server's name and integer.
- Correct calculation of the sum.
- Successful local communication using the Tailscale IP address.
- Server termination when an out-of-range integer (`0`) is received.

**Remote testing with another student's computer:** Pending.

## Additional Test Program

The project includes `shutdown_test.rs`, an additional test utility used to verify that the server terminates when it receives an out-of-range integer.

Run it using:

```bash
cargo run --bin shutdown_test
```

## Author

Oyinade Olukoya

Project: Programming Assignment 1 – Socket Programming