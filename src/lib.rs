use std::io::Write;

pub fn run(writer: &mut impl Write) {
    let result = writeln!(writer, "Hello pipeline-runner");

    match result {
        Ok(_) => (),
        Err(_) => ()
    }
}
