use std::io;
use std::fmt;
use std::error;

macro_rules! stderr {
    ( $( $msg:tt )* ) => {{
        writeln!(io::stderr(), $($msg)*).unwrap();
    }}
}

#[derive(Debug)]
pub enum CliError {
    NotEnoughArgs,
    TooManyArgs,
    Other,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::NotEnoughArgs => write!(f, "too few arguments"),
            CliError::TooManyArgs => write!(f, "too many arguments"),
            CliError::Other => write!(f, "other"),
        }
    }
}

impl error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::NotEnoughArgs => "too few arguments",
            CliError::TooManyArgs => "too many arguments",
            CliError::Other => "other",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CliError::NotEnoughArgs => None,
            CliError::TooManyArgs => None,
            CliError::Other => None,
        }
    }
}

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
            Error::Io(ref e) => write!(f, "{}", e),
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
