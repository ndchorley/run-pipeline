use run_pipeline::run;

mod helpers;

use helpers::as_string;

#[test]
fn it_complains_if_the_pipeline_cant_be_parsed() {
    let mut output = Vec::new();

    run("tests/unparseable-pipeline.yml", &mut output);

    assert_eq!(
        as_string(output),
        "Could not parse pipeline\n"
    );
}

#[test]
fn it_complains_if_the_pipeline_is_missing_a_stages_sequence() {
    let mut output = Vec::new();

    run("tests/missing-stages-pipeline.yml", &mut output);

    assert_eq!(
        as_string(output),
        "Could not parse pipeline: missing a sequence called 'stages'\n"
    );
}
