use std::{io::{Result, Write}, process::{Command, Output}};

use crate::{display::*, domain::*};

impl Pipeline {
    pub fn run_stages(&self, writer: &mut impl Write) {
        let _: Vec<_> =
            self.stages
                .iter()
                .map(|stage| { Self::run_stage(stage, writer) })
                .take_while(|result| result.is_ok())
                .collect();
    }

    fn run_stage(
        stage: &Stage, writer: &mut impl Write
    )  -> core::result::Result<(), ()> {
        display_running_message(&stage.name, writer);

        let output = Self::execute(&stage.command).unwrap();
        display_command_output(output.stdout, writer);

        display_finished_message(&stage.name, output.status, writer);

        if output.status.success() {
            Ok(())
        } else {
            Err(())
        }
    }

    fn execute(command: &String) -> Result<Output> {
        let output =
            Command::new(command).output();

        return output;
    }
}
