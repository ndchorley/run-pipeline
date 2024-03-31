use run_pipeline::run;

mod helpers;

use helpers::as_string;

#[test]
fn it_complains_if_the_pipeline_cant_be_parsed() {
    let mut output = Vec::new();

    run("tests/invalid-pipelines/unparseable-pipeline.yml", &mut output);

    assert_eq!(
        as_string(output),
        "Could not parse pipeline\n"
    );
}

#[test]
fn it_complains_if_the_pipeline_is_missing_a_stages_sequence() {
    let mut output = Vec::new();

    run("tests/invalid-pipelines/missing-stages-pipeline.yml", &mut output);

    assert_eq!(
        as_string(output),
        "Could not parse pipeline: missing a sequence called 'stages'\n"
    );
}

#[test]
fn it_complains_if_a_stage_is_not_a_mapping() {
    let mut output = Vec::new();

    run("tests/invalid-pipelines/stage-not-a-mapping-pipeline.yml", &mut output);

    assert_eq!(
        as_string(output),
        "Could not parse pipeline: stage must be a mapping with keys 'name' and 'command'\n"
    );
}
