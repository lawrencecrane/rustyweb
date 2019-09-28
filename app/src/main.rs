extern crate rustyweb;

use std::net::TcpStream;
use std::io::{Error, ErrorKind};
use rustyweb::http::request::{Request, Method};
use rustyweb::http::response;
use rustyweb::web;

fn main() {
    web::server::serve("0.0.0.0", 8080, respond)
}

fn respond(stream: &TcpStream, request: Request) -> Result<(), Error> {
    let headers = vec!["Content-Type: text/html; charset=utf-8".to_string()];

    match (request.get_method_and_uri(), request.is_websocket_upgrade()) {
        ((Method::GET, "/"), false) =>
            web::server::respond(stream,
                                 response::ok(include_str!("../client/dist/index.html"),
                                              headers)),
        ((Method::GET, "/bundle.js"), false) =>
            web::server::respond(stream,
                                 response::ok(include_str!("../client/dist/bundle.js"),
                                              headers)),
        ((Method::GET, "/ws"), true) => {
            match web::server::upgrade_to_websocket(stream, request) {
                Ok(ok) => {
                    let msg = web::server::read_from_websocket(stream)
                        .unwrap_or(Vec::new());

                    println!("{:?}", msg);

                    Ok(ok)
                },
                Err(err) => Err(err)
            }
        },
        _ =>
            Err(Error::new(ErrorKind::NotFound, "404"))
    }
}
