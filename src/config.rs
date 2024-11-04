use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use clap::{Parser, Subcommand};
pub(crate) const DEVCONTAINER_ROOT: &str = ".devcontainer";

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Run,
    Build,
    Down,
    Attach,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Mount {
    pub(crate) source: String,
    pub(crate) target: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BuildSpec {
    pub(crate) dockerfile: String,
    pub(crate) context: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DevContainerSpec {
    pub(crate) image: Option<String>,
    pub(crate) name: String,
    pub(crate) forward_ports: Option<Vec<String>>,
    pub(crate) container_env: Option<HashMap<String, String>>,
    pub(crate) privileged: Option<bool>,
    pub(crate) mounts: Option<Vec<Mount>>,
    pub(crate) run_args: Option<Vec<String>>,
    pub(crate) entrypoint: Option<String>,
    #[serde(default = "default_command")]
    pub(crate) command: String,
    pub(crate) build: Option<BuildSpec>,
}

fn default_command() -> String {
    "sleep infinity".to_string()
}

impl DevContainerSpec {
    pub(crate) fn get_image_name(&self) -> String {
        match &self.image {
            Some(image) => image.clone(),
            None => self.name.clone(),
        }
    }

    pub(crate) fn get_name(&self) -> String {
        self.name.to_lowercase().clone()
    }
}
