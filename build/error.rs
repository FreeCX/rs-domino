use std::error;
use std::fmt;
use std::io;
use std::string;

#[derive(Debug)]
pub struct BuildError {
    repr: Repr,
}

#[derive(Debug)]
enum Repr {
    Io(io::Error),
    Parse(string::FromUtf8Error),
}

impl From<io::Error> for BuildError {
    fn from(error: io::Error) -> Self {
        BuildError { repr: Repr::Io(error) }
    }
}

impl From<string::FromUtf8Error> for BuildError {
    fn from(error: string::FromUtf8Error) -> Self {
        BuildError { repr: Repr::Parse(error) }
    }
}

impl error::Error for BuildError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.repr)
    }
}

impl error::Error for Repr {}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}
