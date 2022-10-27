use std::fs::File;
use anyhow::Result;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use std::io;
use super::ymd_hm_format;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "Issue Key")]
    pub issue_key: String,

    #[serde(rename = "Issue summary")]
    pub issue_summary: String,

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

pub fn read_csv(file: String) -> Result<Vec<Record>> {
    let mut records: Vec<Record> = vec![];

    let iordr: Box<dyn io::Read> = if file == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(file)?)
    };

    let mut rdr = csv::Reader::from_reader(iordr);
    for result in rdr.deserialize() {
        records.push(result?);
    }

    Ok(records)
}
