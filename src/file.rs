use std::fs;

use crate::errors::Error::{self, Other};

pub fn read_file(pipeline_file: &str) -> Result<String, Error> {
    fs::read_to_string(pipeline_file)
        .map_err(|_| {
            Other(String::from("Could not find pipeline at ") + pipeline_file)
    })
}
