use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufRead, Error, IoSlice};
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    for stream in listener.incoming() {
        connect(stream);
    }
}

fn connect(stream: Result<TcpStream, Error>) {
    match stream {
        Ok(stream) => {
            get_request(&stream).and_then(|_| respond(&stream))
                .unwrap_or_default();
        },
        Err(_) => {}
    };
}

fn respond(mut stream: &TcpStream) -> Result<usize, Error> {
    stream.set_write_timeout(Some(Duration::new(2, 0)))
        .unwrap();

    stream.write_vectored(&response_ok("Hello from the otherside"))
}

fn get_request(stream: &TcpStream) -> Result<Vec<String>, Error> {
    stream.set_read_timeout(Some(Duration::new(2, 0)))
        .unwrap();

    let reader = BufReader::new(stream);

    reader.split('\n' as u8)
        .take_while(|x| x.as_ref().unwrap_or(&Vec::new()).len() > 1)
        .map(parse_request)
        .collect()
}

fn parse_request(xs: Result<Vec<u8>, Error>) -> Result<String, Error> {
    match xs {
        Ok(line) => Ok(String::from_utf8(line).unwrap()),
        Err(e) => Err(e)
    }
}

fn response_ok(msg: &str) -> [IoSlice; 2] {
    [IoSlice::new("HTTP/1.1 200 OK\n\n".as_bytes()),
     IoSlice::new(msg.as_bytes())]
}
