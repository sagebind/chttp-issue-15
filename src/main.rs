extern crate curl;
extern crate grpcio;

use curl::easy::Easy;

fn main() {
    let _env = grpcio::Environment::new(1);

    let mut easy = Easy::new();
    easy.url("https://www.rust-lang.org/").unwrap();
    easy.perform().unwrap();
}
