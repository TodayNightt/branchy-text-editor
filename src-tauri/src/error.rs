use serde::Serialize;
use std::fmt::Display;

trait Format {
    fn kind(&self) -> String;
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, specta::Type, Serialize)]
pub enum Response<T> {
    Success(T),
    Error(String),
}
impl<T> From<Result<T>> for Response<T>
where
    T: Serialize,
{
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(message) => Response::Success(message),
            Err(err) => Response::Error(err.to_string()),
        }
    }
}

#[derive(Debug, thiserror::Error, Serialize, specta::Type)]
pub enum PathError {
    #[error("Could not convert OsString to String")]
    ToStringError,
    #[error("Path not found")]
    PathNotFoundError,
}

#[derive(Debug, thiserror::Error, Serialize, specta::Type)]
pub enum FileError {
    #[error("Could not save the file {0}")]
    SavingFileError(String),
    #[error("Could not get file id : {0}")]
    GetFileError(u32),
    #[error("Could not read file from path : {0}")]
    ReadFileError(String),
    #[error("Language not supported for extension ({0})")]
    LanguageNotSupportError(String),
    #[error("Could not create file with error: {0}")]
    CreateFileError(String),
    #[error("The file contains non UTF-8 characters id : {0}")]
    InvalidUtf8StringError(u32),
}

#[derive(Debug, thiserror::Error, Serialize, specta::Type)]
pub enum NotFoundError {
    #[error("Tree not found for id : {0}")]
    TreeNotFoundError(u32),
    #[error("Parse not found for language : {0}")]
    ParserNotFoundError(String),
    #[error("Query not found for language : {0}")]
    QueryNotFoundError(String),
    #[error("Could not be found. file id : {0}")]
    FileNotFoundError(u32),
}

#[derive(Debug, thiserror::Error, Serialize, specta::Type)]
pub enum SerdeError {
    #[error("Could not serialize error : {0}")]
    SerializeError(String),
    #[error("Could not deserialize error : {0}")]
    DerializeError(String),
}

#[derive(Debug, thiserror::Error, specta::Type, Serialize)]
#[error("MutexLockError : {0}")]
pub struct MutexLockError(pub String);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("FileError : {0}")]
    FileError(#[from] FileError),
    #[error("NotFoundError : {0}")]
    NotFoundError(#[from] NotFoundError),
    #[error("PathError : {0}")]
    PathError(#[from] PathError),
    #[error("IOError : {0}")]
    IOError(#[from] std::io::Error),
    #[error("MutexLockError : {0}")]
    MutexLockError(#[from] MutexLockError),
    #[error("SerdeError : {0}")]
    SerdeError(#[from] SerdeError),
}

impl Format for PathError {
    fn kind(&self) -> String {
        match self {
            PathError::ToStringError => "PathError::ToStringError".to_string(),
            PathError::PathNotFoundError => "PathError::PathNotFoundError".to_string(),
        }
    }
}

impl Format for FileError {
    fn kind(&self) -> String {
        match self {
            FileError::GetFileError(_) => "FileError::GetFileError".to_string(),
            FileError::SavingFileError(_) => "FileError::SavingFileError".to_string(),
            FileError::ReadFileError(_) => "FileError::ReadFileError".to_string(),
            FileError::LanguageNotSupportError(_) => {
                "FileError::LanguageNotSupportError".to_string()
            }
            FileError::CreateFileError(_) => "FileError::CreateFileError".to_string(),
            FileError::InvalidUtf8StringError(_) => "FileError::InvalidUtf8StringError".to_string(),
        }
    }
}

impl Format for NotFoundError {
    fn kind(&self) -> String {
        match self {
            NotFoundError::ParserNotFoundError(_) => {
                "NotFoundError::ParserNotFoundError".to_string()
            }
            NotFoundError::QueryNotFoundError(_) => "NotFoundError::QueryNotFoundError".to_string(),
            NotFoundError::FileNotFoundError(_) => "NotFoundError::FileNotFoundError".to_string(),
            NotFoundError::TreeNotFoundError(_) => "NotFoundError::TreeNotFoundError".to_string(),
        }
    }
}

impl Format for MutexLockError {
    fn kind(&self) -> String {
        "MutexLockError".to_string()
    }
}

impl Format for SerdeError {
    fn kind(&self) -> String {
        match self {
            Self::SerializeError(_) => "SerdeError::SerializeError".to_string(),
            Self::DerializeError(_) => "SerdeError::DerializeError".to_string(),
        }
    }
}
#[derive(Debug, Serialize)]
pub struct ResponseError {
    kind: String,
    message: String,
}
