use std::convert::{TryFrom};
use std::io::{Read};
use std::net::TcpListener;

use crate::http::{ParseError, Request, Response, StatusCode};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

const MAX_BUFFER_LENGTH: usize = 1024;

pub(crate) struct Server<'a> {
    addr: &'a str,
}

impl<'a> Server<'a> {
    pub(crate) fn new(addr: &'a str) -> Self {
        Self { addr }
    }

    pub fn run(&self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        let tcp_listener = TcpListener::bind(self.addr).unwrap();

        loop {
            match tcp_listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; MAX_BUFFER_LENGTH];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read request: {}", e),
                    };
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            };
        }
    }
}
