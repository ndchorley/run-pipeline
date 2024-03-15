use run_pipeline::run;

#[test]
fn it_runs_the_stages_in_the_pipeline() {
    let mut output = Vec::new();

    run("tests/pipeline.yml", &mut output);

    assert_eq!(
        as_string(output),
        concat!(
            "Running Build...\n",
            "some build output\n\n",
            "Build succeeded\n",

            "Running Deploy...\n",
            "some deploy output\n\n",
            "Deploy succeeded\n"
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
