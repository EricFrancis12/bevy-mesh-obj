use std::{
    io,
    num::{ParseFloatError, ParseIntError},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("IO error: {0}")]
    IO(#[from] io::Error),

    #[error("Parse float error: {0}")]
    ParseFloat(#[from] ParseFloatError),

    #[error("Parse int error: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("Unrecognized token: {0}")]
    UnrecognizedToken(String),

    #[error("Expected .obj file to contain 1 object, but found {0} objects instead")]
    ParseSingleObj(usize),

    #[error("Expected object declaration")]
    MissingObjectDeclaration,

    #[error("Expected vertex string in the format: 'v [x] [y] [z]'")]
    InvalidVertexFormat,

    #[error("Expected normal string in the format: 'vn [x] [y] [z]'")]
    InvalidNormalFormat,

    #[error("Expected uv texture string in the format: 'vt [h] [v]'")]
    InvalidUVTextureFormat,

    #[error("Expected smoothing string in the format: 's [s]'")]
    InvalidSmoothingFormat,

    #[error("Expected face definition string in the format: '[i]/[j]/[k]'")]
    InvalidFaceDefinitionString,
}
