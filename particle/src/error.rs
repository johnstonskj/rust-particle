/*!
Provides this crate's [`Error`] and [`Result`] types as well as helper functions.

 */

use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Clone, Debug)]
pub enum Error {
    /// Multiple errors were aggregated from some function below.
    MultipleErrors { sources: Vec<Error> },
    /// An unknown error occurred.
    Unknown { message: String },
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type Result<T> = StdResult<Error, T>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Construct an `Error` from the provided message.
#[inline]
pub fn unknown_error<S: Into<String>>(message: S) -> Error {
    Error::Unknown {
        message: message.into(),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::MultipleErrors { sources } => {
                    format!(
                        "Multiple errors occurred:\n{}",
                        sources
                            .iter()
                            .enumerate()
                            .map(|(i, e)| format!("{i:<3}. {e}"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                }
                Self::Unknown { message } =>
                    format!("An unknown error occurred; message: {message}"),
            }
        )
    }
}

impl StdError for Error {}

// ------------------------------------------------------------------------------------------------
// Implementations From
// ------------------------------------------------------------------------------------------------

impl From<Vec<Error>> for Error {
    fn from(sources: Vec<Error>) -> Self {
        Self::MultipleErrors { sources }
    }
}

impl From<&[Error]> for Error {
    fn from(sources: &[Error]) -> Self {
        Self::MultipleErrors {
            sources: sources.to_vec(),
        }
    }
}

impl FromIterator<Error> for Error {
    fn from_iter<I: IntoIterator<Item = Error>>(iter: I) -> Self {
        Self::MultipleErrors {
            sources: iter.into_iter().collect(),
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::Unknown { message }
    }
}
