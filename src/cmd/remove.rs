use regex::Regex;
use std::{
    fs::{self},
    io::Write,
};

use super::{NEW_LINE, file_from_dry_run};

// target/release/mute --dry-run tests/simple.toml remove after-pattern "^entry_"
// target/release/mute tests/simple.toml remove after-pattern "^entry_"
pub fn remove_after_pattern(file_path: String, pattern: String, dry_run: bool) {
    let contents = fs::read_to_string(&file_path).unwrap();
    let mut line_skipped = false;
    let mut skip_next = false;
    let mut file = file_from_dry_run(file_path, dry_run);

    let regex = Regex::new(pattern.as_str()).unwrap();
    let mut line_iter = contents.split('\n').enumerate().peekable();

    while let Some((index, line)) = line_iter.next() {
        if skip_next {
            skip_next = false;
            line_skipped = true;
            if dry_run {
                println!("(Line: {})\t --- {}", index + 1, line);
            }
        } else if !dry_run {
            file.write_all(line.as_bytes()).unwrap();
            if line_iter.peek().is_some() {
                file.write_all(&[NEW_LINE]).unwrap();
            }
        }

        if regex.is_match(line) {
            skip_next = true;
        }
    }

    file.flush().unwrap();
    if !line_skipped {
        println!("WARNING: Pattern was not found. Please check the file, the regex and try again.");
    }
}

// target/release/mute tests/simple.toml remove at-line 4
// target/release/mute --dry-run tests/simple.toml remove at-line 4
pub fn remove_via_line_number(file_path: String, line_no: usize, dry_run: bool) {
    assert_ne!(
        line_no, 0,
        "Line numbers are 1-indexed. Cannot have a line 0."
    );

    let contents = fs::read_to_string(&file_path).unwrap();
    let mut line_skipped = false;
    let mut file = file_from_dry_run(file_path, dry_run);
    let mut line_iter = contents.split('\n').enumerate().peekable();

    while let Some((index, line)) = line_iter.next() {
        if index + 1 == line_no {
            line_skipped = true;
            if dry_run {
                println!("(Line: {})\t --- {}", line_no, line);
            }
        } else if !dry_run {
            file.write_all(line.as_bytes()).unwrap();
            if line_iter.peek().is_some() {
                file.write_all(&[NEW_LINE]).unwrap();
            }
        }
    }

    file.flush().unwrap();
    if !line_skipped {
        println!("WARNING: Line was not found. Please check the file and try again.");
    }
}

// target/release/mute --dry-run tests/simple.toml remove before-pattern "^entry_"
// target/release/mute tests/simple.toml remove before-pattern "^entry_"
pub fn remove_before_pattern(file_path: String, pattern: String, dry_run: bool) {
    let contents = fs::read_to_string(&file_path).unwrap();
    let mut line_skipped = false;
    let mut skip_last = false;
    let mut line_buffer = "";

    let mut file = file_from_dry_run(file_path, dry_run);

    let regex = Regex::new(pattern.as_str()).unwrap();

    let mut line_iter = contents.split('\n').enumerate().peekable();
    while let Some((index, line)) = line_iter.next() {
        if regex.is_match(line) && index > 0 {
            skip_last = true;
        }

        if skip_last {
            skip_last = false;
            line_skipped = true;
            if dry_run {
                println!("(Line: {})\t --- {}", index + 1, line_buffer);
            }
        } else if !dry_run {
            file.write_all(line_buffer.as_bytes()).unwrap();
            if line_iter.peek().is_some() {
                file.write_all(&[NEW_LINE]).unwrap();
            }
        }

        line_buffer = line;
    }

    if !dry_run {
        file.write_all(line_buffer.as_bytes()).unwrap();
        file.write_all(&[NEW_LINE]).unwrap();
    }

    file.flush().unwrap();
    if !line_skipped {
        println!("WARNING: Pattern was not found. Please check the file, the regex and try again.");
    }
}


// target/release/mute --dry-run tests/simple.toml remove overwrite-pattern "^entry_"
// target/release/mute tests/simple.toml remove overwrite-pattern "^entry_"
pub fn remove_overwrite_pattern(file_path: String, pattern: String, dry_run: bool) {
    let contents = fs::read_to_string(&file_path).unwrap();
    let mut line_skipped = false;
    let mut skip_this = false;
    let mut file = file_from_dry_run(file_path, dry_run);

    let regex = Regex::new(pattern.as_str()).unwrap();

    let mut line_iter = contents.split('\n').enumerate().peekable();
    while let Some((index, line)) = line_iter.next() {
        if regex.is_match(line) {
            line_skipped = true;
            skip_this = true;
        }

        if skip_this {
            skip_this = false;
            if dry_run {
                println!("(Line: {})\t --- {}", index + 1, line);
            }
        } else if !dry_run {
            file.write_all(line.as_bytes()).unwrap();
            if line_iter.peek().is_some() {
                file.write_all(&[NEW_LINE]).unwrap();
            }
        }
    }

    file.flush().unwrap();
    if !line_skipped {
        println!("WARNING: Pattern was not found. Please check the file, the regex and try again.");
    }
}