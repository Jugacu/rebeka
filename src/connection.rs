use std::io::Result as IOResult;
use std::io::{Read, Write};
use std::net::{TcpStream};
use crate::header::Header;
use crate::request::Request;

#[derive(Debug)]
pub enum ConnectionError {}

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(
        stream: TcpStream
    ) -> Connection {
        Connection {
            stream
        }
    }

    pub fn read(&mut self) -> Result<Request, ConnectionError> {
        let mut buf = [0u8; 4096];
        let _ = &self.stream.read(&mut buf).unwrap();

        let request_str = String::from_utf8_lossy(&buf);
        let incoming_request: Vec<&str> = request_str.split("\n").collect();

        // The first line is always the METHOD line
        let req_v = incoming_request[0].split_whitespace().take(3).collect::<Vec<&str>>();
        let [req_method, req_path, _req_version] = <[&str; 3]>::try_from(req_v).unwrap();

        let headers = {
            let mut headers: Vec<Header> = vec![];

            // We skip the first line, which corresponds to the METHOD line
            for header_line in incoming_request.iter().skip(1) {
                let mut header_tuple = header_line.split(": ").collect::<Vec<&str>>();

                if header_tuple.len() != 2 {
                    continue;
                }

                headers.push(Header::new(
                    header_tuple[0].to_string(),
                    header_tuple[1].to_string(),
                ))
            }

            headers
        };

        Ok(
            Request::new(
                *self.stream.peer_addr().as_ref().unwrap(),
                req_method.to_string(),
                req_path.to_string(),
                headers,
            )
        )
    }

    pub fn write(&mut self, bytes: &Vec<u8>) -> IOResult<usize> {
        self.stream.write(bytes)
    }
}