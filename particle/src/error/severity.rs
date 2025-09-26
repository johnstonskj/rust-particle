//!
//! Particle error _severity_.
//!

use core::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::Hash,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ErrorSeverity {
    Information = 1,
    Warning = 2,
    Error = 3,
    Critical = 4,
}

pub trait HasErrorSeverity {
    fn severity(&self) -> ErrorSeverity;
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ErrorSeverity {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match (self, f.alternate()) {
                (ErrorSeverity::Information, false) => "Information",
                (ErrorSeverity::Information, true) => "INFO",
                (ErrorSeverity::Warning, false) => "Warning",
                (ErrorSeverity::Warning, true) => "WARN",
                (ErrorSeverity::Error, false) => "Error",
                (ErrorSeverity::Error, true) => "ERROR",
                (ErrorSeverity::Critical, false) => "Critical",
                (ErrorSeverity::Critical, true) => "CRIT",
            }
        )
    }
}
