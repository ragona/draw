use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IOError;

///
/// Encapsulates every kind of error that can happen in the draw module
///
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

#[derive(Debug)]
pub enum ErrorKind {
    Io(IOError),
    InvalidSpriteId,
    __Nonexhaustive,
}

pub fn new_error(kind: ErrorKind) -> Error {
    Error(Box::new(kind))
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Io(ref err) => write!(f, "{}", err),
            ErrorKind::InvalidSpriteId => write!(f, "Invalid Sprite ID"),
            _ => unreachable!(),
        }
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        new_error(ErrorKind::Io(err))
    }
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self.0 {
            ErrorKind::Io(ref err) => err.description(),
            _ => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self.0 {
            ErrorKind::Io(ref err) => Some(err),
            _ => unreachable!(),
        }
    }
}
