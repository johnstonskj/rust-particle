//!
//! Povides this crate's [`Error`] and [`Result`] types as well as helper functions.
//!

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{
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
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Multiple errors were aggregated from some function below.
    Multiple { sources: Vec<Error> },
    /// An unknown error occurred.
    Unknown { message: String },
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type Result<T> = StdResult<Error, T>;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Multiple { sources } => {
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

impl Error {
    /// Construct an `Error` from the provided message.
    #[inline]
    pub fn unknown<S: Into<String>>(message: S) -> Self {
        Self::Unknown {
            message: message.into(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations From<*>
// ------------------------------------------------------------------------------------------------

impl From<Vec<Error>> for Error {
    fn from(sources: Vec<Error>) -> Self {
        Self::Multiple { sources }
    }
}

impl From<&[Error]> for Error {
    fn from(sources: &[Error]) -> Self {
        Self::Multiple {
            sources: sources.to_vec(),
        }
    }
}

impl FromIterator<Error> for Error {
    fn from_iter<I: IntoIterator<Item = Error>>(iter: I) -> Self {
        Self::Multiple {
            sources: iter.into_iter().collect(),
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::Unknown { message }
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Self::Unknown {
            message: message.to_string(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod severity;
pub use severity::{ErrorSeverity, HasErrorSeverity};

pub mod code;
pub use code::ErrorCode;

pub mod scope;
pub use scope::ErrorHasScope;

#[cfg(feature = "structured-errors")]
pub mod structured;
#[cfg(feature = "structured-errors")]
pub use structured::StructuredError;
