use chrono::Utc;
use comfy_table::Table;
use comfy_table::*;
use owo_colors::{OwoColorize, Style};
use std::fs::File;
use std::io::Write;
use time::format_description;
use time::Time;

use crate::args::Cli;

#[derive(thiserror::Error, Debug)]
pub enum ReadTimesError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error(transparent)]
    TimeError(#[from] time::error::Parse),
}

pub fn print_header(args: &Cli) {
    let title_style = Style::new().red();
    let info_style = Style::new().bright_red().bold();

    let s = r#"
    __________                __  .__                
    \______   \__ __  _______/  |_|__| _____   ____  
     |       _/  |  \/  ___/\   __\  |/     \_/ __ \ 
     |    |   \  |  /\___ \  |  | |  |  Y Y  \  ___/ 
     |____|_  /____//____  > |__| |__|__|_|  /\___  >
            \/           \/                \/     \/ 
    Converting times because we're lazy."#;
    let info = r#"    
    --------------------------------------
    : Created by: Nick Conklin           :
    : 2022                               :
    --------------------------------------"#;

    match args.no_color {
        true => {
            println!("{}", s);
            println!("{}", info);
        }
        false => {
            println!("{}", s.style(title_style));
            println!("{}", info.style(info_style));
        }
    }
}

pub fn convert_times(entered_times: Vec<String>) -> Result<Vec<f64>, time::error::Parse> {
    let format = format_description::parse("[hour]:[minute]")
        .expect("Programming error: Invalid time formatter.");

    entered_times
        .iter()
        .map(|time| Time::parse(time, &format))
        .map(|maybe_parsed_time| {
            maybe_parsed_time.map(|time| (time.minute() as f64 / 60.0) + time.hour() as f64)
        })
        .collect::<Result<Vec<_>, time::error::Parse>>()
}

pub fn read_file(path_to_file: String) -> Result<Vec<(String, f64)>, ReadTimesError> {
    let format = format_description::parse("[hour]:[minute]")
        .expect("Programming error: Invalid time formatter.");

    let file = std::fs::read_to_string(path_to_file)?;

    file.lines()
        .map(|time| {
            (
                time,
                Time::parse(time, &format).map_err(ReadTimesError::from),
            )
        })
        .map(|(time, parsed)| parsed.map(|parsed| (time.to_owned(), parsed.minute() as f64 / 60.0)))
        .collect::<Result<Vec<_>, ReadTimesError>>()
}

pub fn write_file(entered_times: Vec<String>) -> Result<File, ReadTimesError> {
    // Create the file name from a timestamp of UTC::now()
    let now = Utc::now().format("%m-%d-%Y_%s");
    let filename = format!("{now}.txt");

    // Create the file
    let mut file = File::create(filename)?;

    // Write to the file
    for time in entered_times {
        file.write_all(time.as_bytes())?;
    }

    // Return the file
    Ok(file)
}

// TODO: Pretty Print Table of Results
pub fn pretty_print_results(results: Vec<f64>) -> () {
    // Create the table
    let mut table = Table::new();

    // Set the header before adding rows
    table.set_header(vec![
        Cell::new("Times")
            .fg(Color::Blue)
            .add_attribute(Attribute::Bold),
        Cell::new("Converted Output")
            .fg(Color::Blue)
            .add_attribute(Attribute::Bold),
    ]);

    // Loop over the Vector of results and create rows of the times and converted output
    for result in results {
        table.add_row(vec!["12:05", &result.to_string()]);
    }

    // Print the table
    println!("{table}");
}
