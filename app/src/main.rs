extern crate rustyweb;

use std::net::TcpStream;
use std::io::{Error, ErrorKind};
use rustyweb::http::request::{Request, Method};
use rustyweb::http::response;
use rustyweb::web::server;
use rustyweb::websocket::json::Communicator;

fn main() {
    server::serve("0.0.0.0", 8080, respond)
}

fn respond(stream: &TcpStream, request: Request) -> Result<(), Error> {
    let headers = vec!["Content-Type: text/html; charset=utf-8".to_string()];

    match (request.get_method_and_uri(), request.is_websocket_upgrade()) {
        ((Method::GET, "/"), false) =>
            server::respond(stream,
                            response::ok(include_str!("../client/dist/index.html"), headers)),
        ((Method::GET, "/bundle.js"), false) =>
            server::respond(stream,
                            response::ok(include_str!("../client/dist/bundle.js"), headers)),
        ((Method::GET, "/ws"), true) =>
            server::websocket_echo_chamber(stream, request, Communicator {}),
        _ =>
            Err(Error::new(ErrorKind::NotFound, "404"))
    }
}
