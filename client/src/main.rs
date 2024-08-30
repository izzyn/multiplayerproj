use core::time;
use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::thread::{sleep, sleep_ms};
use std::time::Duration;

fn main() -> std::io::Result<()> { 
    println!("Connecting to server...");
    let mut stream : TcpStream; 
    loop{

        stream = match TcpStream::connect_timeout(&SocketAddr::new(IpAddr::from(Ipv4Addr::from_str("127.0.0.1").expect("Incorrect IP adress was given")), 34254), Duration::new(5, 5)) {
            Err(err) => {
                println!("Failed to connect");
                println!("Retrying!");
                continue;
            },
            Ok(i) => i,
        };
        break;
    }

    println!("Established connection");
    loop{
        stream.write(&[1])?;
        sleep_ms(1000);
        stream.read(&mut [0; 128])?;
    }
    Ok(())
} // the stream is closed here
