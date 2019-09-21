use std::net::{TcpListener, TcpStream};
use std::io::{Write, Error};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    for stream in listener.incoming() {
        connect(stream);
    }
}

fn connect(stream: Result<TcpStream, Error>) {
    match stream {
        Ok(stream) => {
            respond(stream);
        }
        Err(_) => { /* connection failed */ }
    }
}

fn respond(mut stream: TcpStream) {
    match stream.write(&response_ok("Hello from the otherside")) {
        Ok(_) => {
            true;
        }
        Err(_) => {
            false;
        }
    }
}

fn response_ok(msg: &str) -> Vec<u8> {
    ["HTTP/1.1 200 OK \n\n", msg].concat().as_bytes().to_vec()
}
