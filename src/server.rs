use std::net::{Ipv4Addr, TcpListener, TcpStream};

pub fn create_server(host: &str, port: u16, closure: fn(TcpStream)) {
    let ipv4 = host.parse::<Ipv4Addr>().expect("Could not parse ip addr!");
    let listener = TcpListener::bind((ipv4, port)).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                closure(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}