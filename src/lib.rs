use std::{fs, io::Write};

use display::*;
use domain::{Pipeline, Stage};
use execution::execute;
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
            let pipeline = parse_pipeline(&pipeline_string);

            let _: Vec<_> = pipeline.stages
                .iter()
                .map(|stage| {
                    display_running_message(&stage.name, writer);

                    let output = execute(&stage.command).unwrap();
                    display_command_output(output.stdout, writer);

                    display_finished_message(&stage.name, output.status, writer);

                    if output.status.success() {
                        Ok(())
                    } else {
                        Err("")
                    }
                })
                .take_while(|result| result.is_ok())
                .collect();
        }

        Err(_) => {
            let message =
                String::from("Could not find pipeline at ") + pipeline_file;

            writeln!(writer, "{}", message).unwrap();
        }
    }
}
