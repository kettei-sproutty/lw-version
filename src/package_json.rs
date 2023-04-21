use anyhow::{Context, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
}

impl PackageJson {
    pub fn parse_version(&self) -> Result<(&str, &str, &str)> {
        let (major, minor, patch) = self
            .version
            .splitn(3, '.')
            .collect_tuple::<(&str, &str, &str)>()
            .with_context(|| format!("Cannot parse version. Found {:?}", self.version))?;

        Ok((major, minor, patch))
    }
}

impl Default for PackageJson {
    fn default() -> Self {
        let package_json = std::fs::read_to_string("package.json")
            .with_context(|| "Failed to read package.json")
            .unwrap();
        let package_json: PackageJson = serde_json::from_str(&package_json)
            .with_context(|| "Failed to deserialize package.json")
            .unwrap();

        package_json
    }
}
