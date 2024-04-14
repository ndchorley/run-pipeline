use std::{io::Write, process::ExitStatus};

pub fn display_running_message(
    stage_name: &String, writer: &mut impl Write
) {
    let running_stage_message =
        String::from("Running ") + stage_name + "...";

    writeln!(writer, "{}", &running_stage_message).unwrap();
}

pub fn display_command_output(
    output: Vec<u8>, writer: &mut impl Write
) {
    let command_output =
        String::from_utf8(output).unwrap();

    writeln!(writer, "{}", &command_output).unwrap();
}

pub fn display_finished_message(
    stage_name: &String, status: ExitStatus, writer: &mut impl Write
) {
    let status_string =
        if status.success() {
            "succeeded"
        } else {
            "failed"
        };

    let colour_string =
        if status.success() {
            GREEN
        } else {
            RED
        };

    let message = format!("{} {}", stage_name, status_string);

    writeln!(
        writer,
        "{}\n", coloured_message(&message, colour_string)
    ).unwrap();
}

pub fn display_running_on_commit_message(
    commit_hash: String, writer: &mut impl Write
) {
    let message =
        format!(
            "Running on commit {}",
            coloured_message(&commit_hash, YELLOW)
        );

    writeln!(writer,"{}\n", message).unwrap();
}

fn coloured_message(message: &String, colour_string: &str) -> String {
    String::from(COLOUR_START) + colour_string + "m" + message + COLOUR_END
}


const GREEN: &str = "32";
const RED: &str = "31";
const YELLOW: &str = "93";

const COLOUR_START: &str = "\x1B[0;";
const COLOUR_END: &str = "\x1B[0m";
