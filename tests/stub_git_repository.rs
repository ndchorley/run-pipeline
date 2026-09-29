use run_pipeline::git::GitRepository;

pub struct StubGitRepository {
    pub head: String,
    pub uncommited_changes: bool
}

impl GitRepository for StubGitRepository {
    fn head(&self) -> String { self.head.to_owned() }
    
    fn has_uncommitted_changes(&self) -> bool {
        self.uncommited_changes
    }
}
