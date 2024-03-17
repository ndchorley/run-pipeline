use std::{fs, io::Write};

use domain::{Pipeline, Stage};
use parsing::parse_pipeline;

pub mod display;
pub mod domain;
pub mod execution;
pub mod parsing;

pub fn run(pipeline_file: &str, writer: &mut impl Write) {
    let read_pipeline_result =
        fs::read_to_string(pipeline_file);

    match read_pipeline_result {
        Ok(pipeline_string) => {
            let pipeline =
                parse_pipeline(&pipeline_string).unwrap();

            pipeline.run_stages(writer);
        }

        Err(_) => {
            let message =
                String::from("Could not find pipeline at ") + pipeline_file;

            writeln!(writer, "{}", message).unwrap();
        }
    }
}
