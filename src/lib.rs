use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufWriter, BufReader, Error};
use std::thread;

pub mod http;

type ResponderType = fn(&TcpStream, http::request::Request) -> Result<(), Error>;

pub fn respond(stream: &TcpStream, msg: &str) -> Result<(), Error> {
    let mut responder = BufWriter::new(stream);

    match responder.write(&http::response::ok(
        msg,
        vec!["Content-Type: text/html; charset=utf-8".to_string()]
    ).to_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

pub fn serve(host: &str, port: isize, responder: ResponderType) {
    let listener = TcpListener::bind([host, ":", &port.to_string()].concat()).unwrap();

    for stream in listener.incoming() {
        thread::spawn(move || {
            connect(stream, responder);
        });
    }
}

fn connect(stream: Result<TcpStream, Error>, responder: ResponderType) {
    match stream {
        Ok(stream) => {
            inquire(&stream).and_then(|request| responder(&stream, request))
                .unwrap_or_default();
        },
        Err(_) => {}
    };
}

fn inquire(stream: &TcpStream) -> Result<http::request::Request, Error> {
    let inquirer = BufReader::new(stream);

    http::request::parse(inquirer)
}
