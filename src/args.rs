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
    /// Uses directory/file enumeration mode
    Time(Time),
}

#[derive(Args)]
pub struct Time {

}
