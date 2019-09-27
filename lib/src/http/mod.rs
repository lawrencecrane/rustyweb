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
        headers.push(format!("Content-Length: {}\n\n", body.len()));

        Response {
            headers: headers.join("\n").as_bytes().to_vec(),
            body: body.to_vec()
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
            match (self.headers.get("Connection"), self.headers.get("Upgrade")) {
                (Some(con), Some(upg)) => con == "Upgrade" && upg == "websocket",
                (_, _) => false
            }
        }

        pub fn generate_websocket_accept_value(&self) -> Option<String> {
            match self.headers.get("Sec-WebSocket-Key") {
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
            headers.insert("Sec-WebSocket-Key".to_string(),
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
            let mut headers = HashMap::new();

            let request = Request::new(RequestLine::new(Method::GET,
                                                        "/".to_string(),
                                                        "".to_string()),
                                       headers,
                                       None);

            assert_eq!(request.generate_websocket_accept_value().is_none(), true)
        }
    }
}
