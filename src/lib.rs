use std::{collections::HashMap, fs, io::Write};

use serde_yaml::{Mapping, Sequence, Value};

pub fn run(pipeline_file: &str, writer: &mut impl Write) {
    let read_pipeline_result =
        fs::read_to_string(pipeline_file);

    match read_pipeline_result {
        Ok(pipeline_string) => {
            let pipeline = parse_pipeline(&pipeline_string);

            display_pipeline(pipeline, writer);
        }

        Err(_) => {
            let message =
                String::from("Could not find pipeline at ") + pipeline_file;

            writeln!(writer, "{}", message).unwrap();
        }
    }
}

struct Pipeline {
    stages: Vec<Stage>
}

struct Stage {
    name: String,
    command: String
}

fn display_pipeline(pipeline: Pipeline, writer: &mut impl Write) {
    let output_lines = make_output_lines(pipeline.stages);

    writeln!(writer, "{}", output_lines.as_slice().join("\n"))
        .unwrap();
}

fn parse_pipeline(pipeline_string: &str) -> Pipeline {
    let yaml: HashMap<String, Sequence> =
        serde_yaml::from_str(&pipeline_string).unwrap();

    let stages: Vec<Stage> =
        yaml
            .get("stages")
            .unwrap()
            .iter()
            .map(|value| parse_stage(value))
            .collect();

    Pipeline { stages }
}

fn parse_stage(value: &Value) -> Stage {
    let stage = value.as_mapping().unwrap();

    Stage {
        name: mandatory_string(stage, "name"),
        command: mandatory_string(stage, "command")
    }
}

fn mandatory_string(mapping: &Mapping, field: &str) -> String {
    mapping.get(field).unwrap().as_str().unwrap().to_string()
}

fn make_output_lines(stages: Vec<Stage>) -> Vec<String> {
    stages
        .iter()
        .map(|stage|
            String::from("name: ") + &stage.name + ", command: " + &stage.command
        )
        .collect()
}
