pub enum Error {
    InvalidPipeline(String),
    UncommittedChanges,
    Other(String)
}

pub fn message_for(error: Error) -> String {
    match error {
        Error::InvalidPipeline(reason) =>
            format!("Could not parse pipeline: {}", reason),
        Error::UncommittedChanges =>
            String::from("There are uncommited changes... aborting"),
        Error::Other(reason) => reason,
    }
}
