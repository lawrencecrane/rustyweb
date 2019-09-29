pub mod json {
    use std::net::TcpStream;
    use std::io::{Write, BufWriter, Error};
    use serde_json;

    use crate::http;
    use crate::parser;
    use crate::web;

    use web::server::WebSocketCommunicator;

    type JSON = serde_json::Value;

    pub struct Communicator {}

    /// Implementation of WebSocketCommunicator that communicates through JSON in echo chamber
    impl WebSocketCommunicator<JSON> for Communicator {
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

        fn write(&self, stream: &TcpStream, msg: JSON) -> Result<(), Error> {
            println!("{:?}", msg);
            let payload = http::websocket::Frame::new(serde_json::to_vec(&msg).unwrap(),
                                                      http::websocket::Opcode::TEXT);

            let mut writer = BufWriter::new(stream);

            match writer.write(&payload.payload) {
                Ok(_) => Ok(()),
                Err(err) => Err(err)
            }
        }
    }
}
