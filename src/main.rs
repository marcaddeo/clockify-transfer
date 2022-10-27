use chrono::{DateTime, Utc};
use float_duration::FloatDuration;
use serde::Deserialize;
use serde_json::json;
use std::io;
use std::io::Write;
use tabwriter::TabWriter;
use confique::{Config, yaml::FormatOptions};
use std::collections::HashMap;
use clap::{Parser, Subcommand};
use std::fs::File;

mod ymd_hm_format;

#[derive(Subcommand, Clone)]
enum Commands {
    /// Print a config template.
    ConfigTemplate,
}

#[derive(Parser)]
#[command(arg_required_else_help(true))]
struct Cli {
    /// Output what would happen, but don't actually submit to Clockify.
    #[arg(short, long)]
    dry_run: bool,

    #[command(subcommand)]
    command: Option<Commands>,

    /// The Jira timesheet CSV export file. Use '-' to read from stdin.
    file: String,

}

#[derive(Config, Debug)]
struct Conf {
    /// The Clockify API base path.
    #[config(default = "https://api.clockify.me/api/v1")]
    api_base_path: String,

    /// Your Clockify API key.
    api_key: String,

    /// Your Clockify Workspace ID.
    workspace_id: String,

    /// A mapping of Jira Project Key to Clockify project ID.
    ///
    /// Example:
    ///
    /// project_map:
    ///   PROJ: 61e33e2d576aeb100a7ed74d
    ///   ANOTHER: 6e56f6ea4cbeb210f8d5be0a
    project_map: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Issue Key")]
    issue_key: String,
    #[serde(rename = "Issue summary")]
    issue_summary: String,
    #[serde(rename = "Hours")]
    hours: f64,
    #[serde(rename = "Work date")]
    #[serde(with = "ymd_hm_format")]
    work_date: DateTime<Utc>,
    #[serde(rename = "Project Key")]
    project_key: String,
    #[serde(rename = "Work Description")]
    work_description: String,
}

fn read_csv(file: String) -> Vec<Record> {
    let mut records: Vec<Record> = vec![];

    let iordr: Box<dyn io::Read> = if file == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::create(file).unwrap())
    };

    let mut rdr = csv::Reader::from_reader(iordr);
    for result in rdr.deserialize() {
        records.push(result.unwrap());
    }

    records
}

fn print_config_template() {
    let yaml = confique::yaml::template::<Conf>(FormatOptions::default());
    println!("{}", yaml);
}

fn process_csv(cli: Cli) {
    let config = Conf::from_file("config.yml").unwrap();
    let client = reqwest::blocking::Client::new();
    let mut tw = TabWriter::new(io::stdout()).minwidth(2).padding(2);

    let records = read_csv(cli.file);
    for record in records {
        write!(
            &mut tw,
            "{}\t // {}\t {}\t {}h\t ... ",
            record.issue_key, record.issue_summary, record.work_description, record.hours
        )
        .unwrap();

        let project_id = match config.project_map.get(&record.project_key) {
            Some(id) => id,
            None => {
                write!(
                    &mut tw,
                    "Could not map project: {};\t skipped.\n",
                    record.project_key
                )
                .unwrap();

                continue;
            }
        };

        let json = json!({
            "start": record.work_date,
            "end": record.work_date + FloatDuration::hours(record.hours).to_chrono().unwrap(),
            "projectId": project_id,
            "description": format!("{}: {}", record.issue_key, record.work_description),
        });

        let api_url = format!("{}/workspaces/{}/time-entries", config.api_base_path, config.workspace_id);

        if !cli.dry_run {
            let response = client
                .post(api_url)
                .header("X-Api-Key", config.api_key.clone())
                .json(&json)
                .send()
                .unwrap();

            match response.error_for_status() {
                Ok(_) => write!(&mut tw, "success.").unwrap(),
                Err(_) => write!(&mut tw, "error.").unwrap(),
            }
        } else {
            write!(&mut tw, "dry run.").unwrap();
        }

        write!(&mut tw, "\n").unwrap();
    }

    tw.flush().unwrap();

}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::ConfigTemplate) => {
            print_config_template()
        },
        None => {
            process_csv(cli)
        }
    }
}
