use std::io::Write;

use domain::{Pipeline, Stage};
use file::read_file;
use parsing::parse_pipeline;

pub mod display;
pub mod domain;
pub mod execution;
pub mod file;
pub mod parsing;

pub fn run(pipeline_file: &str, writer: &mut impl Write) {
    let result =
        read_file(pipeline_file)
            .and_then(|pipeline_string|
                parse_pipeline(&pipeline_string)
            )
            .map(|pipeline|
                Ok::<(), String>(pipeline.run_stages(writer))
            );

    match result {
        Ok(_) => (),

        Err(message) => {
            writeln!(writer, "{}", message).unwrap();
        }
    }
}
