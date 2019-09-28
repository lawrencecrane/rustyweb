pub mod websocket {
    use std::net::TcpStream;
    use std::io::{Read, BufReader};

    pub fn parse(stream: &TcpStream) -> Option<Vec<u8>> {
        let mut reader = BufReader::new(stream);
        let mut buffer = Vec::new();

        match reader.read_to_end(&mut buffer) {
            Ok(_) => Some(buffer),
            Err(_) => None
        }
    }
}

pub mod request {
    use std::net::TcpStream;
    use std::io::{Read, BufReader, BufRead, Error, ErrorKind};
    use std::collections::HashMap;

    use crate::http::request;

    pub fn parse(mut reader: BufReader<&TcpStream>) -> Result<request::Request, Error> {
        let request_line: Result<String, Error> = reader.by_ref().lines().take(1).collect();

        match request_line {
            Ok(req) => {
                match parse_request_line(req) {
                    Ok(parsed) => {
                        let headers = to_headers(parse_get(reader));

                        println!("{:?}", headers);

                        Ok(request::Request::new(parsed,
                                                 headers,
                                                 None))
                    },
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

    fn to_headers(headers: Vec<String>) -> HashMap<String, String> {
        headers.iter()
            .map(split_header)
            .filter(|x| match x {
                Some(_) => true,
                None => false
            })
            .map(|x| x.unwrap())
            .collect()
    }

    fn split_header(header: &String) -> Option<(String, String)> {
        match header.find(":") {
            Some(idx) => {
                let (key, val) = header.split_at(idx);

                Some((key.trim().to_lowercase().to_string(),
                      val.replacen(":", "", 1).trim().to_string()))
            },
            None => None
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const HEADERS: [&str; 5] = ["Host: localhost:8080",
                                    "Connection: KEEP-alive",
                                    "Cache-Control: max-age=0",
                                    "Accept: text/html,application/xhtml+xml,application/xml;",
                                    "Accept-Encoding: gzip, deflate, br"];

        #[test]
        fn test_to_headers() {
            let mut generated = HashMap::new();
            generated.insert("host".to_string(), "localhost:8080".to_string());
            generated.insert("connection".to_string(), "KEEP-alive".to_string());
            generated.insert("cache-control".to_string(), "max-age=0".to_string());
            generated.insert("accept".to_string(),
                             "text/html,application/xhtml+xml,application/xml;".to_string());
            generated.insert("accept-encoding".to_string(), "gzip, deflate, br".to_string());

            let parsed = to_headers(HEADERS.iter().map(|x| x.to_string()).collect());

            assert_eq!(parsed, generated)
        }
    }
}
