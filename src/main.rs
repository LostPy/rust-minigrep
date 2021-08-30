use std::process;
use std::env;

use minigrep::Config;


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Fail in reading arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error in running: {}", e);
        process::exit(1);
    }
}

