use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufRead, Error};
use std::time::Duration;

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
            inquire(&stream).and_then(|_| respond(&stream))
                .unwrap_or_default();
        },
        Err(_) => {}
    };
}

fn respond(mut stream: &TcpStream) -> Result<usize, Error> {
    stream.set_write_timeout(Some(Duration::new(2, 0)))
        .unwrap();

    stream.write_vectored(&http_response::ok("Hello from the otherside"))
}

fn inquire(stream: &TcpStream) -> Result<Vec<String>, Error> {
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
