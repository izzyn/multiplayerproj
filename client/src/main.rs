use core::time;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::{sleep, sleep_ms};

fn main() -> std::io::Result<()> { 
    let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    loop{
        stream.write(&[1])?;
        sleep_ms(1000);
        stream.read(&mut [0; 128])?;
    }
    Ok(())
} // the stream is closed here
