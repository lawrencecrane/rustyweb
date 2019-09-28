pub mod websocket {
    use std::net::TcpStream;
    use std::io::{Read, BufReader, Error, ErrorKind};
    use std::convert::TryInto;

    use crate::http::websocket::{Opcode, Header};

    pub fn parse(stream: &TcpStream) -> Result<Vec<u8>, Error> {
        let mut reader = BufReader::new(stream);
        let mut header_buf = [0; 2];

        reader.by_ref().take(2).read(&mut header_buf).unwrap();

        let header = parse_header(header_buf).unwrap();
        let payload_length = get_actual_payload_length(&header, &mut reader).unwrap();

        match header.opcode {
            Opcode::TEXT => {
                let mut buffer = create_payload_buffer(payload_length);
                reader.read_exact(&mut buffer).unwrap();

                Ok(buffer)
            },
            // TODO: should not actually be error
            Opcode::CLOSE => Err(Error::new(ErrorKind::ConnectionRefused, ""))
        }
    }

    pub fn create_payload_buffer(payload_length: u64) -> Vec<u8> {
        vec![0; ((payload_length as f64 / 8.0).ceil() as u8).try_into().unwrap()]
    }

    fn get_actual_payload_length(header: &Header, reader: &mut BufReader<&TcpStream>)
                                 -> Result<u64, Error> {
        match header.payload_length {
            length if length <= 125 => Ok(length.into()),
            length if length == 126 => {
                let mut payload_buf = [0; 2];
                reader.by_ref().take(2).read(&mut payload_buf).unwrap();

                Ok(u16::from_be_bytes(payload_buf).into())
            },
            length if length == 127 => {
                let mut payload_buf = [0; 8];
                reader.by_ref().take(8).read(&mut payload_buf).unwrap();

                Ok(u64::from_be_bytes(payload_buf))
            },
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid payload length"))
        }
    }

    fn parse_header(header: [u8; 2]) -> Result<Header, Error> {
        let is_final_frame = (header[0] >> 7) == 1;
        let opcode = header[0] & 0xF;
        let is_masked = (header[1] >> 7) == 1;
        let payload_length = header[1] & 0x7F;

        match opcode {
            1 => Ok(Header::new(is_final_frame,
                                Opcode::TEXT,
                                is_masked,
                                payload_length.into())),
            8 => Ok(Header::new(is_final_frame,
                                Opcode::CLOSE,
                                is_masked,
                                payload_length)),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Bad opcode"))
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
