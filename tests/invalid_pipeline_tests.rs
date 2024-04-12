use assertor::*;
use run_pipeline::run;

mod fake_git_repository;
mod helpers;

use fake_git_repository::*;
use helpers::as_string;

#[test]
fn it_complains_if_the_pipeline_cant_be_parsed() {
    let mut output = Vec::new();
    let git_repository = FakeGitRepository {
        head: String::from("does-not-matter")
    };

    run("tests/invalid-pipelines/unparseable-pipeline.yml", &mut output, &git_repository);

    assert_that!(as_string(output))
        .is_equal_to("Could not parse pipeline\n".to_string());
}

#[test]
fn it_complains_if_the_pipeline_is_missing_a_stages_sequence() {
    let mut output = Vec::new();
    let git_repository = FakeGitRepository {
        head: String::from("does-not-matter")
    };

    run("tests/invalid-pipelines/missing-stages-pipeline.yml", &mut output, &git_repository);

    assert_that!(as_string(output))
        .is_equal_to(
            "Could not parse pipeline: missing a sequence called 'stages'\n".to_string()
        );
}

#[test]
fn it_complains_if_a_stage_is_not_a_mapping() {
    let mut output = Vec::new();
    let git_repository = FakeGitRepository {
        head: String::from("does-not-matter")
    };

    run("tests/invalid-pipelines/stage-not-a-mapping-pipeline.yml", &mut output, &git_repository);

    assert_that!(as_string(output))
        .is_equal_to(
            "Could not parse pipeline: stage must be a mapping with keys 'name' and 'command'\n".to_string()
        );
}

#[test]
fn it_complains_if_a_stage_is_missing_a_name() {
    let mut output = Vec::new();
    let git_repository = FakeGitRepository {
        head: String::from("does-not-matter")
    };

    run("tests/invalid-pipelines/stage-missing-a-name-pipeline.yml", &mut output, &git_repository);

    assert_that!(as_string(output))
        .is_equal_to(
            "Could not parse pipeline: stage missing key 'name'\n".to_string()
        );
}

#[test]
fn it_complains_if_a_stage_name_is_not_a_string() {
    let mut output = Vec::new();
    let git_repository = FakeGitRepository {
        head: String::from("does-not-matter")
    };

    run("tests/invalid-pipelines/stage-name-not-a-string-pipeline.yml", &mut output, &git_repository);

    assert_that!(as_string(output))
        .is_equal_to(
            "Could not parse pipeline: stage name must be a string\n".to_string()
        );
}

#[test]
fn it_complains_if_a_stage_is_missing_a_command() {
    let mut output = Vec::new();
    let git_repository = FakeGitRepository {
        head: String::from("does-not-matter")
    };

    run("tests/invalid-pipelines/stage-missing-a-command-pipeline.yml", &mut output, &git_repository);

    assert_that!(as_string(output))
        .is_equal_to(
            "Could not parse pipeline: stage missing key 'command'\n".to_string()
        );
}
