use std::fmt::{Display, Formatter};
use std::io;

pub struct Error {
    message: String,
    original: String,
}

impl Error {
    pub fn new(message: String) -> Error {
        return Error {
            message,
            original: "".to_string(),
        };
    }

    pub fn from(message: String, original: String) -> Error {
        return Error { message, original };
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let original = if self.original != "" {
            format!("\n({})", self.original)
        } else {
            "".to_string()
        };
        return write!(f, "Error: {}{}", self.message, original);
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        return Error::from(e.to_string(), e.to_string());
    }
}
