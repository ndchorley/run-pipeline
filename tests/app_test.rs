use run_pipeline::run;

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

#[test]
fn it_complains_if_the_pipeline_cant_be_found() {
    let mut output = Vec::new();
    let pipeline_file = "tests/does-not-exist.yml";

    run(pipeline_file, &mut output);

    assert_eq!(
        as_string(output),
        String::from("Could not find pipeline at ") + pipeline_file + "\n"
    )
}

fn as_string(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap()
}
