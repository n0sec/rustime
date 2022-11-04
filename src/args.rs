use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct CliOptions {
    /// The times to convert written in hh:mm format (multiple times separated by a comma)
    #[arg(short, long, value_delimiter = ',', conflicts_with="file", required_unless_present="file", group="time")]
    pub times: Vec<String>,

    /// The input file to read and parse
    #[arg(short, long, group="input")]
    pub file: Option<String>,

    /// Don't print the banner and other noise
    #[arg(short, long, global = true, help_heading = "Flags")]
    pub quiet: bool,

    /// Don't display progress
    #[arg(short = 'z', long, global = true, help_heading = "Flags")]
    pub no_progress: bool,

    /// Output file to write results to
    #[arg(short, long, global = true, help_heading = "Flags")]
    pub output: Option<String>,
}
