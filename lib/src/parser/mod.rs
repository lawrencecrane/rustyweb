pub mod request {
    use std::net::TcpStream;
    use std::io::{Read, BufReader, BufRead, Error, ErrorKind};

    use crate::http::request;

    pub fn parse(mut reader: BufReader<&TcpStream>) -> Result<request::Request, Error> {
        let request_line: Result<String, Error> = reader.by_ref().lines().take(1).collect();

        match request_line {
            Ok(req) => {
                match parse_request_line(req) {
                    Ok(parsed) => Ok(request::Request::new(parsed,
                                                           // TODO: parse these to dict/map
                                                           parse_get(reader))),
                    Err(e) => Err(e)
                }
            },
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "Empty request"))
        }
    }

    fn parse_get(reader: BufReader<&TcpStream>) -> Vec<String> {
        reader.lines()
            .map(|line| line.unwrap_or(String::new()))
            .take_while(|line| !line.is_empty())
            .collect()
    }

    fn parse_request_line(line: String) -> Result<request::RequestLine, Error> {
        let splitted: Vec<&str> = line.split_whitespace()
            .collect();

        match splitted.len() {
            3 => {
                match parse_method(splitted[0]) {
                    Some(method) => Ok(request::RequestLine::new(method,
                                                                 splitted[1].to_string(),
                                                                 splitted[2].to_string())),
                    None => Err(Error::new(ErrorKind::InvalidData, "Not valid method"))
                }
            },
            _ => Err(Error::new(ErrorKind::InvalidData, "Not valid request line"))
        }
    }

    fn parse_method(x: &str) -> Option<request::Method> {
        match x {
            "GET" => Some(request::Method::GET),
            _ => None
        }
    }
}
