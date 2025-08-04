use run_pipeline::git::GitRepository;

pub struct FakeGitRepository {
    pub head: String,
    pub uncommited_changes: bool
}

impl GitRepository for FakeGitRepository {
    fn head(&self) -> String { self.head.to_owned() }
    
    fn has_uncommited_changes(&self) -> bool {
        self.uncommited_changes
    }
}
