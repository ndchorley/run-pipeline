use std::io::stdout;

use run_pipeline::{git::PlaceholderGitRepository, run};

fn main() {
    run(
        "pipeline.yml",
        &mut stdout(),
        &PlaceholderGitRepository {}
    );
}
