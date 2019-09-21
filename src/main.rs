use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufRead, Error, IoSlice};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    for stream in listener.incoming() {
        connect(stream);
    }
}

fn connect(stream: Result<TcpStream, Error>) {
    match stream {
        Ok(stream) => {
            request(&stream);
            respond(&stream);
        }
        Err(_) => { /* connection failed */ }
    }
}

fn respond(mut stream: &TcpStream) {
    match stream.write_vectored(&response_ok("Hello from the otherside")) {
        Ok(_) => true,
        Err(_) => false
    };
}

fn request(stream: &TcpStream) -> Vec<String> {
    let reader = BufReader::new(stream);

    reader.split('\n' as u8)
        .map(Result::unwrap)
        .take_while(|xs| xs.len() > 1)
        .map(|xs| String::from_utf8(xs).unwrap())
        .collect()
}

fn response_ok(msg: &str) -> [IoSlice; 2] {
    [IoSlice::new("HTTP/1.1 200 OK\n\n".as_bytes()),
     IoSlice::new(msg.as_bytes())]
}
