use std::{
    io,
    num::{ParseFloatError, ParseIntError},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("IO error: {0}")]
    IOError(#[from] io::Error),

    #[error("Parsing error: {0}")]
    ParseFloatError(#[from] ParseFloatError),

    #[error("Parsing error: {0}")]
    ParseIntError(#[from] ParseIntError),
}
