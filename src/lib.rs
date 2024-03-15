use std::{fs, io::Write};

use domain::{Pipeline, Stage};
use parsing::parse_pipeline;

pub mod domain;
pub mod parsing;

pub fn run(pipeline_file: &str, writer: &mut impl Write) {
    let read_pipeline_result =
        fs::read_to_string(pipeline_file);

    match read_pipeline_result {
        Ok(pipeline_string) => {
            let pipeline = parse_pipeline(&pipeline_string);

            display_pipeline(pipeline, writer);
        }

        Err(_) => {
            let message =
                String::from("Could not find pipeline at ") + pipeline_file;

            writeln!(writer, "{}", message).unwrap();
        }
    }
}

fn display_pipeline(pipeline: Pipeline, writer: &mut impl Write) {
    let output_lines = make_output_lines(pipeline.stages);

    writeln!(writer, "{}", output_lines.as_slice().join("\n"))
        .unwrap();
}

fn make_output_lines(stages: Vec<Stage>) -> Vec<String> {
    stages
        .iter()
        .map(|stage|
            String::from("name: ") + &stage.name + ", command: " + &stage.command
        )
        .collect()
}
