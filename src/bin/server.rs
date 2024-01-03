use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use ctrlc;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    {
        let socket = UdpSocket::bind(&args[1])?;
        socket.connect(&args[2])?;

        let mut buf = Vec::new();
        buf.resize(args[3].parse::<usize>().unwrap(), 0);

        // handle sinyal Ctrl+C
        let running = Arc::new(Mutex::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            let mut running = r.lock().unwrap();
            *running = false;
        })
        .expect("Error setting Ctrl+C handler");

        while *running.lock().unwrap() {
            socket.send(&buf)?;
        }
    }

    println!("Server stopped by user.");

    Ok(())
}
