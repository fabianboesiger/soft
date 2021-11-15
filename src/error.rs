use std::fmt;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ErrorBuilder {
    message: Option<String>,
    file: Option<&'static str>,
    line: Option<u32>,
    column: Option<u32>,
}

impl ErrorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn file(mut self, file: &'static str) -> Self {
        self.file = Some(file);
        self
    }

    pub fn line(mut self, line: u32) -> Self {
        self.line = Some(line);
        self
    }

    pub fn column(mut self, column: u32) -> Self {
        self.column = Some(column);
        self
    }

    pub fn build(self) -> Error {
        Error {
            message: self.message,
            file: self.file.expect("File name required."),
            line: self.line.expect("Line number required."),
            column: self.column.expect("Column number required."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Error {
    message: Option<String>,
    file: &'static str,
    line: u32,
    column: u32,
}

impl Error {
    pub fn message(&self) -> Option<&str> {
        self.message.as_ref().map(|message| message.as_str())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(message) = &self.message {
            write!(f, "A soft assertion error occured in {}, line {}: {}", self.file, self.line, message)
        } else {
            write!(f, "A soft assertion error occured in {}, line {}.", self.file, self.line)
        }
    }
}

impl std::error::Error for Error {}