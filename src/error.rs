use std::fmt;

#[derive(Debug)]
pub enum ForestError {
    UnableToParse,
    FailedToReadDir,
}

impl fmt::Display for ForestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ForestError::UnableToParse => write!(f, "unable to parse input"),
            ForestError::FailedToReadDir => write!(f, "Failed to read directory"),
        }
    }
}
