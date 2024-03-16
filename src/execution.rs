use std::{io::{Result, Write}, process::{Command, Output}};

use crate::{display::*, domain::Pipeline};

impl Pipeline {
    pub fn run_stages(&self, writer: &mut impl Write) {
        let _: Vec<_> =
            self.stages
                .iter()
                .map(|stage| {
                    display_running_message(&stage.name, writer);

                    let output = Self::execute(&stage.command).unwrap();
                    display_command_output(output.stdout, writer);

                    display_finished_message(
                        &stage.name, output.status, writer
                    );

                    if output.status.success() {
                        Ok(())
                    } else {
                        Err("")
                    }
                })
                .take_while(|result| result.is_ok())
                .collect();
    }

    fn execute(command: &String) -> Result<Output> {
        let output =
            Command::new(command).output();

        return output;
    }
}
