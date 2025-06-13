use std::env;
use std::process;

use chapter12::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem: {}", err);
        process::exit(1);

    });

    if let Err(e) = chapter12::run(config) {
        eprintln!("App Error : {}", e);
        process::exit(1);
    } 
}
