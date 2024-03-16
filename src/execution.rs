use std::{io::Result, process::Command, process::Output};


pub fn execute(command: &String) -> Result<Output> {
    let output =
        Command::new(command).output();

    return output;
}
