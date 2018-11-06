extern crate grpcio_sys;
extern crate openssl;

use openssl::ssl::{SslMethod, SslConnector};
use std::io::Write;
use std::net::TcpStream;

fn main() {
    // Connect using libssl.
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
    let stream = TcpStream::connect("google.com:443").unwrap();
    let mut stream = connector.connect("google.com", stream).unwrap();
    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();

    unsafe {
        // Not called, just present to prevent dead code elimination.
        grpcio_sys::grpc_init();
    }
}
