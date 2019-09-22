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

    pub struct Request {
        request_line: String,
        request: Vec<String>
    }

    impl Request {
        pub fn request_line(&self) -> &String {
            &self.request_line
        }

        pub fn request(&self) -> &Vec<String> {
            &self.request
        }
    }

    pub fn parse(mut reader: BufReader<&TcpStream>) -> Result<Request, Error> {

        let request_line: Result<String, Error> = reader.by_ref().lines().take(1).collect();

        match request_line {
            Ok(req) => {
                // TODO: deduce from req which method to parse

                Ok(Request {
                    request_line: req,
                    request: parse_get_request(reader)
                })
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
}
