pub enum Error {
    InvalidPipeline(String),
    Other(String)
}

pub fn message_for(error: Error) -> String {
    match error {
        Error::InvalidPipeline(reason) =>
            format!("{}", reason),
        Error::Other(reason) => reason,
    }
}
