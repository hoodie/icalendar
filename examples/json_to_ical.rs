#![cfg(feature = "parser")]
mod example_utils;
use example_utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(content) = content_from_arg()? {
        let calendar = serde_json::from_str::<icalendar::parser::Calendar>(&content)?;
        println!("{calendar}");
    }
    Ok(())
}
