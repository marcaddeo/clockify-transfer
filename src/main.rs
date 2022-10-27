use crate::csv::{read_csv, write_issues, Record};
use anyhow::{bail, Context, Result};
use clap::Parser;
use cli::{Cli, Commands, TransferArgs};
use conf::Conf;
use confique::Config;
use float_duration::FloatDuration;
use serde_json::json;
use std::io;
use std::io::Write;
use tabwriter::TabWriter;

mod cli;
mod conf;
mod csv;
mod ymd_hm_format;

fn transfer(args: TransferArgs) -> Result<()> {
    let config = Conf::from_file("config.yml")?;
    let client = reqwest::blocking::Client::new();
    let mut tw = TabWriter::new(io::stdout()).minwidth(2).padding(2);

    let mut unprocessed_records: Vec<Record> = vec![];
    let records = read_csv(args.file.clone()).with_context(|| {
        // @TODO move this logic into read_csv errors.
        if args.file == "-" {
            format!("Failed to read csv data from STDIN")
        } else {
            format!("Failed to read csv data from {}", args.file)
        }
    })?;
    for record in records.clone() {
        write!(
            &mut tw,
            "{}\t // {}\t {}\t {}h\t ... ",
            record.issue_key, record.issue_summary, record.work_description, record.hours
        )?;

        let project_id = match config.project_map.get(&record.project_key) {
            Some(id) => id,
            None => {
                write!(
                    &mut tw,
                    "Could not map project: {};\t skipped.\n",
                    record.project_key
                )?;

                continue;
            }
        };

        let json = json!({
            "start": record.work_date,
            "end": record.work_date + FloatDuration::hours(record.hours).to_chrono()?,
            "projectId": project_id,
            "description": format!("{}: {}", record.issue_key, record.work_description),
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
                    unprocessed_records.push(record);
                    write!(&mut tw, "error.")?
                },
            }
        } else {
            write!(&mut tw, "dry run.")?;
        }

        write!(&mut tw, "\n")?;
    }

    tw.flush()?;

    if unprocessed_records.len() > 0 {
        let unprocessed_file = write_issues(args.file, records)?;
        println!("\n\nSome issues were not able to be transferred to Clockify. Check the output for more information. Unprocessed issues have been saved to: {}", unprocessed_file);
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::ConfigTemplate) => conf::print_config_template(),
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
