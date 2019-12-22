use std::io;

use err_derive::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "")]
    Io(#[error(cause)] io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
