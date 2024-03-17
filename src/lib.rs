use std::io::Write;

use domain::{Pipeline, Stage};
use parsing::parse_pipeline;

pub mod display;
pub mod domain;
pub mod execution;
pub mod file;
pub mod parsing;

pub fn run(pipeline_file: &str, writer: &mut impl Write) {
    let read_pipeline_result = file::read_file(pipeline_file);

    match read_pipeline_result {
        Ok(pipeline_string) => {
            match parse_pipeline(&pipeline_string) {
                Ok(pipeline) => { pipeline.run_stages(writer); }
                Err(_) => {
                    let message =
                        String::from("Could not parse pipeline at ") + pipeline_file;

                    writeln!(writer, "{}", message).unwrap();
                }

            }
        }

        Err(message) => {
            writeln!(writer, "{}", message).unwrap();
        }
    }
}
