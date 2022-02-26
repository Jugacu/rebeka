use std::net::{Ipv4Addr, TcpListener};
use crate::connection::Connection;

use crate::request::Request;

pub fn create_server(host: &str, port: u16, closure: fn(Connection)) {
    let ipv4 = host.parse::<Ipv4Addr>().expect("Could not parse ip addr!");
    let listener = TcpListener::bind((ipv4, port)).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                closure(Connection::new(stream))
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}