use std::{net::UdpSocket, time::Instant};
use ctrlc;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    {
        let socket = UdpSocket::bind(&args[1])?;

        let mut buf = Vec::new();
        buf.resize(args[2].parse::<usize>().unwrap(), 0);

        let mut bytes_received = 0;
        let mut instant = Instant::now();

        // handle Ctrl+C signal
        let running = std::sync::Arc::new(std::sync::Mutex::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            let mut running = r.lock().unwrap();
            *running = false;
        })
        .expect("Error setting Ctrl+C handler");

        while *running.lock().unwrap() {
            let (amount, _) = socket.recv_from(&mut buf)?;
            bytes_received += amount;

            if instant.elapsed().as_secs_f64() > 1.0 {
                println!("{} MBit/s", 8.0 * bytes_received as f64 / 1000.0 / 1000.0);
                bytes_received = 0;
                instant = Instant::now();
            }
        }
    }

    println!("Client stopped by user.");

    Ok(())
}
