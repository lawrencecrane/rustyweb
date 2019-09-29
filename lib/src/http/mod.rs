pub mod websocket {
    // * WEBSOCKET OPCODES *
    // %x0 denotes a continuation frame
    // %x1 denotes a text frame
    // %x2 denotes a binary frame
    // %x3–7 are reserved for further non-control frames
    // %x8 denotes a connection close
    // %x9 denotes a ping
    // %xA denotes a pong
    // %xB-F are reserved for further control frames
    #[derive(Debug)]
    pub enum Opcode {
        // CONTINUATION = 0,
        TEXT = 1,
        // BINARY = 2,
        CLOSE = 8,
        // PING = 9,
        // PONG = 10
    }

    pub struct Frame {
        pub payload: Vec<u8>
    }

    impl Frame {
        pub fn new(mut msg: Vec<u8>, opcode: Opcode) -> Frame {
            let payload = match msg.len() {
                length if length < 126 => {
                    let mut header =  vec![128 + opcode as u8, length as u8];
                    header.append(&mut msg);

                    header
                },
                length if length < 65536 => {
                    let mut header = vec![128 + opcode as u8, 126];
                    header.append(&mut (msg.len() as u16).to_be_bytes().to_vec());
                    header.append(&mut msg);

                    header
                },
                // TODO: support for payload length == 127 (payload length >= 2^16)
                _ => panic!()
            };

            Frame {
                payload: payload
            }
        }
    }

    #[derive(Debug)]
    pub struct Header {
        is_final_frame: bool,
        pub opcode: Opcode,
        pub is_masked: bool,
        pub payload_length: usize
    }

    impl Header {
        pub fn new(is_final_frame: bool,
                   opcode: Opcode,
                   is_masked: bool,
                   payload_length: usize) -> Header {
            Header {
                is_final_frame: is_final_frame,
                opcode: opcode,
                is_masked: is_masked,
                payload_length: payload_length
            }
        }
    }

    pub fn unmask_payload(payload: Vec<u8>, masking_key: Option<[u8; 4]>) -> Vec<u8> {
        match masking_key {
            Some(key) => payload.iter()
                    .enumerate()
                    .map(|(i, val)| val ^ key[i % 4])
                    .collect(),
            None => payload
        }
    }
}

pub mod response {
    pub struct Response {
        headers: Vec<u8>,
        body: Vec<u8>
    }

    impl Response {
        pub fn to_bytes(&self) -> Vec<u8> {
            [&self.headers[..], &self.body[..]].concat()
        }
    }

    pub fn ok(msg: &str, mut headers: Vec<String>) -> Response {
        let body = msg.as_bytes();

        headers.insert(0, "HTTP/1.1 200 OK".to_string());
        headers.push(format!("Content-Length: {}\r\n\r\n", body.len()));

        Response {
            headers: headers.join("\r\n").as_bytes().to_vec(),
            body: body.to_vec()
        }
    }

    pub fn websocket(key: String, proto: String) -> Response {
        let headers = vec!["HTTP/1.1 101 Web Socket Protocol Handshake".to_string(),
                           "Connection: Upgrade".to_string(),
                           "Content-Length: 0".to_string(),
                           format!("Sec-WebSocket-Accept: {}", key),
                           format!("Sec-WebSocket-Protocol: {}", proto),
                           "Upgrade: websocket\r\n\r\n".to_string()];

        Response {
            headers: headers.join("\r\n").as_bytes().to_vec(),
            body: vec![]
        }
    }
}

pub mod request {
    // * HTTP REQUEST *
    // Request line
    // Headers
    // An empty line
    // Optional HTTP message body data
    extern crate base64;
    extern crate crypto;

    use std::collections::HashMap;
    use crypto::digest::Digest;
    use crypto::sha1::Sha1;
    use base64::encode;

    const WEBSOCKET_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

    #[derive(Debug)]
    pub enum Method {
        GET,
        // HEAD,
        // POST,
        // PUT,
        // DELETE,
        // CORRECT,
        // OPTIONS,
        // TRACE,
        // PATCH
    }

