mod config;
mod errors;
mod logging;
mod middleware;
mod program;
mod routes;

use std::process::exit;

fn main() {
    if let Err(error) = program::run() {
        eprintln!("{}", error);
        exit(1);
    }
}
