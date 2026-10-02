use std::fs;

use crate::errors::Error::{self, NoPipelineFound};

pub fn read_file(pipeline_file: &str) -> Result<String, Error> {
    fs::read_to_string(pipeline_file)
        .map_err(|_| {
            NoPipelineFound(String::from(pipeline_file))
    })
}
