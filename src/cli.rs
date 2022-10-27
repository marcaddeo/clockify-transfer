use clap::{Parser, Subcommand};

#[derive(Parser, Clone)]
pub struct TransferArgs {
    /// Output what would happen, but don't actually submit to Clockify.
    #[arg(short, long)]
    pub dry_run: bool,

    /// The Jira timesheet CSV export file. Use '-' to read from stdin.
    pub file: String,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Print a config template.
    ConfigTemplate,
}

#[derive(Parser)]
#[command(arg_required_else_help(true))]
#[command(subcommand_negates_reqs(true))]
#[command(args_conflicts_with_subcommands(true))]
pub struct Cli {
    #[command(flatten)]
    pub args: Option<TransferArgs>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}
