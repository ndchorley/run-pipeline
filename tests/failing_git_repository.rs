use run_pipeline::git::GitRepository;

pub struct FailingGitRepositoryDueToAbsenceOfUnderlyingRepository {}

impl GitRepository for FailingGitRepositoryDueToAbsenceOfUnderlyingRepository {
    fn head(&self) -> Result<String, String> {
        todo!()
    }

    fn has_uncommitted_changes(&self) -> Result<bool, run_pipeline::errors::Error> {
        Err(run_pipeline::errors::Error::NoGitRepositoryFound)
    }
}
