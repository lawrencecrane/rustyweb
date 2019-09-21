use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufRead, Error, ErrorKind};

mod http_response;

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

fn respond(mut stream: &TcpStream, request: Vec<String>) -> Result<usize, Error> {
    println!("{:?}", request);

    stream.write_vectored(&http_response::ok("Hello from the otherside"))
}

fn inquire(stream: &TcpStream) -> Result<Vec<String>, Error> {
    let reader = BufReader::new(stream);

    let req: Vec<String> = reader.lines()
        .map(|line| line.unwrap_or(String::new()))
        .take_while(|line| !line.is_empty())
        .collect();

    match req.len() {
        0 => Err(Error::new(ErrorKind::InvalidData, "Empty request")),
        _ => Ok(req)
    }
}
