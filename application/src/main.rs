mod config;
mod errors;
mod logging;
mod middleware;
mod program;
mod routes;
use clap::Parser;
use std::process::exit;

fn main() {
    let config = config::Config::parse();
    if let Err(error) = program::run(&config) {
        eprintln!("{}", error);
        exit(1);
    }
}
