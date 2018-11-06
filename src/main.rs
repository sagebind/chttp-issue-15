extern crate chttp;
extern crate env_logger;
extern crate grpcio;

use chttp::{Client, Options, RedirectPolicy};
use std::env;

fn main() {
    env::set_var("RUST_LOG", "chttp=info");
    env_logger::init();

    println!("{}", chttp::version());

    let _env = grpcio::Environment::new(1);

    let client = Client::builder()
        .options(Options::default().with_redirect_policy(RedirectPolicy::Limit(10)))
        .build()
        .unwrap();
    let mut response = client
        .get("https://github.com/pantsbuild/pex/releases/download/v1.5.1/pex27")
        .unwrap();
    if !response.status().is_success() {
        panic!("Well that's no good. Got: {}", response.status());
    }

    std::fs::File::create("pex27.pex")
        .and_then(|mut file| std::io::copy(response.body_mut(), &mut file).map(|_| file))
        .unwrap();
}
