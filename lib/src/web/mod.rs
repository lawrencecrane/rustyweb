pub mod server {
    use std::net::{TcpListener, TcpStream};
    use std::io::{Write, BufWriter, BufReader, Error, ErrorKind};
    use std::thread;
    use serde_json;

    use crate::http;
    use crate::parser;

    type JSON = serde_json::Value;
    type ResponderType = fn(&TcpStream, http::request::Request) -> Result<(), Error>;

    pub struct WebSocketJSONHandler {}

    impl WebSocketCommunicator<JSON> for WebSocketJSONHandler {
        fn protocol(&self) -> &str{
            "json"
        }

        fn read(&self, stream: &TcpStream) -> Result<Option<JSON>, Error> {
            match parser::websocket::parse(stream) {
                Ok(Some(msg)) =>
                    Ok(Some(serde_json::from_str(&String::from_utf8(msg).unwrap()).unwrap())),
                Ok(None) => Ok(None),
                Err(err) => Err(err)
            }
        }
    }

    pub trait WebSocketCommunicator<T> where T: std::fmt::Debug {
        fn protocol(&self) -> &str;
        fn read(&self, stream: &TcpStream) -> Result<Option<T>, Error>;
    }

    pub fn communicate_via_websocket<T> (
        stream: &TcpStream,
        request: http::request::Request,
        communicator: impl WebSocketCommunicator<T>
    ) -> Result<(), Error> where T: std::fmt::Debug {
        upgrade_to_websocket(stream, request, communicator.protocol()).unwrap();

        loop {
            match communicator.read(stream) {
                Ok(Some(msg)) => {
                    println!("{:?}", msg);
                },
                Ok(None) => { break Ok(()); }
                Err(err) => { break Err(err); }
            }
        }
    }

    fn upgrade_to_websocket(stream: &TcpStream,
                            request: http::request::Request,
                            protocol: &str) -> Result<(), Error> {
        let is_ok = match request.get_websocket_protocol() {
            Some(protos) => protos.contains(&protocol),
            None => true
        };

        match (request.generate_websocket_accept_value(), is_ok) {
            (Some(key), true) =>
                respond(stream, http::response::websocket(key, protocol.to_string())),
            _ => Err(Error::new(ErrorKind::ConnectionAborted, ""))
        }
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
