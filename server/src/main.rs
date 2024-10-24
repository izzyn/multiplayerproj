// src/bin/server/main.rs
use shared::{
    clients::Client,
    connect,
    data::{encode_i32, encode_string},
};
use shared_proc::netfunc;
use std::{error::Error, io};
use tokio::{
    io::Interest,
    net::{TcpListener, TcpStream},
};

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

#[netfunc]
fn test(a: f32) {
    println!("{a} WOAH!");
}
async fn handle_stream(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Connection from {}", stream.peer_addr().unwrap());
    let mut client = Client::new();
    let mut queue: Vec<u8> = vec![];
    connect!(2, client, test);

    loop {
        let ready = stream
            .ready(Interest::READABLE | Interest::WRITABLE)
            .await?;
        if ready.is_readable() {
            match stream.try_read(&mut client.outputbffr) {
                Ok(_n) => {
                    match shared::data::parse(&client.outputbffr) {
                        Ok(t) => {
                            let _ = client.exec_data(t);
                            queue.push(1);
                        }
                        Err(e) => return Err(Box::new(e)),
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
            if let Some(_a) = queue.pop() {
                client.send(
                    2,
                    &[
                        encode_string("hello wassup!".to_string())?,
                        encode_i32(32).to_vec(),
                    ]
                    .to_vec()
                    .concat(),
                );
                client.push_input();
                match stream.try_write(&client.inputbffr) {
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
