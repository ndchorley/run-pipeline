use std::{collections::HashMap, fs, io::Write};

use serde_yaml::Sequence;

pub fn run(pipeline_file: &str, writer: &mut impl Write) {
    let pipeline_string =
        fs::read_to_string(pipeline_file).unwrap();

    let pipeline = parse_pipeline(&pipeline_string);

    let output_lines = make_output_lines(pipeline.stages);

    writeln!(writer, "{}", output_lines.as_slice().join("\n"))
        .unwrap();
}

struct Pipeline {
    stages: Vec<Stage>
}

struct Stage {
    name: String,
    command: String
}

fn parse_pipeline(pipeline_string: &str) -> Pipeline {
    let yaml: HashMap<String, Sequence> =
        serde_yaml::from_str(&pipeline_string).unwrap();

    let stages: Vec<Stage> =
        yaml
            .get("stages")
            .unwrap()
            .iter()
            .map(|value| {
                let stage = value.as_mapping().unwrap();

                return Stage {
                    name: stage.get("name").unwrap().as_str().unwrap().to_string(),
                    command: stage.get("command").unwrap().as_str().unwrap().to_string()
                };
            })
            .collect();

    return Pipeline { stages }
}

fn make_output_lines(stages: Vec<Stage>) -> Vec<String> {
    return
        stages
            .iter()
            .map(|stage| {
                return String::from("name: ") + &stage.name + ", command: " + &stage.command;
            })
            .collect();
}
