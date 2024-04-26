use git2::Repository;

pub trait GitRepository {
    fn head(&self) -> String;
}

pub struct FileSystemGitRepository {
    pub directory: String
}

impl GitRepository for FileSystemGitRepository {
    fn head(&self) -> String {
        let repository = Repository::open(&self.directory).unwrap();

        let result = repository.head().unwrap();

        result.target().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use assertor::*;
    use git2::{Repository, Signature};

    use crate::git::GitRepository;

    use super::FileSystemGitRepository;

    #[test]
    fn it_returns_the_hash_of_the_latest_commit() {
        let repository_path = "./some-repo";
        let commit_hash = create_repository_with_a_commit(&repository_path);

        let repository = FileSystemGitRepository { directory: String::from(repository_path) };

        assert_that!(repository.head()).is_equal_to(commit_hash);

        remove_repository(repository_path)
    }

    fn create_repository_with_a_commit(path: &str) -> String {
        let repository = Repository::init(path).unwrap();
        let tree =
            repository.find_tree(repository.index().unwrap().write_tree().unwrap()).unwrap();

        let author = Signature::now("Some One", "someone@example.com").unwrap();

        let commit = repository
            .commit(
                Some("HEAD"),
                &author,
                &author,
                "Some commit message",
                &tree,
                &vec![]
        ).unwrap();

        commit.to_string()
    }

    fn remove_repository(repository_path: &str) {
        fs::remove_dir_all(repository_path).unwrap();
    }
}
