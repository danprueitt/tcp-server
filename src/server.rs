use std::convert::{TryFrom, TryInto};
use std::io::Read;
use std::net::TcpListener;

use crate::http::Request;

const MAX_BUFFER_LENGTH: usize = 1024;

pub(crate) struct Server<'a> {
    addr: &'a str,
}

impl<'a> Server<'a> {
    pub(crate) fn new(addr: &'a str) -> Self {
        Self { addr }
    }

    pub fn run(&self) {
        println!("Listening on {}", self.addr);
        let tcp_listener = TcpListener::bind(self.addr).unwrap();

        loop {
            match tcp_listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; MAX_BUFFER_LENGTH];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse request: {}", e)
                            };
                        }
                        Err(e) => println!("Failed to read request: {}", e),
                    };
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            };
        }
    }
}
