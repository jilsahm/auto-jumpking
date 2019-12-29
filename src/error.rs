use std::{
    fmt::{Display, Formatter},
    io::Error,
};

pub enum FileError {
    InvalidContentError(String),
    IoError(Error),
}

impl Display for FileError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "{}",
            match self {
                FileError::InvalidContentError(what) => what.to_string(),
                FileError::IoError(e) => e.to_string(),
            }
        )
    }
}

impl From<Error> for FileError {
    fn from(e: Error) -> Self {
        FileError::IoError(e)
    }
}

pub enum ArgsError {
    MissingLevelParameter,
    IoError
}

impl Display for ArgsError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "{}",
            match self {
                ArgsError::MissingLevelParameter => "The level paramter is missing".to_string(),
                ArgsError::IoError => "Given level file could not be read. Be sure that the file exists in the resource folder ".to_string(),
            }
        )
    }
}

impl From<Error> for ArgsError {
    fn from(_: Error) -> Self {
        ArgsError::IoError
    }
}