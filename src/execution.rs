use std::{io::{Result, Write}, process::{Command, Output}};

use crate::{display::*, domain::*};

impl Pipeline {
    pub fn run_stages(
        &self, writer: &mut impl Write
    ) -> core::result::Result<(), String> {
        let _: Vec<_> =
            self.stages
                .iter()
                .map(|stage| { stage.run(writer) })
                .take_while(|result| result.is_ok())
                .collect();

        Ok(())
    }
}

impl Stage {
    fn run(&self, writer: &mut impl Write) -> core::result::Result<(), ()> {
        display_running_message(&self.name, writer);

        let output = Self::execute(&self.command).unwrap();
        display_command_output(output.stdout, writer);

        display_finished_message(&self.name, output.status, writer);

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
