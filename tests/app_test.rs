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
fn it_does_not_run_subsequent_stages_after_a_failure() {
    let mut output = Vec::new();

    run("tests/failing-build-pipeline.yml", &mut output);

    assert_eq!(
        as_string(output),
        concat!(
            "Running Build...\n",
            "some failure\n\n",
            "Build failed\n",
        )
    );
}

#[test]
fn it_complains_if_the_pipeline_cant_be_found() {
    let mut output = Vec::new();

    run("tests/does-not-exist.yml", &mut output);

    assert_eq!(
        as_string(output),
        "Could not find pipeline at tests/does-not-exist.yml\n"
    )
}

fn as_string(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap()
}
