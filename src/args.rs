use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    /// Don't print the banner and other noise
    #[arg(short, long, global = true, help_heading = "Flags")]
    pub quiet: bool,

    /// Disable color output
    #[arg(long, global = true, help_heading = "Flags")]
    pub no_color: bool,

    /// Don't display progress
    #[arg(short = 'z', long, global = true, help_heading = "Flags")]
    pub no_progress: bool,

    /// Output file to write results to
    #[arg(short, long, global = true, help_heading = "Flags")]
    pub output: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Parses times into decimal
    Time(TimeOptions),
}

#[derive(Args, Debug)]
pub struct TimeOptions {
    /// The times to convert (multiple times separated by a comma)
    #[arg(short, long, value_delimiter = ',', conflicts_with="file", group="time")]
    pub times: Option<Vec<String>>,

    /// The input file to read and parse
    #[arg(short, long, group="input")]
    pub file: Option<String>,
}
