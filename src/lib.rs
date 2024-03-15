use std::{fs, io::Write, process::Command};

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

            pipeline.stages.iter().for_each(|stage| execute(stage, writer));
        }

        Err(_) => {
            let message =
                String::from("Could not find pipeline at ") + pipeline_file;

            writeln!(writer, "{}", message).unwrap();
        }
    }
}

fn execute(stage: &Stage, writer: &mut impl Write) {
    let running_stage_message =
        String::from("Running ") + &stage.name + "...";
    writeln!(writer, "{}", &running_stage_message).unwrap();

    let output = Command::new(&stage.command).output().unwrap();

    writeln!(writer, "{}", String::from_utf8(output.stdout).unwrap())
        .unwrap();

    let finished_stage_message = String::from(&stage.name) + " succeeded";
    writeln!(writer, "{}", &finished_stage_message).unwrap();
}
