use std::fs::File;

const NEW_LINE: u8 = b'\n';


fn file_from_dry_run(file_path: String, dry_run: bool) -> File {
    if dry_run {
        std::fs::OpenOptions::new()
            .read(true)
            .open(file_path)
            .unwrap()
    } else {
        std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)
            .unwrap()
    }
}

pub mod add;
pub mod remove;
