//!
//! Particle `[StructuredError]` trait.
//!

use crate::error::{code::ErrorCode, ErrorSeverity};
use core::{error::Error, fmt::Debug, hash::Hash};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait StructuredError: Error + Clone + Debug + PartialEq + Eq + Hash {
    type Code: ErrorCode;
    type Url;

    fn code(&self) -> Self::Code;
    fn severity(&self) -> ErrorSeverity;

    fn help_url_template() -> Self::Url;
    fn help_url(&self) -> Option<Self::Url>;
}
