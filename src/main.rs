mod args;
mod helpers;

use core::panic;

use args::Cli;
use clap::Parser;
fn main() {
    let args = Cli::parse();

    helpers::print_header();
    let times = helpers::convert_times(vec!["12:05".to_string(), "12:10".to_string()]);

    match times {
        Ok(times) => println!("{:.2?}", times),
        Err(_) => panic!("Error"),
    }
}
