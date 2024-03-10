use std::io::stdout;

use pipeline_runner::run;

fn main() {    
    run("pipeline.yml", &mut stdout());
}
