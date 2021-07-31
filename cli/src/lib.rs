use anyhow::Result;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnchorPackage {
    pub name: String,
    pub address: String,
    pub path: String,
}

impl AnchorPackage {
    pub fn from(package: &str, path: &str, address: Pubkey) -> Result<Self> {
        Ok(Self {
            name: package.to_string(),
            path: path.to_string(),
            address: address.to_string(),
        })
    }
}
