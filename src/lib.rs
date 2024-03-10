use std::{collections::HashMap, fs, io::Write};

use serde_yaml::Sequence;

pub fn run(writer: &mut impl Write) {
    let pipeline_string =
        fs::read_to_string("tests/pipeline.yml").unwrap();

    let yaml: HashMap<String, Sequence> =
        serde_yaml::from_str(&pipeline_string).unwrap();

    let stages: Vec<String> =
        yaml
        .get("stages")
        .unwrap()
        .iter()
        .map(|value| {
            let stage = value.as_mapping().unwrap();

            let name =
                stage.get("name").unwrap().as_str().unwrap();

            let command =
                stage.get("command").unwrap().as_str().unwrap();

            return String::from("name: ") + name + ", command: " + command;
        })
        .collect();

    let result = writeln!(writer, "{}", stages.as_slice().join("\n"));

    match result {
        Ok(_) => (),
        Err(_) => ()
    }
}
