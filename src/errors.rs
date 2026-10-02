pub enum Error {
    NoPipelineFound(String),
    InvalidPipeline(String),
    UncommittedChanges,
    Other(String)
}

pub fn message_for(error: Error) -> String {
    match error {
        Error::NoPipelineFound(path) =>
            format!("Could not find pipeline at {}", path),
        Error::InvalidPipeline(reason) =>
            format!("Could not parse pipeline: {}", reason),
        Error::UncommittedChanges =>
            String::from("There are uncommited changes... aborting"),
        Error::Other(reason) => reason,
    }
}
