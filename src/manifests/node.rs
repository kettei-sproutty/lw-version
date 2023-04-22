use anyhow::{Context, Result};
use itertools::Itertools;
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

        let parsed_version = Self::parse_version(&package_json.version).unwrap();

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

    pub fn parse_version(version: &str) -> Result<(String, String, String)> {
        // todo: move inside utils
        let (major, minor, patch) = version
            .splitn(3, '.')
            .into_iter()
            .collect_tuple::<(&str, &str, &str)>()
            .with_context(|| "Cannot parse version field")
            .unwrap();

        Ok((major.to_owned(), minor.to_owned(), patch.to_owned()))
    }

    pub fn join_version(parsed_version: (String, String, String)) -> String {
        // todo: move inside utils
        format!(
            "{}.{}.{}",
            parsed_version.0, parsed_version.1, parsed_version.2
        )
    }
}
