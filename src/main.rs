use crate::csv::{read_issues, write_issues, Issue};
use anyhow::{bail, Context, Result};
use api::{ApiClient, TimeEntry};
use clap::Parser;
use cli::{Cli, Commands, TransferArgs};
use conf::Conf;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use tabwriter::TabWriter;

mod api;
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
    let api_client = ApiClient::new(config.api_base_path.as_str().into(), config.api_key)?;
    let mut tw = TabWriter::new(io::stdout()).minwidth(2).padding(2);

    let mut unprocessed_issues: Vec<Issue> = vec![];
    let issues = read_issues(args.file.clone()).with_context(|| {
        if args.file == "-" {
            "Failed to read csv data from STDIN".to_string()
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

        let project_name = match config.project_map.get(&issue.project_key) {
            Some(name) => name,
            None => {
                writeln!(
                    &mut tw,
                    "Could not find project key in map: {};\t skipped.",
                    issue.project_key
                )?;

                continue;
            }
        };

        let project_list = api_client.get_projects(config.workspace_id.clone())?;
        let project = match project_list
            .into_iter()
            .find(|project| &project.name == project_name)
        {
            Some(project) => project,
            None => {
                writeln!(
                    &mut tw,
                    "Could not find Clockify project id for: {};\t skipped.",
                    project_name
                )?;

                continue;
            }
        };

        if !args.dry_run {
            let time_entry = TimeEntry::new(
                project.id,
                issue.work_date,
                issue.hours,
                format!("{}: {}", issue.key, issue.work_description),
            )?;

            let response = api_client.post_time_entry(config.workspace_id.clone(), time_entry)?;
            match response.error_for_status_ref() {
                Ok(_) => write!(&mut tw, "success.")?,
                Err(_) => {
                    unprocessed_issues.push(issue);
                    write!(&mut tw, "error.")?
                }
            }
        } else {
            write!(&mut tw, "dry run.")?;
        }

        writeln!(&mut tw)?;
    }

    tw.flush()?;

    if !unprocessed_issues.is_empty() {
        let unprocessed_file = write_issues(args.file, issues)?;
        print!("\n\nWARNING: Some issues were transferred to Clockify. ");
        println!(
            "Unprocessed issues have been written to: {}",
            unprocessed_file
        );
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
