use regex::Regex;
use std::{
    fs::{self},
    io::Write,
};

use super::{file_from_dry_run, NEW_LINE};

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

    let line_iter = contents.split('\n').enumerate().peekable();
    for (index, line) in line_iter {
        if regex.is_match(line) && index > 0 {
            skip_last = true;
        }

        if skip_last {
            skip_last = false;
            line_skipped = true;
            if dry_run {
                println!("(Line: {})\t --- {}", index, line_buffer);
            }
        } else if !dry_run && index > 0 {
            file.write_all(line_buffer.as_bytes()).unwrap();
            file.write_all(&[NEW_LINE]).unwrap();
        }
        line_buffer = line;
    }

    if !dry_run {
        file.write_all(line_buffer.as_bytes()).unwrap();
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

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use tempfile::NamedTempFile;

    use crate::cmd::remove::{
        remove_before_pattern, remove_overwrite_pattern, remove_via_line_number,
    };

    use super::remove_after_pattern;

    const FAUX_FILE: &str = "[table]\n\
    [[subtable1]]\n\
    entry_1=\"one\"\n\
    entry_2=\"two\"\n\
    \n\
    [[subtable2]]\n\
    entry_4=\"four\"";

    #[test]
    fn test_rm_after_pattern() {
        // create test file
        let mut file1 = NamedTempFile::new().unwrap();
        file1.write_all(FAUX_FILE.as_bytes()).unwrap();
        file1.flush().unwrap();
        let file_path = file1.path().to_str().unwrap().to_owned();
        // mutate file
        remove_after_pattern(file_path, "^\\[\\[subtable2]]".to_owned(), false);
        // read in file
        let mut mutated_contents = String::new();
        let mut file2 = file1.reopen().unwrap();
        file2.read_to_string(&mut mutated_contents).unwrap();
        // compare results
        let expected = "[table]\n\
        [[subtable1]]\n\
        entry_1=\"one\"\n\
        entry_2=\"two\"\n\
        \n\
        [[subtable2]]\n";
        assert_eq!(expected, mutated_contents);
    }

    #[test]
    fn test_rm_via_line_number() {
        // create test file
        let mut file1 = NamedTempFile::new().unwrap();
        file1.write_all(FAUX_FILE.as_bytes()).unwrap();
        file1.flush().unwrap();
        let file_path = file1.path().to_str().unwrap().to_owned();
        // mutate file
        remove_via_line_number(file_path, 1, false);
        // read in file
        let mut mutated_contents = String::new();
        let mut file2 = file1.reopen().unwrap();
        file2.read_to_string(&mut mutated_contents).unwrap();
        // compare results
        let expected = "[[subtable1]]\n\
        entry_1=\"one\"\n\
        entry_2=\"two\"\n\
        \n\
        [[subtable2]]\n\
        entry_4=\"four\"";
        assert_eq!(expected, mutated_contents);
    }

    #[test]
    fn test_rm_before_pattern() {
        // create test file
        let mut file1 = NamedTempFile::new().unwrap();
        file1.write_all(FAUX_FILE.as_bytes()).unwrap();
        file1.flush().unwrap();
        let file_path = file1.path().to_str().unwrap().to_owned();
        // mutate file
        remove_before_pattern(file_path, "^\\[\\[subtable1]]".to_owned(), false);
        // read in file
        let mut mutated_contents = String::new();
        let mut file2 = file1.reopen().unwrap();
        file2.read_to_string(&mut mutated_contents).unwrap();
        // compare results
        let expected = "[[subtable1]]\n\
        entry_1=\"one\"\n\
        entry_2=\"two\"\n\
        \n\
        [[subtable2]]\n\
        entry_4=\"four\"";
        assert_eq!(expected, mutated_contents);
    }

    #[test]
    fn test_rm_overwrite_pattern() {
        // create test file
        let mut file1 = NamedTempFile::new().unwrap();
        file1.write_all(FAUX_FILE.as_bytes()).unwrap();
        file1.flush().unwrap();
        let file_path = file1.path().to_str().unwrap().to_owned();
        // mutate file
        remove_overwrite_pattern(file_path, "^\\[table]".to_owned(), false);
        // read in file
        let mut mutated_contents = String::new();
        let mut file2 = file1.reopen().unwrap();
        file2.read_to_string(&mut mutated_contents).unwrap();
        // compare results
        let expected = "[[subtable1]]\n\
        entry_1=\"one\"\n\
        entry_2=\"two\"\n\
        \n\
        [[subtable2]]\n\
        entry_4=\"four\"";
        assert_eq!(expected, mutated_contents);
    }
}
