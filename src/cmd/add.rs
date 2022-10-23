use regex::Regex;
use std::{
    fs::{self},
    io::Write, process,
};

use super::NEW_LINE;
use crate::cmd::file_from_dry_run;

pub fn add_after_pattern(file_path: String, pattern: String, entry: String, dry_run: bool) {
    let contents = fs::read_to_string(&file_path).unwrap();
    let mut this_line = false;
    let mut lines_found = 0;
    let mut file = file_from_dry_run(file_path, dry_run);

    let regex = Regex::new(pattern.as_str()).unwrap();

    let mut line_iter = contents.split('\n').enumerate().peekable();
    while let Some((index, line)) = line_iter.next() {
        if regex.is_match(line) {
            lines_found += 1;
            this_line = true;
        }
        if !dry_run {
            file.write_all(line.as_bytes()).unwrap();

            if this_line {
                file.write_all(&[NEW_LINE]).unwrap();
                file.write_all(entry.as_bytes()).unwrap();
                this_line = false;
            }

            if line_iter.peek().is_some() {
                file.write_all(&[NEW_LINE]).unwrap();
            }
        } else if this_line {
            println!("(Line: {})\t --- {}", index + 1 + lines_found, entry);
            this_line = false;
        }
    }
    file.flush().unwrap();
    if lines_found == 0 {
        println!("WARNING: Pattern was not found. Please check the file, the regex and try again.");
        process::exit(exitcode::DATAERR);
    }
}

pub fn add_via_line_number(file_path: String, line_no: usize, entry: String, dry_run: bool) {
    assert_ne!(
        line_no, 0,
        "Line numbers are 1-indexed. Cannot have a line 0."
    );

    let contents = fs::read_to_string(&file_path).unwrap();
    let mut line_found = false;
    let mut file = file_from_dry_run(file_path, dry_run);

    let mut line_iter = contents.split('\n').enumerate().peekable();
    while let Some((index, line)) = line_iter.next() {
        if index + 1 == line_no {
            line_found = true;
            if dry_run {
                println!("(Line: {})\t --- {}", line_no, line);
                println!("(Line: {})\t +++ {}", line_no, entry);
                println!("(Line: {})\t +++ {}", line_no + 1, line);
            } else {
                file.write_all(entry.as_bytes()).unwrap();
                file.write_all(&[NEW_LINE]).unwrap();
            }
        }
        if !dry_run {
            file.write_all(line.as_bytes()).unwrap();
            if line_iter.peek().is_some() {
                file.write_all(&[NEW_LINE]).unwrap();
            }
        }
    }

    file.flush().unwrap();
    if !line_found {
        println!("WARNING: Line was not found. Please check the file and try again.");
        process::exit(exitcode::DATAERR);
    }
}

pub fn add_before_pattern(file_path: String, pattern: String, entry: String, dry_run: bool) {
    let contents = fs::read_to_string(&file_path).unwrap();
    let mut this_line = false;
    let mut lines_found = 0;
    let mut file = file_from_dry_run(file_path, dry_run);

    let regex = Regex::new(pattern.as_str()).unwrap();

    let mut line_iter = contents.split('\n').enumerate().peekable();
    while let Some((index, line)) = line_iter.next() {
        if regex.is_match(line) {
            lines_found += 1;
            this_line = true;
        }
        if !dry_run {
            if this_line {
                file.write_all(entry.as_bytes()).unwrap();
                file.write_all(&[NEW_LINE]).unwrap();
                this_line = false;
            }
            file.write_all(line.as_bytes()).unwrap();
            if line_iter.peek().is_some() {
                file.write_all(&[NEW_LINE]).unwrap();
            }
        } else if this_line {
            println!("(Line: {})\t --- {}", index + lines_found, entry);
            this_line = false;
        }
    }
    file.flush().unwrap();
    if lines_found == 0 {
        println!("WARNING: Pattern was not found. Please check the file, the regex and try again.");
        process::exit(exitcode::DATAERR);
    }
}

pub fn overwrite_via_line_number(file_path: String, line_no: usize, entry: String, dry_run: bool) {
    assert_ne!(
        line_no, 0,
        "Line numbers are 1-indexed. Cannot have a line 0."
    );

    let contents = fs::read_to_string(&file_path).unwrap();
    let mut line_found = false;
    let mut file = file_from_dry_run(file_path, dry_run);

    let mut line_iter = contents.split('\n').enumerate().peekable();
    while let Some((index, line)) = line_iter.next() {
        if index + 1 == line_no {
            line_found = true;
            if dry_run {
                println!("(Line: {})\t --- {}", line_no, line);
                println!("(Line: {})\t +++ {}", line_no, entry);
            } else {
                file.write_all(entry.as_bytes()).unwrap();
                if line_iter.peek().is_some() {
                    file.write_all(&[NEW_LINE]).unwrap();
                }
            }
        } else if !dry_run {
            file.write_all(line.as_bytes()).unwrap();
            if line_iter.peek().is_some() {
                file.write_all(&[NEW_LINE]).unwrap();
            }
        }
    }

    file.flush().unwrap();
    if !line_found {
        println!("WARNING: Line was not found. Please check the file and try again.");
        process::exit(exitcode::DATAERR);
    }
}

