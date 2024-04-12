use std::io::Write;

use domain::{Pipeline, Stage};
use file::read_file;
use git::GitRepository;
use parsing::parse_pipeline;

pub mod display;
pub mod domain;
pub mod execution;
pub mod file;
pub mod git;
pub mod parsing;

pub fn run(pipeline_file: &str, writer: &mut impl Write, git_repository: &impl GitRepository) {
    let result =
        read_file(pipeline_file)
            .and_then(|pipeline_string| parse_pipeline(&pipeline_string))
            .and_then(|pipeline| pipeline.run_stages(writer, git_repository));

    match result {
        Ok(_) => (),

        Err(message) =>
            writeln!(writer, "{}", message).unwrap()
    }
}
