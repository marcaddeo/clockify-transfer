use std::io;
use serde::Deserialize;
use serde_json::json;
use chrono::{DateTime, Utc};
use float_duration::FloatDuration;
use tabwriter::TabWriter;
use std::io::Write;

mod ymd_hm_format;

const API_BASE_PATH: &'static str = "https://api.clockify.me/api/v1";
const API_KEY: &'static str = "YmRkMWEzNjktYWFhOS00ZTU0LTg1MWUtODVmZDZlODg5OTc4";
const WORKSPACE: &'static str = "602c50615ce12a7fc451b6e9";
const PROJECTS: &'static [(&'static str, &'static str)] = &[
    ("CAIC", "61eeee2d576a3b100a7ed74d"),
    ("WCF", "6356f6ea4cbeb210f8d5b30a"),
];

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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

fn main() {
    let client = reqwest::blocking::Client::new();
    let mut records: Vec<Record> = vec![];
    let mut tw = TabWriter::new(io::stdout())
        .minwidth(2)
        .padding(2);

    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.deserialize() {
        records.push(result.unwrap());
    }

    let mut output: String = String::new();
    for record in records {
        output.push_str(&format!("{}\t // {}\t {}\t {}h\t ... ", record.issue_key, record.issue_summary, record.work_description, record.hours));
        let project = PROJECTS.into_iter().filter(|(key, _)| key == &record.project_key).next();

        let (_, project_id) = match project {
            Some(project) => project,
            None =>  {
                output.push_str(&format!("Could not map project: {};\t skipped.\n", record.project_key));

                continue;
            },
        };

        let json = json!({
            "start": record.work_date,
            "end": record.work_date + FloatDuration::hours(record.hours).to_chrono().unwrap(),
            "projectId": project_id,
            "description": format!("{}: {}", record.issue_key, record.work_description),
        });

        let api_url = format!("{}/workspaces/{}/time-entries", API_BASE_PATH, WORKSPACE);
        let response = client.post(api_url)
            .header("X-Api-Key", API_KEY)
            .json(&json)
            .send()
            .unwrap();

        match response.error_for_status() {
            Ok(_) => output.push_str("success."),
            Err(_) => output.push_str("error."),
        }

        output.push_str("\n");
    }

    tw.write_all(output.as_bytes()).unwrap();
    tw.flush().unwrap();
}