pub fn overwrite_pattern(file_path: String, pattern: String, entry: String, dry_run: bool) {
    let contents = fs::read_to_string(&file_path).unwrap();
    let mut this_line = false;
    let mut found_line = false;
    let mut file = file_from_dry_run(file_path, dry_run);

    let regex = Regex::new(pattern.as_str()).unwrap();

    let mut line_iter = contents.split('\n').enumerate().peekable();
    while let Some((index, line)) = line_iter.next() {
        if regex.is_match(line) {
            found_line = true;
            this_line = true;
        }
        if !dry_run {
            if this_line {
                file.write_all(entry.as_bytes()).unwrap();
                if line_iter.peek().is_some() {
                    file.write_all(&[NEW_LINE]).unwrap();
                }
                this_line = false;
            } else {
                file.write_all(line.as_bytes()).unwrap();
                if line_iter.peek().is_some() {
                    file.write_all(&[NEW_LINE]).unwrap();
                }
            }
        } else if this_line {
            println!("(Line: {})\t --- {}", index + 1, line);
            println!("(Line: {})\t +++ {}", index + 1, entry);
            this_line = false;
        }
    }
    file.flush().unwrap();
    if !found_line {
        println!("WARNING: Pattern was not found. Please check the file, the regex and try again.");
        process::exit(exitcode::DATAERR);
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use tempfile::NamedTempFile;

    use crate::cmd::add::{
        add_after_pattern, add_before_pattern, add_via_line_number, overwrite_pattern,
        overwrite_via_line_number,
    };

    const FAUX_FILE: &str = "[table]\n\
    [[subtable1]]\n\
    entry_1=\"one\"\n\
    entry_2=\"two\"\n\
    \n\
    [[subtable2]]\n\
    entry_4=\"four\"";

    #[test]
    fn test_add_after_pattern() {
        // create test file
        let mut file1 = NamedTempFile::new().unwrap();
        file1.write_all(FAUX_FILE.as_bytes()).unwrap();
        file1.flush().unwrap();
        let file_path = file1.path().to_str().unwrap().to_owned();
        // mutate file
        add_after_pattern(
            file_path,
            "^\\[\\[subtable2]]".to_owned(),
            "check=1".to_owned(),
            false,
        );
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
        [[subtable2]]\n\
        check=1\n\
        entry_4=\"four\"";
        assert_eq!(expected, mutated_contents);
    }

    #[test]
    fn test_add_via_linenumber() {
        // create test file
        let mut file1 = NamedTempFile::new().unwrap();
        file1.write_all(FAUX_FILE.as_bytes()).unwrap();
        file1.flush().unwrap();
        let file_path = file1.path().to_str().unwrap().to_owned();
        // mutate file
        add_via_line_number(file_path, 3, "entry_0=\"zero\"".to_owned(), false);
        // read in file
        let mut mutated_contents = String::new();
        let mut file2 = file1.reopen().unwrap();
        file2.read_to_string(&mut mutated_contents).unwrap();
        // compare results
        let expected = "[table]\n\
        [[subtable1]]\n\
        entry_0=\"zero\"\n\
        entry_1=\"one\"\n\
        entry_2=\"two\"\n\
        \n\
        [[subtable2]]\n\
        entry_4=\"four\"";
        assert_eq!(expected, mutated_contents);
    }

    #[test]
    fn test_add_before_pattern() {
        // create test file
        let mut file1 = NamedTempFile::new().unwrap();
        file1.write_all(FAUX_FILE.as_bytes()).unwrap();
        file1.flush().unwrap();
        let file_path = file1.path().to_str().unwrap().to_owned();
        // mutate file
        add_before_pattern(
            file_path,
            "^entry_1".to_owned(),
            "entry_0=\"zero\"".to_owned(),
            false,
        );
        // read in file
        let mut mutated_contents = String::new();
        let mut file2 = file1.reopen().unwrap();
        file2.read_to_string(&mut mutated_contents).unwrap();
        // compare results
        let expected = "[table]\n\
        [[subtable1]]\n\
        entry_0=\"zero\"\n\
        entry_1=\"one\"\n\
        entry_2=\"two\"\n\
        \n\
        [[subtable2]]\n\
        entry_4=\"four\"";
        assert_eq!(expected, mutated_contents);
    }

    #[test]
    fn test_overwrite_line() {
        // create test file
        let mut file1 = NamedTempFile::new().unwrap();
        file1.write_all(FAUX_FILE.as_bytes()).unwrap();
        file1.flush().unwrap();
        let file_path = file1.path().to_str().unwrap().to_owned();
        // mutate file
        overwrite_via_line_number(file_path, 1, "[tabby]".to_owned(), false);
        // read in file
        let mut mutated_contents = String::new();
        let mut file2 = file1.reopen().unwrap();
        file2.read_to_string(&mut mutated_contents).unwrap();
        // compare results
        let expected = "[tabby]\n\
        [[subtable1]]\n\
        entry_1=\"one\"\n\
        entry_2=\"two\"\n\
        \n\
        [[subtable2]]\n\
        entry_4=\"four\"";
        assert_eq!(expected, mutated_contents);
    }

    #[test]
    fn test_overwrite_pattern() {
        // create test file
        let mut file1 = NamedTempFile::new().unwrap();
        file1.write_all(FAUX_FILE.as_bytes()).unwrap();
        file1.flush().unwrap();
        let file_path = file1.path().to_str().unwrap().to_owned();
        // mutate file
        overwrite_pattern(
            file_path,
            "^\\[table".to_owned(),
            "[tabby]".to_owned(),
            false,
        );
        // read in file
        let mut mutated_contents = String::new();
        let mut file2 = file1.reopen().unwrap();
        file2.read_to_string(&mut mutated_contents).unwrap();
        // compare results
        let expected = "[tabby]\n\
        [[subtable1]]\n\
        entry_1=\"one\"\n\
        entry_2=\"two\"\n\
        \n\
        [[subtable2]]\n\
        entry_4=\"four\"";
        assert_eq!(expected, mutated_contents);
    }
}
