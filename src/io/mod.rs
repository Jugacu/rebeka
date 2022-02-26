pub(crate) mod common;

use std::collections::HashMap;
use std::net::SocketAddr;
use crate::io::common::{StatusCode};

pub struct Request {
    pub remote_addr: SocketAddr,
    pub method: String,
    pub path: String,
    headers: HashMap<String, String>,
}

impl Request {
    pub fn new(
        remote_addr: SocketAddr,
        method: String,
        path: String,
        headers: HashMap<String, String>,
    ) -> Request {
        Request {
            remote_addr,
            method,
            path,
            headers,
        }
    }
}

pub struct Response {
    headers: HashMap<String, String>,
    status: StatusCode,
    content: Vec<u8>,
}

impl Response {
    pub fn new(
        headers: HashMap<String, String>,
        status: StatusCode,
        content: Vec<u8>,
    ) -> Response {
        Response {
            headers,
            status,
            content,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
    //                let headers = "HTTP/1.1 404 Not found\r\nContent-Type: text/html\r\n\r\n".as_bytes().to_vec();
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend(b"HTTP/1.1 ");
        bytes.extend(self.status.to_string().as_bytes());
        bytes.extend(b"\r\n");

        for (field, value) in &self.headers {
            bytes.extend(format!("{}:{}", field, value).as_bytes());
            bytes.extend(b"\r\n");
        }

        bytes.extend(b"\r\n");

        bytes.extend(&self.content);

        bytes
    }
}