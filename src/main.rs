use std::{env, process};

use mygrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for \x1b[34m{}\x1b[0m", config.pattern);
    println!("In file \x1b[34m{}\x1b[0m\n", config.file_path);

    if let Err(e) = mygrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
