mod helpers;
mod args;

use args::Cli;
use clap::Parser;
fn main() {
    let args = Cli::parse();

    helpers::print_header();
}
