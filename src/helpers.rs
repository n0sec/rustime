use owo_colors::OwoColorize;
use time::Time;
use time::error::Parse;
use time::format_description;

pub fn print_header() {
    let s = r#"
    __________                __  .__                
    \______   \__ __  _______/  |_|__| _____   ____  
     |       _/  |  \/  ___/\   __\  |/     \_/ __ \ 
     |    |   \  |  /\___ \  |  | |  |  Y Y  \  ___/ 
     |____|_  /____//____  > |__| |__|__|_|  /\___  >
            \/           \/                \/     \/ 
    "#.red();
    println!("{}", s);
}

pub fn convert_time(entered_time: &str) -> f64 {
    let format = format_description::parse("[hour]:[minute]").unwrap();
    let time = Time::parse(entered_time, &format).expect("Incorrect time format");
    
    let minutes = time.minute() as f64;
    let decimal_minutes: f64 = minutes / 60.0;

    decimal_minutes
}

