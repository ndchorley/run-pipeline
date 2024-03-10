use pipeline_runner::run;

#[test]
fn it_displays_the_stages_in_pipeline_yml() {
    let mut output = Vec::new();

    run("tests/pipeline.yml", &mut output);

    assert_eq!(
        as_string(output),
        concat!(
            "name: Build, command: ./build.sh\n",
            "name: Deploy, command: ./deploy.sh\n"
        )
    );
}

fn as_string(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap()
}
