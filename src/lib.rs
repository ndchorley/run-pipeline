use std::{fs, io::Write, process::Output};

use domain::{Pipeline, Stage};
use execution::execute;
use parsing::parse_pipeline;

pub mod domain;
pub mod execution;
pub mod parsing;

pub fn run(pipeline_file: &str, writer: &mut impl Write) {
    let read_pipeline_result =
        fs::read_to_string(pipeline_file);

    match read_pipeline_result {
        Ok(pipeline_string) => {
            let pipeline = parse_pipeline(&pipeline_string);

            pipeline.stages
                .iter()
                .for_each(|stage| {
                    display_running_message(&stage.name, writer);

                    let output = execute(&stage.command).unwrap();
                    display_command_output(output, writer);

                    display_finished_message(&stage.name, writer);
                });
        }

        Err(_) => {
            let message =
                String::from("Could not find pipeline at ") + pipeline_file;

            writeln!(writer, "{}", message).unwrap();
        }
    }
}

fn display_running_message(stage_name: &String, writer: &mut impl Write) {
    let running_stage_message =
        String::from("Running ") + stage_name + "...";

    writeln!(writer, "{}", &running_stage_message).unwrap();
}

fn display_command_output(output: Output, writer: &mut impl Write) {
    let command_output =
        String::from_utf8(output.stdout).unwrap();

    writeln!(writer, "{}", &command_output).unwrap();
}

fn display_finished_message(stage_name: &String, writer: &mut impl Write) {
    let finished_stage_message =
        String::new() + stage_name + " succeeded";

    writeln!(writer, "{}", &finished_stage_message).unwrap();
}
