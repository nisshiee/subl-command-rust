use std::error::{Error as StdError};
use std::ffi::OsString;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    InvalidArgument(OsString),
    UnexpectedIoError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidArgument(ref arg)     => write!(f, "Invalid argument: {}", arg.to_string_lossy()),
            Error::UnexpectedIoError(ref error) => write!(f, "Unexpected IO error: {}", error),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidArgument(_)   => "Invalid argument",
            Error::UnexpectedIoError(_) => "Unexpected IO error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::UnexpectedIoError(ref error) => Some(error),
            _                                   => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::UnexpectedIoError(err)
    }
}
