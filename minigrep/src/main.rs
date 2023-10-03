extern crate minigrep;

use std::process;
use minigrep::Config;
use minigrep::args::CliArgs;
use clap::Parser;

fn main() {
    let args: CliArgs = CliArgs::parse();
    println!("{} {}", args.query, args.filename);

    let config = Config::new(args).unwrap_or_else(|err|{
        println!("Problem parsing arg: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        println!("Application error: {}", e);

        process::exit(1);
    }


}


