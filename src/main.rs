use std::io::stdout;

use run_pipeline::run;

fn main() {    
    run("pipeline.yml", &mut stdout());
}
