#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use bindings::wasi::cli::environment::{get_arguments, get_environment};
use clap::Parser;

struct Component;

bindings::export!(Component with_types_in bindings);

#[derive(Parser, Debug)]
struct Args {
    /// Name
    #[arg(short, long)]
    name: String,
}

impl Guest for Component {
    fn run() -> Result<(), ()> {
        println!("args: {:?}\nenvs: {:?}", get_arguments(), get_environment());

        let args = Args::parse();
        println!("name: {}", args.name);
        Ok(())
    }
}
