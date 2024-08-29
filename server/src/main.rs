use std::{
    collections::hash_map::HashMap, io::{prelude::*, BufReader}, net::{SocketAddr, TcpListener, TcpStream}
};

struct Client {
    ualive : bool,
    address : SocketAddr,
}
fn main() {
    let mut clients : HashMap<SocketAddr, &Client> = HashMap::new();
    let listener = TcpListener::bind("127.0.0.1:34254").unwrap();

    for stream in listener.incoming() {
        let stream = match stream {
            Err(err) => {
                println!("Recieved stream error {err:?}");
                continue;
            },
            Ok(i) => i,
        };
        let _ = stream.set_nonblocking(true);

        handle_connection(stream, &clients);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream : TcpStream, mut clients : &HashMap<SocketAddr, &Client>){
    let addr = match stream.peer_addr(){
        Ok(adress) => adress,
        Err(err) => {
            println!("Got error: {err:?} while trying to recover the peer adress");
            return;
        }
    };
    
    
    println!("Got connection from {}", addr);
}
