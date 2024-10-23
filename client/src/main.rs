// src/bin/client.rs
use core::net;
use helper::*;
use shared_proc::{expand, netfunc};
use std::error::Error;
use std::io;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let _a = 3.1415;
    let length = 0b1111111111111111111111111;
    let server_addr = "127.0.0.1:1234";
    let stream: TcpStream = TcpStream::connect(server_addr).await.unwrap();
    println!("Connected to {}", stream.peer_addr().unwrap());
    send_request(stream).await.unwrap();
}

#[netfunc]
fn testa(a: &str, b: i32) {
    println!("{}{}", a, b);
}

async fn send_request(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        println!("Expecting input...");
        let mut close = false;
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer)?;
        match buffer.trim() {
            "close" => close = true,
            _ => {
                println!("Not real argument");
            }
        }
        loop {
            stream.writable().await?;
            println!("{:?}", &shared::data::encode_u64(32948));
            let parsed = shared::data::encode_u64(32948);
            let len = parsed.len();

            let data_ids = shared::data::DataIDs::ENDPKG as u8;
            let mut buff: [u8; 10] = [0; 10];
            let bufflen = buff.len();
            buff[..parsed.len()].copy_from_slice(&parsed);
            buff[len] = data_ids;

            match stream.try_write(&buff) {
                Ok(n) => {
                    println!("Wrote {} bytes", n);
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        let mut buf: [u8; 4096];
        loop {
            stream.readable().await?;
            buf = [0; 4096];
            match stream.try_read(&mut buf) {
                Ok(n) => {
                    let mut vec = Vec::with_capacity(n);
                    buf.take(n as u64).read_to_end(&mut vec).await?;
                    let s = String::from_utf8(buf.to_vec()).unwrap();
                    println!("Got reply from host: {}", s);
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => return Err(e.into()),
            }
        }
        if close {
            println!("Amongus");
            break Ok(());
        }
    }
}
