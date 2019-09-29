pub mod server {
    use std::net::{TcpListener, TcpStream};
    use std::io::{Write, BufWriter, BufReader, Error, ErrorKind};
    use std::thread;

    use crate::http;
    use crate::parser;

    type ResponderType = fn(&TcpStream, http::request::Request) -> Result<(), Error>;

    pub trait WebSocketCommunicator<T> {
        fn protocol(&self) -> &str;
        fn read(&self, stream: &TcpStream) -> Result<Option<T>, Error>;
        fn write(&self, stream: &TcpStream, msg: T) -> Result<(), Error>;
    }

    /// Communicate with single client via websocket
    /// by reading their message and then sending them message
    pub fn websocket_echo_chamber<T> (
        stream: &TcpStream,
        request: http::request::Request,
        communicator: impl WebSocketCommunicator<T>
    ) -> Result<(), Error> {
        upgrade_to_websocket(stream, request, communicator.protocol()).unwrap();

        loop {
            match communicator.read(stream) {
                Ok(Some(msg)) => {
                    match communicator.write(stream, msg) {
                        Ok(_) => {},
                        Err(err) => { break Err(err); }
                    }
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

    /// Implementation of responder function that can be used in connect
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

    /// Read request from client and then pass it to responder
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
