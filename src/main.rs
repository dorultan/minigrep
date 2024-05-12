use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Error while building config: {error}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
