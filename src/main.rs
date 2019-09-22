use std::net::TcpStream;
use std::io::{Error, ErrorKind};

extern crate rustyweb;

fn main() {
    rustyweb::serve("0.0.0.0", 80, respond)
}

fn respond(stream: &TcpStream,
           request: rustyweb::http::request::Request) -> Result<(), Error> {
    match (&request.request_line().method, request.request_line().uri.as_ref()) {
        (rustyweb::http::request::Method::GET, "/") => {
            rustyweb::respond(stream, "Hello from the otherside")
        },
        _ => {
            Err(Error::new(ErrorKind::NotFound, "404"))
        }
    }
}
