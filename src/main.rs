use cligrep::Config;
use std::env;
use std::process;

fn main() {
    // collect all the args passed on the command line into an iterable
    let args: Vec<String> = env::args().collect();
    // store/save the values in the args
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    if let Err(err) = cligrep::run(config) {
        println!("Application error occurred {}", err);

        process::exit(1);
    }
}