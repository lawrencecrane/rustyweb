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
        headers.insert(0, "HTTP/1.1 200 OK".to_string());

        Response {
            headers: headers.join("\n").as_bytes().to_vec(),
            body: format!("\n\n{}", msg).as_bytes().to_vec()
        }
    }
}

pub mod request {
    // * HTTP REQUEST *
    // Request line
    // Headers
    // An empty line
    // Optional HTTP message body data

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
        request_line: RequestLine,
        request: Vec<String>
    }

    #[derive(Debug)]
    pub struct RequestLine {
        method: Method,
        uri: String,
        version: String
    }

    impl Request {
        pub fn new(request_line: RequestLine, request: Vec<String>) -> Request {
            Request {
                request_line: request_line,
                request: request
            }
        }

        pub fn get_method_and_uri(&self) -> (&Method, &str) {
            self.request_line.get_method_and_uri()
        }

        pub fn request(&self) -> &Vec<String> {
            &self.request
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
