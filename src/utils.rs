use anyhow::{Context, Result};
use itertools::Itertools;

pub fn parse_version(version: &str) -> Result<(String, String, String)> {
    let (major, minor, patch) = version
        .splitn(3, '.')
        .into_iter()
        .collect_tuple::<(&str, &str, &str)>()
        .with_context(|| "Cannot parse version field")
        .unwrap();

    Ok((major.to_owned(), minor.to_owned(), patch.to_owned()))
}

pub fn join_version(parsed_version: (String, String, String)) -> String {
    format!(
        "{}.{}.{}",
        parsed_version.0, parsed_version.1, parsed_version.2
    )
}
