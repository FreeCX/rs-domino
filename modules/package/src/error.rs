use std::array::TryFromSliceError;
use std::error;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

pub struct Error {
    repr: Repr,
}

#[derive(Debug)]
enum Repr {
    Io(io::Error),
    TryFrom(TryFromSliceError),
    Other(OtherError),
}

#[derive(Debug)]
pub enum OtherError {
    IncorrectFormat(usize),
    EmptyParentPath,
    InvalidFileSize,
    InvalidFileFormat,
    CorruptedData,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.repr, f)
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Repr::Io(ref e) => e.fmt(f),
            Repr::TryFrom(ref e) => e.fmt(f),
            Repr::Other(ref e) => e.fmt(f),
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
            OtherError::CorruptedData => write!(f, "Package data is corrupted"),
        }
    }
}

impl From<OtherError> for Error {
    fn from(error: OtherError) -> Self {
        Error { repr: Repr::Other(error) }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error { repr: Repr::Io(error) }
    }
}

impl From<TryFromSliceError> for Error {
    fn from(error: TryFromSliceError) -> Self {
        Error { repr: Repr::TryFrom(error) }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.repr)
    }
}

impl error::Error for Repr {}
