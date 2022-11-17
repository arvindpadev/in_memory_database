use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    description: String
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error {
    pub fn new(description: String) -> Error {
        Error { description }
    }
}