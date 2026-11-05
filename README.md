# TCP Socket Programming in Rust

## Project Overview

This project implements a TCP client-server communication system using Rust's standard networking library (`std::net`).

The client accepts an integer from the user, sends the client's name and number to the server, receives the server's response, and calculates the sum of both numbers.

The server listens for incoming TCP connections, receives client information, validates the integer, calculates the sum using a predefined server number, and sends its name and number back to the client.

The project also demonstrates cross-language communication between a Rust server and a Python client over a remote TCP connection.

## Technologies Used

- Programming Language: Rust
- Additional Client Language:** Python (used for remote interoperability testing)
- Networking Protocol:TCP
- Rust Libraries: `std::net`, `std::io`
- Python Library:`socket`
- Development Environment:Visual Studio Code
- Operating Systems: macOS and Linux (Ubuntu)
- Private Networking:Tailscale

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

The Rust client:

1. Prompts the user to enter an integer between 1 and 100.
2. Validates the entered integer.
3. Establishes a TCP connection with the server.
4. Sends its name and integer to the server.
5. Waits for the server's response.
6. Extracts the server's name and integer.
7. Calculates and displays the sum.
8. Closes the connection when execution finishes.

### Server

The Rust server:

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
Client of Francis Ntim|25
```

**Server response:**

```text
Server of Oyinade Olukoya|10
```

Both programs communicate using TCP on port `5050`.

## Network Configuration

The Rust server listens on all available network interfaces:

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

For remote testing, both computers must be connected to the same Tailscale network, with appropriate network access.

## How to Run the Application

Make sure Rust and Cargo are installed.

### 1. Start the server

Open a terminal in the project directory and run:

```bash
cargo run --bin server
```

The server will display a listening message and wait for client connections.

### 2. Start the Rust client

Open a second terminal in the project directory and run:

```bash
cargo run --bin client
```

Enter an integer between 1 and 100 when prompted.

### 3. Remote testing with a Python client

The remote interoperability test was performed using:

- **Server:** Rust running on macOS.
- **Client:** Python running on Ubuntu Linux.
- **Network:** Tailscale.
- **Server Tailscale IP:** `100.114.182.117`.
- **Server Port:** `5050`.
- **Client:** Francis Ntim.

The Python client used the standard `socket` library to establish a TCP connection with the Rust server.

Run the Python client from its project directory using:

```bash
python3 client.py
```

The client connects to the Rust server using its Tailscale IP address and port.

## Example Test Results

**Remote Rust server output:**

```text
Server of Oyinade Olukoya is listening on 0.0.0.0:5050.
Waiting for a client connection...
Connection accepted from 100.69.50.127:45638.
Received message: Client of Francis Ntim|25
Client name: Client of Francis Ntim
Server name: Server of Oyinade Olukoya
Client number: 25
Server number: 10
Sum: 25 + 10 = 35
Server reply sent to client.
Client connection finished.
Waiting for a client connection...
```

**Expected calculation:**

```text
25 + 10 = 35
```

## Testing and Verification

The following tests have been performed:

- Successful local TCP communication between the Rust client and server.
- Successful transmission of client name and integer.
- Successful transmission of server name and integer.
- Correct calculation of the sum.
- Successful Tailscale connectivity between the Mac and Ubuntu computers.
- Successful remote TCP connection from the Ubuntu computer to the macOS server.
- Successful communication between a Python client and Rust server.
- Server termination when an out-of-range integer (`0`) is received.

The remote server-side test confirmed that the Rust server received Francis Ntim's client name and integer, calculated the sum, and successfully sent its reply.

## Additional Test Program

The project includes `shutdown_test.rs`, an additional test utility used to verify that the server terminates when it receives an out-of-range integer.

Run it using:

```bash
cargo run --bin shutdown_test
```

## Author

Oyinade Olukoya

Project: Programming Assignment 1 – Socket Programming