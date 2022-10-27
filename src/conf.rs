use confique::{yaml::FormatOptions, Config};
use std::collections::HashMap;

#[derive(Config, Debug)]
pub struct Conf {
    /// The Clockify API base path.
    #[config(default = "https://api.clockify.me/api/v1")]
    pub api_base_path: String,

    /// Your Clockify API key.
    pub api_key: String,

    /// Your Clockify Workspace ID.
    pub workspace_id: String,

    /// A mapping of Jira Project Key to Clockify project ID.
    ///
    /// Example:
    ///
    /// project_map:
    ///   PROJ: 61e33e2d576aeb100a7ed74d
    ///   ANOTHER: 6e56f6ea4cbeb210f8d5be0a
    pub project_map: HashMap<String, String>,
}

pub fn print_config_template() {
    let yaml = confique::yaml::template::<Conf>(FormatOptions::default());
    println!("{}", yaml);
}
