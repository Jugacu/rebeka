use std::net::SocketAddr;

use crate::header::Header;

pub struct Request {
    pub remote_addr: SocketAddr,
    pub method: String,
    pub path: String,
    pub headers: Vec<Header>,
}

impl Request {
    pub fn new(
        remote_addr: SocketAddr,
        method: String,
        path: String,
        headers: Vec<Header>
    ) -> Request {
        Request {
            remote_addr,
            method,
            path,
            headers
        }
    }

    pub fn respond(&self) {
    }
}