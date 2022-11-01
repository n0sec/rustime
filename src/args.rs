use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
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
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Parses times into decimal
    Time(Time),
}

#[derive(Args)]
pub struct Time {
    /// The times to convert (multiple times separated by a comma)
    #[arg(short, long, value_delimiter = ',', conflicts_with="file", required = true)]
    pub times: Vec<String>,

    /// The input file to read and parse
    #[arg(short, long)]
    pub file: Option<String>,
}
