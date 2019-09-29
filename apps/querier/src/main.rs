extern crate rustyweb;

use std::net::TcpStream;
use std::io::{Write, BufWriter, Error, ErrorKind};
use serde_json;

use rustyweb::web::{server, websocket};
use rustyweb::parser;
use rustyweb::http;
use rustyweb::http::request::{Request, Method};
use rustyweb::http::response;

type JSON = serde_json::Value;

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
            websocket::echo_chamber(stream, request, EchoChamber {}),
        _ =>
            Err(Error::new(ErrorKind::NotFound, "404"))
    }
}

struct EchoChamber {}

impl websocket::Communicator<JSON> for EchoChamber {
    fn protocol(&self) -> &str{
        "json"
    }

    fn receive(&self, stream: &TcpStream) -> Result<Option<JSON>, Error> {
        match parser::websocket::parse(stream) {
            Ok(Some(msg)) =>
                Ok(Some(serde_json::from_str(&String::from_utf8(msg).unwrap()).unwrap())),
            Ok(None) => Ok(None),
            Err(err) => Err(err)
        }
    }

    fn send(&self, stream: &TcpStream, msg: JSON) -> Result<(), Error> {
        println!("{:?}", msg);
        let payload = http::websocket::Frame::new(serde_json::to_vec(&msg).unwrap(),
                                                  http::websocket::Opcode::TEXT);

        let mut writer = BufWriter::new(stream);

        match writer.write(&payload.payload) {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
}
