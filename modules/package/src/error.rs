use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Other(OtherError),
}

#[derive(Debug)]
pub enum OtherError {
    IncorrectFormat(usize),
    EmptyParentPath,
    InvalidFileSize,
    InvalidFileFormat,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(ref e) => e.fmt(f),
            Error::Other(ref e) => e.fmt(f),
        }
    }
}

impl fmt::Display for OtherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OtherError::IncorrectFormat(line) => write!(f, "Incorrect item format at line {line}"),
            OtherError::EmptyParentPath => write!(f, "List file does not have parent"),
            OtherError::InvalidFileSize => write!(f, "Invalid file size"),
            OtherError::InvalidFileFormat => write!(f, "Invalid file format"),
        }
    }
}

impl From<OtherError> for Error {
    fn from(error: OtherError) -> Self {
        Error::Other(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}
