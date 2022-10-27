use clap::{Parser, Subcommand};

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Print a config template.
    ConfigTemplate,
}

#[derive(Parser)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    /// Output what would happen, but don't actually submit to Clockify.
    #[arg(short, long)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,

    /// The Jira timesheet CSV export file. Use '-' to read from stdin.
    pub file: String,
}
