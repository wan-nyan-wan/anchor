use anyhow::{anyhow, Result};
use cargo_toml::Manifest;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnchorPackage {
    pub name: String,
    pub description: String,
    pub license: Option<String>,
    pub address: String,
}

impl AnchorPackage {
    pub fn from(manifest: Manifest, address: Pubkey) -> Result<Self> {
        let package = manifest
            .package
            .ok_or(anyhow!("Package section not provided"))?;
        let description = package
            .description
            .ok_or(anyhow!("Description not provided"))?;
        Ok(Self {
            name: package.name,
            description,
            license: package.license,
            address: address.to_string(),
        })
    }
}
