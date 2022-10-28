use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Clone)]
pub struct TransferArgs {
    /// The Jira timesheet CSV export file. Use '-' to read from stdin.
    pub file: String,

    /// Output what would happen, but don't actually submit to Clockify.
    #[arg(short, long)]
    pub dry_run: bool,

    /// Load configuration from a custom location. Defaults to: $XDG_CONFIG/clockify-transfer/config.yml
    #[arg(short, long = "config", value_name = "FILE")]
    pub config_path: Option<PathBuf>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Print a config template
    ConfigTemplate,
    /// Create a config file. Defaults to: $XDG_CONFIG/clockify-transfer/config.yml
    Init {
        /// Create configuration at a custom location.
        #[arg(short, long = "config", value_name = "FILE")]
        config_path: Option<PathBuf>,
    },
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
