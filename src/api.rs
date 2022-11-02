use super::ymd_hm_format;
use anyhow::Result;
use chrono::{DateTime, Utc};
use float_duration::FloatDuration;
use reqwest::blocking::{Client, Response};
use reqwest::header;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntry {
    #[serde(deserialize_with = "ymd_hm_format::deserialize")]
    start: DateTime<Utc>,

    #[serde(deserialize_with = "ymd_hm_format::deserialize")]
    end: DateTime<Utc>,

    project_id: String,
    description: String,
}

impl TimeEntry {
    pub fn new(
        project_id: String,
        start: DateTime<Utc>,
        hours: f64,
        description: String,
    ) -> Result<Self> {
        // Hacky fix for timezone issue...
        let start = start + FloatDuration::hours(4.0).to_chrono()?;
        Ok(TimeEntry {
            start,
            end: start + FloatDuration::hours(hours).to_chrono()?,
            description,
            project_id,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Workspace {
    pub id: String,
    pub name: String,
}

pub struct ApiClient {
    base_url: Url,
    client: reqwest::blocking::Client,
}

impl ApiClient {
    pub fn new(base_url: String, api_key: String) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Api-Key", header::HeaderValue::from_str(&api_key)?);

        let client = Client::builder().default_headers(headers).build()?;

        Ok(ApiClient {
            base_url: Url::parse(&base_url)?,
            client,
        })
    }

    pub fn get_workspaces(&self) -> Result<Vec<Workspace>> {
        self.client
            .get(
                self.base_url
                    .join("workspaces")?
                    .as_str(),
            )
            .send()?
            .json::<Vec<Workspace>>()
            .map_err(anyhow::Error::from)
    }

    pub fn get_projects(&self, workspace: &str) -> Result<Vec<Project>> {
        self.client
            .get(
                self.base_url
                    .join(&format!("workspaces/{}/projects", workspace))?
                    .as_str(),
            )
            .send()?
            .json::<Vec<Project>>()
            .map_err(anyhow::Error::from)
    }

    pub fn post_time_entry(&self, workspace: &str, time_entry: TimeEntry) -> Result<Response> {
        self.client
            .post(
                self.base_url
                    .join(&format!("workspaces/{}/time-entries", workspace))?
                    .as_str(),
            )
            .json(&time_entry)
            .send()
            .map_err(anyhow::Error::from)
    }
}
