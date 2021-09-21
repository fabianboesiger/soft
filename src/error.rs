use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Error {
    message: Option<String>,
}

impl Error {
    pub fn with_message(message: String) -> Self {
        Error {
            message: Some(message),
        }
    }

    pub fn without_message() -> Self {
        Error { message: None }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(message) = &self.message {
            write!(f, "A soft assertion error occured: {}", message)
        } else {
            write!(f, "A soft assertion error occured.")
        }
    }
}

impl std::error::Error for Error {}

pub type Result = std::result::Result<(), Error>;
