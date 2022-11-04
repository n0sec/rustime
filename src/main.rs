mod args;
mod helpers;

use args::CliOptions;
use clap::Parser;
use helpers::*;
fn main() {
    let args = CliOptions::parse();

    if !args.times.is_empty() {
        if let false = args.quiet {
            print_header();
        }
        let converted_times = convert_times(args.times);
        let results = pretty_print_results(converted_times);
        match results {
            Ok(_) => (),
            Err(e) => println!("{}", e)
        }
    }

    if let Some(filename) = args.file {
        if let false = args.quiet {
            print_header();
        }
        let converted_times = read_file(filename);
        let results = pretty_print_results(converted_times);
        match results {
            Ok(_) => (),
            Err(e) => println!("{}", e)
        }
    }

    // if let Some(filename) = args.output {
    //     write_file(args.times, filename);
    // }


}
