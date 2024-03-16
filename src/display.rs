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

    let finished_stage_message =
        colour_start(status) + stage_name + " " + status_string + COLOUR_END;

    writeln!(writer, "{}\n", &finished_stage_message).unwrap();
}

fn colour_start(status: ExitStatus) -> String {
    let colour_string =
        if status.success() { GREEN } else { RED };

    String::from("\x1B[0;") + colour_string + "m"
}

const GREEN: &str = "32";

const RED: &str = "31";

const COLOUR_END: &str = "\x1B[0m";
