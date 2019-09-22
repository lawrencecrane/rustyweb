extern crate rustyweb;

use std::net::TcpStream;
use std::io::{Error, ErrorKind};
use rustyweb::http::request::{Request, Method};
use rustyweb::web;

fn main() {
    web::server::serve("0.0.0.0", 8080, respond)
}

fn respond(stream: &TcpStream, request: Request) -> Result<(), Error> {
    let headers = vec!["Content-Type: text/html; charset=utf-8".to_string()];

    match request.get_method_and_uri() {
        (Method::GET, "/") => web::server::respond(stream,
                                                   "Hello from the otherside",
                                                   headers),
        _ => Err(Error::new(ErrorKind::NotFound, "404"))
    }
}