    #[derive(Debug)]
    pub struct Request {
        request: RequestLine,
        headers: HashMap<String, String>,
        data: Option<String>
    }

    #[derive(Debug)]
    pub struct RequestLine {
        method: Method,
        uri: String,
        version: String
    }

    impl Request {
        pub fn new(request: RequestLine,
                   headers: HashMap<String, String>,
                   data: Option<String>) -> Request {
            Request {
                request: request,
                headers: headers,
                data: data
            }
        }

        pub fn get_method_and_uri(&self) -> (&Method, &str) {
            self.request.get_method_and_uri()
        }

        pub fn headers(&self) -> &HashMap<String, String> {
            &self.headers
        }

        pub fn is_websocket_upgrade(&self) -> bool {
            match (self.headers.get("connection"), self.headers.get("upgrade")) {
                (Some(con), Some(upg)) =>
                    con.to_lowercase() == "upgrade" && upg.to_lowercase() == "websocket",
                (_, _) => false
            }
        }

        pub fn get_websocket_protocol(&self) -> Option<Vec<&str>> {
            match self.headers.get("sec-websocket-protocol") {
                Some(protos) => {
                    Some(protos.split(",").collect())
                },
                None => None
            }
        }

        pub fn generate_websocket_accept_value(&self) -> Option<String> {
            match self.headers.get("sec-websocket-key") {
                Some(val) => {
                    let mut hasher = Sha1::new();
                    hasher.input_str(&format!("{}{}", val, WEBSOCKET_GUID).to_string());

                    let mut hash = vec![0; hasher.output_bytes()];
                    hasher.result(&mut hash);

                    Some(encode(&hash))
                },
                None => None
            }
        }
    }

    impl RequestLine {
        pub fn new(method: Method, uri: String, version: String) -> RequestLine {
            RequestLine {
                method: method,
                uri: uri,
                version: version
            }
        }

        fn get_method_and_uri(&self) -> (&Method, &str) {
            (&self.method, self.uri.as_ref())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_generate_websocket_accept_value_ok() {
            let mut headers = HashMap::new();
            headers.insert("sec-websocket-key".to_string(),
                           "dGhlIHNhbXBsZSBub25jZQ==".to_string());

            let request = Request::new(RequestLine::new(Method::GET,
                                                        "/".to_string(),
                                                        "".to_string()),
                                       headers,
                                       None);

            assert_eq!(request.generate_websocket_accept_value().unwrap(),
                       "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=")
        }

        #[test]
        fn test_generate_websocket_accept_value_ok2() {
            let mut headers = HashMap::new();
            headers.insert("sec-websocket-key".to_string(),
                           "JZSMZ2B02uL4y5/Bgg1tnw==".to_string());

            let request = Request::new(RequestLine::new(Method::GET,
                                                        "/".to_string(),
                                                        "".to_string()),
                                       headers,
                                       None);

            assert_eq!(request.generate_websocket_accept_value().unwrap(),
                       "0/NFjjkhrm2G5Yqyl/BWoNY/+AQ=")
        }

        #[test]
        fn test_generate_websocket_accept_value_ok3() {
            let mut headers = HashMap::new();
            headers.insert("sec-websocket-key".to_string(),
                           "dGhlIHNhbXBsZSBub25jZQ==".to_string());

            let request = Request::new(RequestLine::new(Method::GET,
                                                        "/".to_string(),
                                                        "".to_string()),
                                       headers,
                                       None);

            assert_eq!(request.generate_websocket_accept_value().unwrap(),
                       "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=")
        }

        #[test]
        fn test_generate_websocket_accept_value_bad() {
            let headers = HashMap::new();

            let request = Request::new(RequestLine::new(Method::GET,
                                                        "/".to_string(),
                                                        "".to_string()),
                                       headers,
                                       None);

            assert_eq!(request.generate_websocket_accept_value().is_none(), true)
        }
    }
}
