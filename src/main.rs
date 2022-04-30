use cligrep::Config;
use std::env;
use std::process;

fn main() {
    // collect all the args passed on the command line into an iterable
    // store/save the values in the args
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    if let Err(err) = cligrep::run(config) {
        eprintln!("Application error occurred {}", err);

        process::exit(1);
    }
}
