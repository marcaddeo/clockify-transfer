use crate::csv::{read_issues, write_issues, Issue};
use anyhow::{bail, Context, Result};
use clap::Parser;
use cli::{Cli, Commands, TransferArgs};
use conf::Conf;
use float_duration::FloatDuration;
use serde_json::json;
use std::io;
use std::io::Write;
use tabwriter::TabWriter;
use std::path::PathBuf;

mod cli;
mod conf;
mod csv;
mod ymd_hm_format;

fn init(config_path: Option<PathBuf>) -> Result<()> {
    let config_path = conf::write_config_template(config_path)?;
    let config_path_str = match config_path.to_str() {
        Some(s) => s,
        None => bail!("Could not convert path to string"),
    };
    println!("Configuration file created: {}", config_path_str);

    Ok(())
}

fn transfer(args: TransferArgs) -> Result<()> {
    let config = Conf::load(args.config_path)?;
    let client = reqwest::blocking::Client::new();
    let mut tw = TabWriter::new(io::stdout()).minwidth(2).padding(2);

    let mut unprocessed_issues: Vec<Issue> = vec![];
    let issues = read_issues(args.file.clone()).with_context(|| {
        if args.file == "-" {
            format!("Failed to read csv data from STDIN")
        } else {
            format!("Failed to read csv data from {}", args.file)
        }
    })?;
    for issue in issues.clone() {
        write!(
            &mut tw,
            "{}\t // {}\t {}\t {}h\t ... ",
            issue.key, issue.summary, issue.work_description, issue.hours
        )?;

        let project_id = match config.project_map.get(&issue.project_key) {
            Some(id) => id,
            None => {
                write!(
                    &mut tw,
                    "Could not map project: {};\t skipped.\n",
                    issue.project_key
                )?;

                continue;
            }
        };

        let json = json!({
            "start": issue.work_date,
            "end": issue.work_date + FloatDuration::hours(issue.hours).to_chrono()?,
            "projectId": project_id,
            "description": format!("{}: {}", issue.key, issue.work_description),
        });

        let api_url = format!(
            "{}/workspaces/{}/time-entries",
            config.api_base_path, config.workspace_id
        );

        if !args.dry_run {
            let response = client
                .post(api_url)
                .header("X-Api-Key", config.api_key.clone())
                .json(&json)
                .send()?;

            match response.error_for_status() {
                Ok(_) => write!(&mut tw, "success.")?,
                Err(_) => {
                    unprocessed_issues.push(issue);
                    write!(&mut tw, "error.")?
                },
            }
        } else {
            write!(&mut tw, "dry run.")?;
        }

        write!(&mut tw, "\n")?;
    }

    tw.flush()?;

    if unprocessed_issues.len() > 0 {
        let unprocessed_file = write_issues(args.file, issues)?;
        print!("\n\nWARNING: Some issues were transferred to Clockify. ");
        println!("Unprocessed issues have been written to: {}", unprocessed_file);
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::ConfigTemplate) => conf::print_config_template(),
        Some(Commands::Init { config_path }) => init(config_path.clone())?,
        None => {
            let args = match cli.args {
                Some(args) => args,
                None => bail!("Could not parse CLI arguments"),
            };

            transfer(args)?
        }
    }

    Ok(())
}
