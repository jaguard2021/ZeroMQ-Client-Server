# ZeroMQ Client-Server Rust Project

## Project Description

This project is a simple implementation of a UDP Client-Server using the Rust programming language and ZeroMQ for communication. The client program measures the throughput of data transfer from the server, while the server program sends data to the client.

## Key Features

- **UDP Communication**: Using `UdpSocket` for communication between the client and server via the UDP protocol.
- **ZeroMQ Integration**: Utilizing the `zmq` crate to integrate ZeroMQ into the project.

## How to Run the Project

1. **Install Rust**: Make sure you have Rust and Cargo installed. If not, [download Rust](https://www.rust-lang.org/tools/install) and follow the installation instructions.

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/jaguard2021/ZeroMQ-Client-Server.git
   cd zmq-client-server
   ```
3. Build and Run the Client:  

   ```bash
   cd client
   cargo build --release
   ./target/release/client <server_address> <buffer_size>
   ```
4. Build and Run the Server:

   ```bash
   cd server
   cargo build --release
   ./target/release/server <server_address> <client_address> <buffer_size>
   ```
5. Stopping the Program: 
   
   To stop the program, press Ctrl+C in the terminal where the program is running.
