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

    match (request.is_websocket_upgrade(), request.get_method_and_uri()) {
        (true, (Method::GET, "/")) => {
            println!("Got websocket!");
            Err(Error::new(ErrorKind::NotFound, "404"))
        },
        (false, (Method::GET, "/")) =>
            web::server::respond(stream,
                                 include_str!("../client/dist/index.html"),
                                 headers),
        (false, (Method::GET, "/bundle.js")) =>
            web::server::respond(stream,
                                 include_str!("../client/dist/bundle.js"),
                                 headers),
        _ =>
            Err(Error::new(ErrorKind::NotFound, "404"))
    }
}
