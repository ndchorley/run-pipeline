use std::{io::Write, process::Command};

use crate::Stage;

pub fn execute(stage: &Stage, writer: &mut impl Write) {
    let running_stage_message =
        String::from("Running ") + &stage.name + "...";
    writeln!(writer, "{}", &running_stage_message)
        .unwrap();

    let output =
        Command::new(&stage.command).output().unwrap();

    let command_output =
        String::from_utf8(output.stdout).unwrap();
    writeln!(writer, "{}", &command_output).unwrap();

    let finished_stage_message =
        String::from(&stage.name) + " succeeded";
    writeln!(writer, "{}", &finished_stage_message).unwrap();
}
