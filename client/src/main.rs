// src/bin/client.rs
use shared::data::{encode_f32, parse};
use shared_proc::netfunc;
use std::error::Error;
use std::io::{self};
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
    let mut client = shared::clients::Client::new();

    shared::connect!(2, client, testa);
    loop {
        loop {
            client.send(2, &encode_f32(200.0));
            client.push_input();
            println!("Expecting input...");
            let mut buffer = String::new();
            let stdin = io::stdin(); // We get `Stdin` here.
            stdin.read_line(&mut buffer)?;
            stream.writable().await?;
            match stream.try_write(&client.inputbffr) {
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

        loop {
            stream.readable().await?;
            match stream.try_read(&mut client.outputbffr) {
                Ok(n) => {
                    println!("Read {n} bytes");
                    match client.exec_data(parse(&client.outputbffr)?) {
                        Err(e) => {
                            println!("{e}");
                            break;
                        }
                        Ok(()) => break,
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => return Err(e.into()),
            }
        }
    }
}
