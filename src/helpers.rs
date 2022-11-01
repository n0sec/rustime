use owo_colors::{OwoColorize, Style};
use time::format_description;
use time::Time;

#[derive(thiserror::Error, Debug)]
pub enum ReadTimesError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error(transparent)]
    TimeError(#[from] time::error::Parse),
}

pub fn print_header() {
    let s = r#"
    __________                __  .__                
    \______   \__ __  _______/  |_|__| _____   ____  
     |       _/  |  \/  ___/\   __\  |/     \_/ __ \ 
     |    |   \  |  /\___ \  |  | |  |  Y Y  \  ___/ 
     |____|_  /____//____  > |__| |__|__|_|  /\___  >
            \/           \/                \/     \/ 
    Converting times because we're lazy."#
        .red();
    let info_style = Style::new().yellow().bold();
    let info = r#"    --------------------------------------
    : Created by: Nick Conklin           :
    : 2022                               :
    --------------------------------------"#
        .style(info_style);
    println!("{}", s);
    println!("{}", info);
}

pub fn convert_times(entered_times: Vec<String>) -> Result<Vec<f64>, time::error::Parse> {
    let format = format_description::parse("[hour]:[minute]")
        .expect("Programming error: Invalid time formatter.");

    entered_times
        .iter()
        .map(|time| Time::parse(time, &format))
        .map(|maybe_parsed_time| maybe_parsed_time.map(|time| time.minute() as f64 / 60.0))
        .collect::<Result<Vec<_>, time::error::Parse>>()
}

pub fn read_file(path_to_file: String) -> Result<Vec<f64>, ReadTimesError> {
    let format = format_description::parse("[hour]:[minute]")
        .expect("Programming error: Invalid time formatter.");

    let binding = std::fs::read_to_string(path_to_file)?;
    let file_lines = binding.lines();

    file_lines
        .into_iter()
        .map(|time| Time::parse(time, &format).map_err(ReadTimesError::from))
        .map(|maybe_parsed_time| maybe_parsed_time.map(|time| time.minute() as f64 / 60.0))
        .collect::<Result<Vec<_>, ReadTimesError>>()
}
