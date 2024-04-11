#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use bindings::wasi::cli::environment::{get_arguments, get_environment};

struct Component;

bindings::export!(Component with_types_in bindings);

impl Guest for Component {
    fn run() -> Result<(), ()> {
        println!("args: {:?}\nenvs: {:?}", get_arguments(), get_environment());
        Ok(())
    }
}
