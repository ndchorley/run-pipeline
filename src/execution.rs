use std::{io::{Result, Write}, process::{Command, Output}};

use crate::{display::*, domain::*, git::GitRepository};

impl Pipeline {
    pub fn run_stages(
        &self, writer: &mut impl Write, git_repository: &impl GitRepository
    ) -> core::result::Result<(), String> {
        writeln!(writer, "Running on commit \x1B[0;93m{}\x1B[0m\n", git_repository.head()).unwrap();

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
