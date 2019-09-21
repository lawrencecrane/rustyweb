use std::io::IoSlice;

pub fn ok(msg: &str) -> [IoSlice; 2] {
    [IoSlice::new("HTTP/1.1 200 OK\n\n".as_bytes()),
     IoSlice::new(msg.as_bytes())]
}
