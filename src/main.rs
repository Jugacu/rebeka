pub(crate) mod io;

pub(crate) mod server;
pub(crate) mod connection;

use std::collections::HashMap;
use std::fs;
use std::path::{Path};
use crate::io::Response;

use crate::server::*;

fn main() {
    create_server("0.0.0.0", 80, |mut request| {
        let final_path = ["./public".to_string(), request.path].join("");

        let path = Path::new(&final_path);

        match fs::read(path) {
            Ok(contents) => {
                Response::new(
                    HashMap::from([]),
                    200.into(),
                    contents,
                )
            }
            Err(err) => {
                Response::new(
                    HashMap::from([]),
                    404.into(),
                    "".as_bytes().to_vec(),
                )
            }
        }
    });
}
