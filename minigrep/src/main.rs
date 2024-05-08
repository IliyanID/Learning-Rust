use std::{env, process};
use minigrep::Config;

fn main() {
    let mut args:Vec<String> = env::args().collect();
    let config = Config::new(&mut args);
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

