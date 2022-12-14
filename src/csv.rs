use super::ymd_hm_format;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Issue {
    #[serde(rename = "Issue Key")]
    pub key: String,

    #[serde(rename = "Issue summary")]
    pub summary: String,

    #[serde(rename = "Hours")]
    pub hours: f64,

    #[serde(rename = "Work date")]
    #[serde(with = "ymd_hm_format")]
    pub work_date: DateTime<Utc>,

    #[serde(rename = "Project Key")]
    pub project_key: String,

    #[serde(rename = "Work Description")]
    pub work_description: String,
}

pub fn read_issues(file: String) -> Result<Vec<Issue>> {
    let mut issues: Vec<Issue> = vec![];

    let iordr: Box<dyn io::Read> = if file == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(file)?)
    };

    let mut rdr = csv::Reader::from_reader(iordr);
    for issue in rdr.deserialize() {
        issues.push(issue?);
    }

    Ok(issues)
}

pub fn write_issues(file: String, issues: Vec<Issue>) -> Result<String> {
    let unprocessed_file;
    let iowtr: Box<dyn io::Write> = if file == "-" {
        unprocessed_file = String::from("STDERR");

        Box::new(io::stderr())
    } else {
        unprocessed_file = format!("{}-unprocessed-issues", file);

        Box::new(File::create(unprocessed_file.clone())?)
    };

    let mut wtr = csv::Writer::from_writer(iowtr);
    for issue in issues {
        wtr.serialize(issue)?;
    }

    Ok(unprocessed_file)
}
