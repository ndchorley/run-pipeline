use std::collections::HashMap;
use serde_yaml::{Mapping, Sequence, Value};

use crate::Pipeline;
use crate::Stage;

pub fn parse_pipeline(pipeline_string: &str) -> Result<Pipeline, String> {
    parse_yaml(pipeline_string)
        .and_then(|yaml| find_stages_sequence(&yaml))
        .and_then(|stages_sequence| parse_stages(stages_sequence))
        .and_then(|stages| Ok(Pipeline { stages }))
}

fn parse_yaml(pipeline_string: &str) -> Result<HashMap<String, Sequence>, String> {
    serde_yaml::from_str::<HashMap<String, Sequence>>(&pipeline_string)
        .map_err(|_| { String::from("Could not parse pipeline") } )
}

fn find_stages_sequence(yaml: &HashMap<String, Vec<Value>>) -> Result<Vec<Value>, String> {
    match yaml.get("stages") {
        Some(stages_sequence) => Ok(stages_sequence.to_vec()),
        None => Err(String::from("Could not parse pipeline: missing a sequence called 'stages'"))
    }
}

fn parse_stages(stages_sequence: Vec<Value>) -> Result<Vec<Stage>, String> {
    stages_sequence
        .iter()
        .map(|value| parse_stage(value))
        .collect()
}

fn parse_stage(value: &Value) -> Result<Stage, String> {
    match value.as_mapping() {
        Some(stage) => Ok(
            Stage {
                name: mandatory_string(stage, "name"),
                command: mandatory_string(stage, "command")
            }
        ),

        None => Err(
            String::from("Could not parse pipeline: stage must be a mapping with keys 'name' and 'command'")
        )
    }
}

fn mandatory_string(mapping: &Mapping, field: &str) -> String {
    mapping.get(field).unwrap().as_str().unwrap().to_string()
}
