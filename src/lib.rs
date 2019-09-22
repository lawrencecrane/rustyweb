use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufWriter, BufReader, Error};

mod http;

pub fn serve(host: &str, port: isize) {
    let listener = TcpListener::bind([host, ":", &port.to_string()].concat()).unwrap();

    for stream in listener.incoming() {
        connect(stream);
    }
}

fn connect(stream: Result<TcpStream, Error>) {
    match stream {
        Ok(stream) => {
            inquire(&stream).and_then(|request| respond(&stream, request))
                .unwrap_or_default();
        },
        Err(_) => {}
    };
}

fn respond(stream: &TcpStream, request: http::request::Request) -> Result<usize, Error> {
    println!("{:?}", request.request_line());
    println!("{:?}", request.request());

    let mut responder = BufWriter::new(stream);

    responder.write(
        &http::response::ok("Hello from the otherside",
                            vec!["Content-Type: text/html; charset=utf-8".to_string()])
            .to_bytes()
    )
}

fn inquire(stream: &TcpStream) -> Result<http::request::Request, Error> {
    let inquirer = BufReader::new(stream);

    http::request::parse(inquirer)
}
