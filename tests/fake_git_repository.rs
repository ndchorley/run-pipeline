use run_pipeline::git::GitRepository;

pub struct FakeGitRepository {
    pub head: String
}

impl FakeGitRepository {
    pub fn new() -> Self {
        FakeGitRepository {
            head: String::from("does-not-matter")
        }
    }
}

impl GitRepository for FakeGitRepository {
    fn head(&self) -> String { self.head.to_owned() }
}
