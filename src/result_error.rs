use std::error::Error as StdError;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Error {
    // macroquad
    MacroquadError(macroquad::Error),
    // misc
    UnknownError(Box<dyn StdError>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MacroquadError(err) => write!(f, "macroquad error: {}", err),
            Error::UnknownError(err) => write!(f, "unknown error: {}", err),
        }
    }
}

impl StdError for Error {}
