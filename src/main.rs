pub mod cli;
pub mod cmd;

use clap::Parser;
use cli::parser::CLIArguments;
use cmd::{
    add::{
        add_after_pattern, add_before_pattern, add_via_line_number, overwrite_pattern,
        overwrite_via_line_number,
    },
    remove::{
        remove_after_pattern, remove_before_pattern, remove_overwrite_pattern,
        remove_via_line_number,
    },
};

use crate::cli::parser::{
    What::{Add, Remove},
    WhereCommands::{AfterPattern, AtLine, BeforePattern, OverwriteLine, OverwritePattern},
};

fn main() {
    let cli_args = CLIArguments::parse();
    let file_path = cli_args.file;
    let maybe_entry = cli_args.entry;
    let cmd = cli_args.command;
    let dry_run = cli_args.dry_run;

    match (maybe_entry, cmd) {
        (Some(_), Remove(_)) => {
            panic!("Cannot provide an entry to add while using the Remove command.")
        }
        (None, Add(_)) => panic!("Must provide an entry to add while using the Add command."),
        (None, Remove(operation)) => match operation.command {
            AfterPattern(pattern_mut) => {
                remove_after_pattern(file_path, pattern_mut.pattern, dry_run)
            }
            AtLine(line_mut) => remove_via_line_number(file_path, line_mut.line_number, dry_run),
            BeforePattern(pattern_mut) => {
                remove_before_pattern(file_path, pattern_mut.pattern, dry_run)
            }
            OverwritePattern(pattern_mut) => {
                remove_overwrite_pattern(file_path, pattern_mut.pattern, dry_run)
            }
            OverwriteLine(line_mut) => {
                remove_via_line_number(file_path, line_mut.line_number, dry_run)
            }
        },
        (Some(new_entry), Add(operation)) => match operation.command {
            AfterPattern(pattern_mut) => {
                add_after_pattern(file_path, pattern_mut.pattern, new_entry, dry_run)
            }
            AtLine(line_mut) => {
                add_via_line_number(file_path, line_mut.line_number, new_entry, dry_run)
            }
            BeforePattern(pattern_mut) => {
                add_before_pattern(file_path, pattern_mut.pattern, new_entry, dry_run)
            }
            OverwritePattern(pattern_mut) => {
                overwrite_pattern(file_path, pattern_mut.pattern, new_entry, dry_run)
            }
            OverwriteLine(line_mut) => {
                overwrite_via_line_number(file_path, line_mut.line_number, new_entry, dry_run)
            }
        },
    }
}
