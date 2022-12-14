# Mute
Command line tool to help users easily manipulate, ideally configuration/text-based, line delimited files. The goal of this project it to provide simple, but common, configuration mutations in an easy to read/auditable tool. There's nothing special here, its just a tool I wanted handy and couldn't find so I made a v0.1 in an afternoon. Hint - the code isn't beautiful. For example, say you have a toml file as follows:
```toml
[[network]]
entry_1="one"
[[security]]
entry_2="two"
```
And for an integration test you want to add an entry under the `security` sub-table. You might then run this tool the following way:
```bash
mute some_config.toml "NEW_ENTRY=123" add after-pattern "^\[\[security]]"
```
which will yield the following file (**overwriting** the current):
```toml
[[network]]
entry_1="one"
[[security]]
NEW_ENTRY=123
entry_2="two"
```

## More Examples

**Remove the 4th line from the file `big.conf`.**
```bash
mute big.conf remove at-line 4
```

**Dry-run swapping the copyright owner of this repository in the `LICENSE` file**
```bash
mute --dry-run LICENSE "Copyright (c) 2022 Cookie Monster" add overwrite-pattern "^Copyright"
```
Here's the output of that command:
```
(Line: 3)        --- Copyright (c) 2022 Casey Kneale
(Line: 3)        +++ Copyright (c) 2022 Cookie Monster
```
Whew thank goodness we used the dry-run command to do that!

**Show how many instances of a line entry that starts with the text `common_entry`(via a regex) exist inside `simple.toml`.**
```bash
mute --dry-run simple.toml "NEW_ENTRY" add overwrite-pattern "^common_entry"
```

**Replace all entries which starts with `common_entry` inside of `simple.toml` with `NEW_ENTRY`.**
```bash
mute simple.toml "NEW_ENTRY" add overwrite-pattern "^common_entry"
```

**Remove all lines that begin with `entry_` in entries.txt.**
```bash
mute entries.txt remove overwrite-pattern "^entry_"
```

## Options
```
Options:
      --dry-run  This will not make any changes but will report the changes made if the command is executed
  -h, --help     Print help information
  -V, --version  Print version information
```
Probably the only option worth describing is `--dry-run`. This will print a diff for all lines that **would** be changed in a given file, with a given set of commands, if the `--dry-run` option was not provided. Correct, this option does not mutate any files, but does report what would change if it was run. There's 2 reasons for this:
  1. Let users fact check themselves before mutating a file. Yea you should always be backing up files before tinkering with this.
  2. Logging changes to files during runs. So you may opt to call `--dry-run` inside of say a test suite so there is documentation as to what changed(or didn't change) due to the command, and then run the command without the dry-run option.

## Commands
```
Commands:
  add     This mode will add an additional line or overwrite a line in an existing file at a position specified via a regex or a line number.
  remove  This mode will remove a line from a file at a position specified by a regex or a line number.
```

Additionally each command has a list of subcommands available too choose from that define the behavior we expect.

```
Commands:
  after-pattern      The line after a regex match is effected. Note: multiple matches can happen in a file
  at-line            The line at the specified line number (starting from 1) is effected
  before-pattern     The line before a regex match is effected. Note: multiple matches can happen in a file
  overwrite-pattern  The line which matches a regex is replaced. Note: multiple matches can happen in a file
  overwrite-line     The line specified by the line number (starting from 1) is over written with the entry
```
Please note:
 - All `pattern` (regex based) options operate on the lines of the file not on the entire file. 
 - The `line` (line number based) options start from line 1 and on. We don't use 0 indexing here. Simply because most IDE's do not and that'd be confusing.

## Is this the right tool for me?
 - I have a simple need which involves modifying text based files delimited by `\n` characters.
 - I would like to log the changes I make to files so that debugging consumers of file changes made by these operations easier.
 - I want to receive an error code if my pattern doesn't match any lines in the file.
 - I like to download CLI tools written by strangers and try them out.

## Is this the wrong tool for me?
 - I want to pipe file "contents" into this tool without creating a file.
 - I want to open files larger than memory, or need to operate on files longer than the `usize` max.
 - I only want to match on `N` lines with a given pattern that has `M` matches.
 - I need to Remove/Overwrite a "range" of line numbers.
 - I know bash, sed, awk, etc and have completely memorized how to use them, and would strongly prefer not too use this tool.
 - I need these logs to be changed as fast as my computer will allow!
