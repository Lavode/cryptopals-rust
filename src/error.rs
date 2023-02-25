use std::fmt::Display;

pub enum Error {
    GenericError(String),
    DataError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::GenericError(msg) => {
                write!(f, "{}", msg)
            }
            Error::DataError(msg) => {
                write!(f, "Data error: {}", msg)
            }
        }
    }
}
