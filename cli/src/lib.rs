use crate::config::{Config, WithPath};
use anchor_client::Cluster;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub mod config;
pub mod template;

// Version of the docker image.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DOCKER_BUILDER_VERSION: &str = VERSION;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnchorPackage {
    pub name: String,
    pub address: String,
    pub path: String,
}

impl AnchorPackage {
    pub fn from(name: String, cfg: &WithPath<Config>) -> Result<Self> {
        let cluster = &cfg.provider.cluster;
        if cluster != &Cluster::Mainnet {
            return Err(anyhow!("Publishing requires the mainnet cluster"));
        }
        let program_details = cfg
            .programs
            .get(cluster)
            .ok_or(anyhow!("Program not provided in Anchor.toml"))?
            .get(&name)
            .ok_or(anyhow!("Program not provided in Anchor.toml"))?;
        let path = program_details
            .path
            .clone()
            .ok_or(anyhow!("Path to program binary not provided"))?;
        let address = program_details.address.to_string();
        Ok(Self {
            name,
            path,
            address,
        })
    }
}
