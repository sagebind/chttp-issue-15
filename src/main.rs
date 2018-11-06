extern crate curl;
extern crate grpcio_sys;

use curl::easy::Easy;

fn main() {
    let mut easy = Easy::new();
    easy.url("https://www.rust-lang.org/").unwrap();
    easy.perform().unwrap();

    unsafe {
        // Not called, just present to prevent dead code elimination.
        grpcio_sys::grpc_init();
    }
}
