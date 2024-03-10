use std::{collections::HashMap, fs, io::Write};

use serde_yaml::Sequence;

pub fn run(pipeline_file: &str, writer: &mut impl Write) {
    let pipeline_string =
        fs::read_to_string(pipeline_file).unwrap();

    let stages: Vec<String> =
        parse_stages(&pipeline_string).iter()
        .map(|stage| {
            return String::from("name: ") + &stage.name + ", command: " + &stage.command;
        })
        .collect();

    let result = writeln!(writer, "{}", stages.as_slice().join("\n"));

    match result {
        Ok(_) => (),
        Err(_) => ()
    }
}

struct Stage {
    name: String,
    command: String
}

fn parse_stages(pipeline_string: &str) -> Vec<Stage> {
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

    return stages
}
