use assertor::*;
use run_pipeline::run;

mod fake_git_repository;
mod helpers;

use helpers::as_string;
use fake_git_repository::*;


#[test]
fn it_runs_the_stages_in_the_pipeline() {
    let mut output = Vec::new();

    let git_repository = FakeGitRepository {
        head: String::from("73c043215dfc973fe8a11eb2f761bc67b330eb3e")
    };

    run(
        "tests/successful/pipeline.yml",
        &mut output,
        &git_repository
    );

    assert_that!(as_string(output))
        .is_equal_to(
            "Running on commit 73c043215dfc973fe8a11eb2f761bc67b330eb3e\n\
            Running Build...\n\
            some build output\n\n\
            \x1B[0;32mBuild succeeded\x1B[0m\n\n\
            \
            Running Deploy...\n\
            some deploy output\n\n\
            \x1B[0;32mDeploy succeeded\x1B[0m\n\n".to_string()
        );
}

#[test]
fn it_does_not_run_subsequent_stages_after_a_failure() {
    let mut output = Vec::new();
    let git_repository = FakeGitRepository {
        head: String::from("does-not-matter")
    };

    run("tests/failing-build/pipeline.yml", &mut output, &git_repository);

    assert_that!(as_string(output))
        .ends_with(
            "Running Build...\n\
            some failure\n\n\
            \x1B[0;31mBuild failed\x1B[0m\n\n".to_string()
        );
}

#[test]
fn it_complains_if_the_pipeline_cant_be_found() {
    let mut output = Vec::new();
    let git_repository = FakeGitRepository {
        head: String::from("does-not-matter")
    };

    run("tests/does-not-exist.yml", &mut output, &git_repository);

    assert_that!(as_string(output))
        .is_equal_to(
            "Could not find pipeline at tests/does-not-exist.yml\n".to_string()
        );
}
