use super::ymd_hm_format;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use float_duration::FloatDuration;
use reqwest::Url;
use reqwest::header;
use reqwest::blocking::{Client, Response};

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
    pub fn new(project_id: String, start: DateTime<Utc>, hours: f64, description: String) -> Result<Self> {
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

pub struct ApiClient {
    base_url: Url,
    client: reqwest::blocking::Client,
}

impl ApiClient {
    pub fn new(base_url: String, api_key: String) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Api-Key", header::HeaderValue::from_str(&api_key)?);

        let client = Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(ApiClient {
            base_url: Url::parse(&base_url)?,
            client,
        })
    }

    // let project_map: HashMap<String, String> = HashMap::new();
    // for (key, name) in config.project_map {
    //     let json = json!({
    //         "start": issue.work_date,
    //         "end": issue.work_date + FloatDuration::hours(issue.hours).to_chrono()?,
    //         "projectId": project_id,
    //         "description": format!("{}: {}", issue.key, issue.work_description),
    //     });

    //     let api_url = format!(
    //         "{}/workspaces/{}/time-entries",
    //         config.api_base_path, config.workspace_id
    //     );

    // }

    pub fn get_projects(&self, workspace: String) -> Result<Vec<Project>> {
        self.client
            .get(self.base_url.join(&format!("workspaces/{}/projects", workspace))?.as_str())
            .send()?
            .json::<Vec<Project>>()
            .map_err(anyhow::Error::from)
    }

    pub fn post_time_entry(&self, workspace: String, time_entry: TimeEntry) -> Result<Response> {
        self.client
            .post(self.base_url.join(&format!("workspaces/{}/time-entries", workspace))?.as_str())
            .json(&time_entry)
            .send()
            .map_err(anyhow::Error::from)
    }
}
