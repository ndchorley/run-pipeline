pub trait GitRepository {
    fn head(&self) -> String;
}

pub struct PlaceholderGitRepository;

impl GitRepository for PlaceholderGitRepository {
    fn head(&self) -> String { String::from("some-commit-hash") }
}