use anyhow::Context;
use git2::{Repository, StatusEntry};

pub struct GitHelper {
    repository: Repository,
}

impl GitHelper {
    pub fn new() -> Result<Self, anyhow::Error> {
        let repository =
            Repository::open(".").with_context(|| "This project is not a git repository")?;

        Ok(Self { repository })
    }

    pub fn check_project_status(&self) -> Result<(), anyhow::Error> {
        let statutes = self
            .repository
            .statuses(None)
            .with_context(|| "Impossible read the repository status")?;

        let is_modified = !statutes
            .iter()
            .any(|status| status.status().is_index_modified());

        if is_modified {
            panic!(
                "Project work directory should be empty. Please stash or commit all modified files"
            );
        }

        Ok(())
    }

    pub fn read_history_from_tag(&self) -> () {
        ()
    }
}
