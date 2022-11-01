use owo_colors::{OwoColorize, Style};
use time::format_description;
use time::Time;
use comfy_table::Table;

use crate::args::Cli;
use crate::args::Commands;

#[derive(thiserror::Error, Debug)]
pub enum ReadTimesError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error(transparent)]
    TimeError(#[from] time::error::Parse),
}

pub fn print_header(args: &Cli) {
    let title_style = Style::new()
    .red();
    let info_style = Style::new().yellow().bold();

    let s = r#"
    __________                __  .__                
    \______   \__ __  _______/  |_|__| _____   ____  
     |       _/  |  \/  ___/\   __\  |/     \_/ __ \ 
     |    |   \  |  /\___ \  |  | |  |  Y Y  \  ___/ 
     |____|_  /____//____  > |__| |__|__|_|  /\___  >
            \/           \/                \/     \/ 
    Converting times because we're lazy."#;
    let info = r#"    --------------------------------------
    : Created by: Nick Conklin           :
    : 2022                               :
    --------------------------------------"#;

    match args.no_color {
        true => {
            println!("{}", s);
            println!("{}", info);
        },
        false => {
            println!("{}", s.style(title_style));
            println!("{}", info.style(info_style));
        },
    }
}

// TODO: Needs to also return the entered times lol
pub fn convert_times(entered_times: Vec<String>) -> Result<Vec<f64>, time::error::Parse> {
    let format = format_description::parse("[hour]:[minute]")
        .expect("Programming error: Invalid time formatter.");

    entered_times
        .iter()
        .map(|time| Time::parse(time, &format))
        .map(|maybe_parsed_time| maybe_parsed_time.map(|time| (time.minute() as f64 / 60.0) + time.hour() as f64))
        .collect::<Result<Vec<_>, time::error::Parse>>()
}

pub fn read_file(path_to_file: String) -> Result<Vec<f64>, ReadTimesError> {
    let format = format_description::parse("[hour]:[minute]")
        .expect("Programming error: Invalid time formatter.");

    let file = std::fs::read_to_string(path_to_file)?;

    file.lines()
        .map(|time| Time::parse(time, &format).map_err(ReadTimesError::from))
        .map(|maybe_parsed_time| maybe_parsed_time.map(|time| time.minute() as f64 / 60.0))
        .collect::<Result<Vec<_>, ReadTimesError>>()
}

// TODO: Pretty Print Table of Results
pub fn pretty_print_results(results: Vec<f64>) -> () {
    // Create the table
    let mut table = Table::new();

    // Set the header before adding rows
    table.set_header(vec!["Times", "Converted Output"]);

    // Loop over the Vector of results and create rows of the times and converted output
    for result in results {
        table.add_row(vec!["12:05", &result.to_string()]);
    }

    // Print the table
    println!("{table}");
}
