extern crate umlparser;

use std::env;
use std::process;

use umlparser::Config;

fn main() {
//Konfigurationsvariablen werden eingelesen:
	let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = umlparser::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

}