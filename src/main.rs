extern crate rustyweb;

use std::net::TcpStream;
use std::io::{Error, ErrorKind};
use rustyweb::http::request::{Request, Method};

fn main() {
    rustyweb::serve("0.0.0.0", 80, respond)
}

fn respond(stream: &TcpStream, request: Request) -> Result<(), Error> {
    match request.get_method_and_uri() {
        (Method::GET, "/") => rustyweb::respond(stream, "Hello from the otherside"),
        _ => Err(Error::new(ErrorKind::NotFound, "404"))
    }
}
