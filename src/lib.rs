use std::io::Write;

use domain::{Pipeline, Stage};
use file::read_file;
use git::GitRepository;
use parsing::parse_pipeline;

use crate::errors::Error::{self, Other};

pub mod display;
pub mod domain;
pub mod errors;
pub mod execution;
pub mod file;
pub mod git;
pub mod parsing;

pub fn run(pipeline_file: &str, writer: &mut impl Write, git_repository: &impl GitRepository) {
    let result =
        check_for_uncommited_changes(git_repository)
            .and_then(|_| read_file(pipeline_file))
            .and_then(|pipeline_string| parse_pipeline(&pipeline_string))
            .and_then(|pipeline| pipeline.run_stages(writer, git_repository));

    match result {
        Ok(_) => (),

        Err(error) => {
            match error {
                Error::InvalidPipeline(message) => writeln!(writer, "{}", message).unwrap(),
                Other(message) => writeln!(writer, "{}", message).unwrap(),
            }
        }
    }
}

fn check_for_uncommited_changes(git_repository: &impl GitRepository) -> Result<(), Error> {
    match git_repository.has_uncommitted_changes() {
            false => Ok(()),
            true => Err(Other(String::from("There are uncommited changes... aborting"))),
        }
}
