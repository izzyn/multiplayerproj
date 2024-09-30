// src/bin/server.rs
use std::{error::Error, i32, io};
use tokio::{
    io::{AsyncReadExt, Interest},
    net::{TcpListener, TcpStream},
};
use core::net::SocketAddr;

struct Client {
    sendpkts : Vec<i32>,
    sendbfr : [i32; 2048],
    readbfr : [i32; 2048],
    addr : SocketAddr,
    
}


#[tokio::main]
async fn main() {
    let bind_addr = "0.0.0.0:1234";
    listen(bind_addr).await;
}

async fn listen(bind_addr: &str) {
    let listener = TcpListener::bind(bind_addr).await.unwrap();

    loop {
        let (stream, _) = match listener.accept().await {
            Ok(t) => t,
            Err(_e) => continue,
        };
        tokio::spawn(async move {
            match handle_stream(stream).await {
                Ok(t) => t,
                Err(e) => println!("{:?}", e),
            };
        });
    }
}

async fn handle_stream(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Connection from {}", stream.peer_addr().unwrap());
    let mut reply_queue: Vec<Vec<u8>> = Vec::new();
    let mut buf: [u8; 1024];
    
    loop {
        let ready = stream
            .ready(Interest::READABLE | Interest::WRITABLE)
            .await?;
        if ready.is_readable() {
	    buf = [0; 1024];
            match stream.try_read(&mut buf) {
                Ok(n) => {
                    println!("{:?}", buf);
                    let parsed = match shared::data::parse(&buf) {
                        Ok(t) => println!("{:?}", t.nodes[0].data),
                        Err(e) => return Err(Box::new(e))
                    };

                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        if ready.is_writable() {
            if let Some(msg) =  reply_queue.pop() {
                match stream.try_write(&msg) {
                    Ok(n) => {
                        println!("Wrote {} bytes", n);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }
        }
    }
}
