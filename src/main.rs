mod args;
mod helpers;

use args::Cli;
use args::Commands;
use clap::Parser;
use helpers::print_header;
use helpers::read_file;
use helpers::{pretty_print_results, convert_times};
fn main() {
    let args = Cli::parse();

    match args.quiet {
        true => {},
        false => print_header(&args),
    }

    match args.command {
        Commands::Time(options) => {
            match (options.times, options.file) {
                (Some(times), _) => {
                    let converted_times = convert_times(times);
                    pretty_print_results(converted_times);
                },
                (_, Some(path_to_file)) => {
                    let converted_times = read_file(path_to_file);
                    pretty_print_results(converted_times);
                },
                (None, None) => println!("No options detected"),
            }
        }
    }

}
