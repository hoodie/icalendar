#![cfg(feature = "parser")]
use icalendar::{parser::unfold, Calendar};

mod example_utils;
use example_utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(sample) = content_from_arg()? {
        let unfolded = unfold(&sample);
        print_with_lines(&unfolded);

        let parsed_calendar = match sample.parse::<Calendar>() {
            Ok(read) => read,
            Err(error) => {
                println!("{error}"); // println!(error) yields prettier output
                return Ok(());
            }
        };
        println!("{parsed_calendar}");
    }
    Ok(())
}
