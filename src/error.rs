use id3;
use std::{fmt, io, str};

/// A set of errors that can occur when interfacing with the Open-in-Live Service
#[derive(Debug)]
pub enum Error {
    /// File related error
    File(String),
    /// ID3 Error
    Id3(id3::Error),
    /// I/O operation error
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::File(ref err) => write!(f, "{}", err),
            Error::Id3(ref err) => write!(f, "{}", err),
            Error::Io(ref err) => write!(f, "{}", err),
        }
    }
}

impl From<id3::Error> for Error {
    fn from(err: id3::Error) -> Error {
        Error::Id3(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
