use crate::utils::parse_version;
use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NodeManifest {
    pub name: String,
    pub version: String,
    #[serde(skip)]
    pub parsed_version: (String, String, String),
    #[serde(skip)]
    path: String,
}

impl NodeManifest {
    pub fn from_path(path: Option<&str>) -> Self {
        let path = path.unwrap_or("package.json");

        let manifest = std::fs::read_to_string((path))
            .with_context(|| format!("Cannot read {}", path))
            .unwrap();

        let package_json = serde_json::from_str::<NodeManifest>(&manifest)
            .with_context(|| format!("Cannot deserialize {}", path))
            .unwrap();

        let parsed_version = parse_version(&package_json.version).unwrap();

        Self {
            name: package_json.name,
            version: package_json.version,
            parsed_version,
            path: path.to_owned(),
        }
    }

    pub fn to_path(&self) {
        todo!()
    }
}
