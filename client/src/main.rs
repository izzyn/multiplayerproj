// src/bin/client.rs
use std::error::Error;
use std::io;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;



#[tokio::main]
async fn main() {
    let server_addr = "127.0.0.1:1234";
    let stream: TcpStream = TcpStream::connect(server_addr).await.unwrap();
    println!("Connected to {}", stream.peer_addr().unwrap());
    send_request(stream).await.unwrap();
}

async fn send_request(stream: TcpStream) -> Result<(), Box<dyn Error>> {

    let mut buf = BuffData{
        d_type: Type::String,
        data: Vec::new(),
    };
    
    loop {
        println!("Expecting input...");
        let mut close = false;
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer)?;
        match buffer.trim(){
            "close" => close = true,
            _ => {println!("Not real argument");},
        }
        loop {
            stream.writable().await?;
            match stream.try_write(b"Hello!") {
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
        if close{
            println!("Amongus");
            break Ok(());
        }
    }

}

