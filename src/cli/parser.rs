use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(
    author="Dr. Casey Kneale", 
    version="0.1.0", 
    about="Mutate files without bashing out your brains and DOS'ing stackexchange.", 
    long_about = None
)]
pub struct CLIArguments {
    /// The file/path to mutate.
    pub file: String,
    /// Required for the `Add` command. Ignored if using the `Remove` command.
    pub entry: Option<String>,
    #[clap(long)]
    /// This will not make any changes but will report the changes made if the command is executed.
    pub dry_run: bool,
    #[clap(subcommand)]
    pub command: What,
}

#[derive(Debug, Subcommand, Clone)]
pub enum What {
    /// This mode will add an additional line or overwrite a line in an existing file at a position specified by a regex or a line number.
    Add(Where),
    /// This mode will remove a line from a file at a position specified by a regex or a line number.
    Remove(Where),
}

#[derive(Parser, Debug, Clone)]
pub struct Where {
    #[clap(subcommand)]
    pub command: WhereCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum WhereCommands {
    /// The line after a regex match is effected. Note: multiple matches can happen in a file.
    AfterPattern(PatternMutation),
    /// The line at the specified line number (starting from 1) is effected.
    AtLine(LineMutation),
    /// The line before a regex match is effected. Note: multiple matches can happen in a file.
    BeforePattern(PatternMutation),
    /// The line which matches a regex is replaced. Note: multiple matches can happen in a file.
    OverwritePattern(PatternMutation),
    /// The line specified by the line number (starting from 1) is over written with the entry.
    OverwriteLine(LineMutation),
}

#[derive(Parser, Debug, Clone)]
pub struct PatternMutation {
    pub pattern: String,
}

#[derive(Parser, Debug, Clone)]
pub struct LineMutation {
    pub line_number: usize,
}
