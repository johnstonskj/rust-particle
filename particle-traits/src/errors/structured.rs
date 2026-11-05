//!
//! Particle `[StructuredError]` trait.
//!

use crate::errors::{code::ErrorCode, severity::Severity};
use core::{error::Error, fmt::Debug, hash::Hash};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait StructuredError: Error + Clone + Debug + PartialEq + Eq + Hash {
    type Code: ErrorCode;
    type Severity: Severity;
    type Url;

    fn code(&self) -> Self::Code;
    fn severity(&self) -> Self::Severity;

    fn help_url_template() -> Self::Url;
    fn help_url(&self) -> Option<Self::Url>;
}
