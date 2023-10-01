use std::env::VarError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    FileSystemError(String),
    IOError(std::io::Error),
    InvalidUtf8(FromUtf8Error),
    VarError(VarError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::FileSystemError(e) => write!(f, "FileSystemError: {}", e),
            Error::IOError(e) => write!(f, "I/O Error: {}", e),
            Error::InvalidUtf8(s) => write!(f, "InvalidUtf8: {}", s),
            Error::VarError(s) => write!(f, "VarError: {}", s),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}
impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::InvalidUtf8(e)
    }
}
impl From<VarError> for Error {
    fn from(e: VarError) -> Self {
        Error::VarError(e)
    }
}
