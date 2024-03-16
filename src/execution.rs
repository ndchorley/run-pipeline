use std::{io::Result, process::Command, process::Output};

use crate::Stage;

pub fn execute(stage: &Stage) -> Result<Output> {
    let output =
        Command::new(&stage.command).output();

    return output;
}
