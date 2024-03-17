use std::collections::HashMap;
use serde_yaml::{Mapping, Sequence, Value};

use crate::Pipeline;
use crate::Stage;

pub fn parse_pipeline(pipeline_string: &str) -> Result<Pipeline, ()> {
    let yaml: HashMap<String, Sequence> =
        serde_yaml::from_str(&pipeline_string).unwrap();

    let stages =
        yaml
            .get("stages")
            .unwrap()
            .iter()
            .map(|value| parse_stage(value))
            .collect();

    Ok(Pipeline { stages })
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
