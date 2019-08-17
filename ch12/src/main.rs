use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let config = Config::from_argv(&argv).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });
    if let Err(e) = minigrep::run(&config) {
        eprintln!("Runtime error: {}", e);
        process::exit(1)
    };
}

