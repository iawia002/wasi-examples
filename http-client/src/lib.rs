#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use std::time::Duration;
use wasi_http_client::Client;

struct Component;

bindings::export!(Component with_types_in bindings);

impl Guest for Component {
    fn run() -> Result<(), ()> {
        let resp = Client::new()
            .post("https://httpbin.org/post")
            .body("hello".as_bytes())
            .connect_timeout(Duration::from_secs(5))
            .send()
            .unwrap();
        println!("status code: {}", resp.status());
        for (key, value) in resp.headers() {
            println!("{key}: {value}");
        }
        println!("body:\n{}", String::from_utf8(resp.body().clone()).unwrap());
        Ok(())
    }
}
