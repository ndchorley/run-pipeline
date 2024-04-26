use std::io::stdout;

use run_pipeline::{git::FileSystemGitRepository, run};

fn main() {
    run(
        "pipeline.yml",
        &mut stdout(),
        &FileSystemGitRepository { directory: String::from(".") }
    );
}
