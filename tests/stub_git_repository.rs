use run_pipeline::{errors::Error, git::GitRepository};

pub struct StubGitRepository {
    pub head: String,
    pub uncommitted_changes: bool
}

impl GitRepository for StubGitRepository {
    fn head(&self) -> Result<String, String> {
        Ok(self.head.to_owned())
    }
    
    fn has_uncommitted_changes(&self) -> Result<bool, Error> {
        Ok(self.uncommitted_changes)
    }
}
