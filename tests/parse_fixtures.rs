#![cfg(feature = "parser")]
use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use icalendar::parser::{read_calendar, unfold};

fn with_all_fixtures<F>(
    sub_folder: impl AsRef<Path> ,
    f: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&Path), // -> Result<(), Box<dyn std::error::Error>>,
{
    let fixture_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("fixtures")
        .join(sub_folder);

    for path in fs::read_dir(fixture_path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().and_then(OsStr::to_str) == Some("ics"))
        .map(|entry| entry.path())
    {
        f(&path);
    }

    Ok(())
}

#[test]
fn parse_fixtures_root() {
    with_all_fixtures("", |path| {
        let fixture = std::fs::read_to_string(path).unwrap();
        if let Err(error) = read_calendar(&unfold(&fixture)) {
            println!("{error}");

            panic!("test failed");
        }
    })
    .unwrap();
}

#[test]
fn parse_fixtures_icalendar_rb() {
    with_all_fixtures("icalendar-rb", |path| {
        let fixture = std::fs::read_to_string(path).unwrap();
        if let Err(error) = read_calendar(&unfold(&fixture)) {
            println!("{error}");

            panic!("test failed: {path:?}");
        }
    })
    .unwrap();
}

/// this one is a bit special because the file is not valid utf8
/// but it's nice to know that we can still parse it
#[test]
fn parse_fixtures_icalendar_rb_bad_utf8() {
    eprintln!("these fixtures produce errors");
    with_all_fixtures("icalendar-rb-bad-utf8", |path| {
        // let fixture = std::fs::read_to_string(&path).unwrap();
        let file_content = std::fs::read(path).unwrap();
        let fixture = dbg!(String::from_utf8_lossy(&file_content));

        dbg!(read_calendar(&unfold(&fixture)).unwrap());
    })
    .unwrap();
}
