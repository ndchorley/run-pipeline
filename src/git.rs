use git2::Repository;

pub trait GitRepository {
    fn head(&self) -> String;
    fn has_uncommited_changes(&self) -> bool;
}

pub struct FileSystemGitRepository {
    pub directory: String
}

impl GitRepository for FileSystemGitRepository {
    fn head(&self) -> String {
        let repository = Repository::open(&self.directory).unwrap();

        let head = repository.head().unwrap();

        head.target().unwrap().to_string()
    }
    
    fn has_uncommited_changes(&self) -> bool { false }
}

#[cfg(test)]
mod tests {
    use assertor::*;
    use git2::{Repository, Signature};

    use crate::git::GitRepository;

    use super::FileSystemGitRepository;

    #[test]
    fn it_returns_the_hash_of_the_latest_commit() {
        let a_directory = temporary_directory();
        let repository_path = a_directory.as_str();
        let commit_hash = create_repository_with_a_commit(&repository_path);

        let repository = FileSystemGitRepository { directory: String::from(repository_path) };

        assert_that!(repository.head()).is_equal_to(commit_hash);
    }

    #[test]
    fn it_reports_when_there_are_no_uncommited_changes() {
        let a_directory = temporary_directory();
        let repository_path = a_directory.as_str();
        let _ = create_repository_with_a_commit(&repository_path);

        let repository = FileSystemGitRepository { directory: String::from(repository_path) };      

        assert_that!(repository.has_uncommited_changes()).is_false();
    }
    
    fn temporary_directory() -> String {
        let suffix = fastrand::i32(0..std::i32::MAX).to_string();

        format!("/tmp/run-pipeline-repo{}", suffix)
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
}
