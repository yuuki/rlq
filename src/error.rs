use std::io;
use std::fmt;
use std::error;

#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &ParseError { ref msg } = self;
        write!(f, "Parse error: {}", msg)
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "parse error"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse(ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => write!(f, "IO error: {}", e),
            Error::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::Parse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::Parse(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parse(e)
    }
}
