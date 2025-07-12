#![cfg(feature = "parser")]
use icalendar::parser::unfold;

mod example_utils;
use example_utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(sample) = content_from_arg()? {
        let unfolded = unfold(&sample);
        // print_with_lines(&unfolded);

        match icalendar::parser::read_calendar(&unfolded) {
            Ok(read) => println!("{read}"),
            Err(error) => println!("human-readable error\n{error}"),
        }
    }
    Ok(())
}
