use capnp;
use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub struct Error {
    description: &'static str
}

impl Error {
    pub fn new(description: &'static str) -> Error {
        Error { description: description }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.description
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

impl From<capnp::Error> for Error {
    fn from(err: capnp::Error) -> Error {
        match err {
            capnp::Error::Decode { description: d, detail: _ } => Error { description: d },
            capnp::Error::Io(_) => Error { description: "IO Error" }
        }
    }
}

impl From<&'static str> for Error {
    fn from(err: &'static str) -> Error {
        Error { description: err }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
