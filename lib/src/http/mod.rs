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
    use std::collections::HashMap;

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
}
