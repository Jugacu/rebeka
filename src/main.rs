pub(crate) mod server;
pub(crate) mod header;
pub(crate) mod request;
pub(crate) mod connection;

use std::fs;
use std::io::{Read, Write};
use std::path::{Path};

use crate::server::*;

fn main() {
    create_server("0.0.0.0", 80, |mut con| {
        let request = con.read().unwrap();

        let final_path = ["./public".to_string(), request.path].join("");

        let path = Path::new(&final_path);

        match fs::read(path) {
            Ok(contents) => {
                let headers = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n".as_bytes().to_vec();

                con.write(&[headers, contents].concat()).unwrap();
            }
            Err(err) => {
                let headers = "HTTP/1.1 404 Not found\r\nContent-Type: text/html\r\n\r\n".as_bytes().to_vec();

                con.write(&[headers].concat()).unwrap();
            }
        }
    });
}
