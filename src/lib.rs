use std::error;
use std::fmt;

pub mod days;

pub type Result<T> = std::result::Result<T, AdventError>;

#[derive(Debug)]
pub enum AdventError {
    MissingArgument(String),
    NotImplemented(u32, u32),
    UnexpectedError(String),
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl fmt::Display for AdventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::MissingArgument(ref s) => write!(f, "'{}' is a required argument.", s),
            Self::NotImplemented(d, p) => write!(f, "Day: {}, Part: {} not implemented yet.", d, p),
            Self::UnexpectedError(ref s) => write!(f, "{}", s),
            Self::Io(ref e) => e.fmt(f),
            Self::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for AdventError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::MissingArgument(_) => None,
            Self::NotImplemented(_, _) => None,
            Self::UnexpectedError(_) => None,
            Self::Io(ref e) => Some(e),
            Self::Parse(ref e) => Some(e),
        }
    }
}

impl From<std::io::Error> for AdventError {
    fn from(err: std::io::Error) -> Self {
        AdventError::Io(err)
    }
}

impl From<std::num::ParseIntError> for AdventError {
    fn from(err: std::num::ParseIntError) -> Self {
        AdventError::Parse(err)
    }
}
