pub mod server {
    use std::net::{TcpListener, TcpStream};
    use std::io::{Write, BufWriter, BufReader, Error, ErrorKind};
    use std::thread;

    use crate::http;
    use crate::parser;

    type ResponderType = fn(&TcpStream, http::request::Request) -> Result<(), Error>;

    pub fn upgrade_to_websocket(stream: &TcpStream,
                                request: http::request::Request) -> Result<(), Error> {
        match (request.generate_websocket_accept_value(),
               request.get_websocket_protocol() == "json") {
            (Some(key), true) =>
                respond(stream, http::response::websocket(key, "json".to_string())),
            _ => Err(Error::new(ErrorKind::ConnectionAborted, ""))
        }
    }

    pub fn read_from_websocket(stream: &TcpStream) -> Result<Vec<u8>, Error> {
        parser::websocket::parse(stream)
    }

    pub fn respond(stream: &TcpStream,
                   response: http::response::Response) -> Result<(), Error> {
        let mut responder = BufWriter::new(stream);

        match responder.write(&response.to_bytes()) {
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

        parser::request::parse(inquirer)
    }
}
