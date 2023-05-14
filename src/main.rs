use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    let config = Config::new(&args, case_sensitive).unwrap_or_else(|_| {
        eprintln!("usage: minigrep [pattern] [filename]");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
