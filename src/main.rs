pub(crate) mod server;

use std::fs;
use std::io::{Read, Write};
use std::path::{Path};

use crate::server::*;

fn main() {
    create_server("0.0.0.0", 80, |mut stream| {
        let mut buf = [0u8; 4096];
        stream.read(&mut buf).expect("Could not write request into buffer");

        let request_str = String::from_utf8_lossy(&buf);
        let incoming_request: Vec<&str> = request_str.split("\n").collect();


        let v = incoming_request[0].split_whitespace().take(3).collect::<Vec<&str>>();
        let [_req_method, req_path, _req_version] = <[&str; 3]>::try_from(v).unwrap();

        let final_path = ["./public", req_path].join("");

        let path = Path::new(&final_path);

        match fs::read(path) {
            Ok(contents) => {
                let headers = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n".as_bytes().to_vec();

                stream.write(&[headers, contents].concat()).unwrap();
            }
            Err(err) => {
                let headers = "HTTP/1.1 404 Not found\r\nContent-Type: text/html\r\n\r\n".as_bytes().to_vec();

                stream.write(&[headers].concat()).unwrap();
            }
        }
    });
}
