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

    pub fn from(path_to_package_json: Option<&str>) -> Self {
        let path = path_to_package_json.unwrap_or_else(|| "package.json");

        let package_json = std::fs::read_to_string(path)
            .with_context(|| "Failed to read package.json")
            .unwrap();
        let package_json: PackageJson = serde_json::from_str(&package_json)
            .with_context(|| "Failed to deserialize package.json")
            .unwrap();

        package_json
    }
}

#[cfg(test)]
mod tests {
    use crate::package_json::PackageJson;

    #[test]
    fn should_read_from_path() -> Result<(), String> {
        PackageJson::from(Some("package.json"));
        Ok(())
    }

    #[test]
    fn should_read_from_default_path() -> Result<(), String> {
        PackageJson::from(None);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn should_panic_on_wrong_path() {
        PackageJson::from(Some("package.json5"));
    }

    #[test]
    fn should_parse_version() -> Result<(), String> {
        let package_json = PackageJson::from(None);
        package_json.parse_version().unwrap();

        Ok(())
    }

    #[test]
    #[should_panic]
    fn should_panic_on_bad_version() {
        let package_json = PackageJson {
            name: "lw-version".to_owned(),
            version: "0.1".to_owned(),
        };

        package_json.parse_version().unwrap();
    }

    #[test]
    fn should_set_new_version() {
        todo!()
    }

    #[test]
    fn should_panic_on_lower_version_set() {
        todo!()
    }
}
