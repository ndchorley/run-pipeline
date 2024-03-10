use pipeline_runner::run;

#[test]
fn it_displays_a_message() {
    let mut output = Vec::new();

    run(&mut output);

    assert_eq!(
        as_string(output),
        "Hello pipeline-runner\n"
    );
}

fn as_string(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap()
}
