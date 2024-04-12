use run_pipeline::git::GitRepository;

pub struct FakeGitRepository {
    pub head: String
}

impl GitRepository for FakeGitRepository {
    fn head(&self) -> String { self.head.to_owned() }
}
