use crate::io::{Request, Response};
use std::net::{Ipv4Addr, TcpListener};
use crate::connection::Connection;

pub fn create_server(host: &str, port: u16, closure: fn(Request) -> Response) {
    let ipv4 = host.parse::<Ipv4Addr>().expect("Could not parse ip addr!");
    let listener = TcpListener::bind((ipv4, port)).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut con =  Connection::new(stream);

                let response = closure(con.read().unwrap());

                con.write(&response.to_bytes()).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}