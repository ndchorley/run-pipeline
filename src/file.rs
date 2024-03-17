use std::fs;

pub fn read_file(pipeline_file: &str) -> Result<String, String> {
    fs::read_to_string(pipeline_file)
        .map_err(|_| {
            String::from("Could not find pipeline at ") + pipeline_file
    })
}
