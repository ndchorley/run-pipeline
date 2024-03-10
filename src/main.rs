use std::io::stdout;

use pipeline_runner::run;

fn main() {    
    run(&mut stdout());
}
