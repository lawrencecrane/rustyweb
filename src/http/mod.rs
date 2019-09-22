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

    pub fn ok(msg: &str, mut header: Vec<String>) -> Response {
        header.insert(0, "HTTP/1.1 200 OK".to_string());

        Response {
            headers: header.join("\n").as_bytes().to_vec(),
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

    use std::net::TcpStream;
    use std::io::{Read, BufReader, BufRead, Error, ErrorKind};

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
        pub fn get_method_and_uri(&self) -> (&Method, &str) {
            self.request_line.get_method_and_uri()
        }

        pub fn request(&self) -> &Vec<String> {
            &self.request
        }
    }

    impl RequestLine {
        fn get_method_and_uri(&self) -> (&Method, &str) {
            (&self.method, self.uri.as_ref())
        }
    }

    pub fn parse(mut reader: BufReader<&TcpStream>) -> Result<Request, Error> {
        let request_line: Result<String, Error> = reader.by_ref().lines().take(1).collect();

        match request_line {
            Ok(req) => {
                match parse_request_line(req) {
                    Ok(parsed) => {
                        Ok(Request {
                            request_line: parsed,
                            // TODO: implement parser for other request methods
                            request: parse_get_request(reader)
                        })
                    },
                    Err(e) => {
                        Err(e)
                    }
                }
            },
            Err(_) => {
                Err(Error::new(ErrorKind::InvalidData, "Empty request"))
            }
        }
    }

    fn parse_get_request(reader: BufReader<&TcpStream>) -> Vec<String> {
        reader.lines()
            .map(|line| line.unwrap_or(String::new()))
            .take_while(|line| !line.is_empty())
            .collect()
    }

    fn parse_request_line(line: String) -> Result<RequestLine, Error> {
        let splitted: Vec<&str> = line.split_whitespace()
            .collect();

        match splitted.len() {
            3 => {
                match parse_method(splitted[0]) {
                    Some(method) => {
                        Ok(RequestLine {
                            method: method,
                            uri: splitted[1].to_string(),
                            version: splitted[2].to_string()
                        })
                    },
                    None => {
                        Err(Error::new(ErrorKind::InvalidData, "Not valid method"))
                    }
                }
            },
            _ => {
                Err(Error::new(ErrorKind::InvalidData, "Not valid request line"))
            }
        }
    }

    fn parse_method(x: &str) -> Option<Method> {
        match x {
            "GET" => Some(Method::GET),
            _ => None
        }
    }
}
