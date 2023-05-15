use std::{env, process};

use minigrep::Config;

fn main() {
    let args = env::args();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    let config = Config::new(args, case_sensitive).unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!("usage: minigrep [query] [filename]");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
