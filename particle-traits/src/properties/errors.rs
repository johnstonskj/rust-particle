//!
//! Povides property traits for structured error handling, errors codes, severites, and help URLs.
//!

use core::{
    error::Error as CoreError,
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Error Code
// ------------------------------------------------------------------------------------------------

/// 
/// Provides a trait for error code types.
/// 
pub trait ErrorCode: Clone + Copy + Debug + Display + PartialEq + Eq + Hash {}


/// 
/// Provides a trait for types that have an associated error code.
/// 
pub trait HasErrorCode {
    type Code: ErrorCode;

    fn error_code(&self) -> Self::Code;
}

/// 
/// Provides a trait for types that have a mutable associated error code.
/// 
pub trait HasErrorCodeMut: HasErrorCode {
    fn set_error_code(&mut self, code: Self::Code);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Severity
// ------------------------------------------------------------------------------------------------

/// 
/// Provides a trait for severity types.
/// 
pub trait Severity:
    Clone + Copy + Debug + Display + PartialEq + Eq + Hash + PartialOrd + Ord + FromStr
{
    fn warning() -> Self;
    fn is_warning(self) -> bool;
    fn as_warning(&self) -> Option<&Self> {
        if self.is_warning() { Some(self) } else { None }
    }

    fn error() -> Self;
    fn is_error(self) -> bool;
    fn as_error(&self) -> Option<&Self> {
        if self.is_error() { Some(self) } else { None }
    }

    fn critical() -> Self;
    fn is_critical(self) -> bool;
    fn as_critical(&self) -> Option<&Self> {
        if self.is_critical() { Some(self) } else { None }
    }

    fn highest_value() -> Self;
    fn lowest_value() -> Self;
    fn is_highest(&self) -> bool {
        *self == Self::highest_value()
    }
    fn is_lowest(&self) -> bool {
        *self == Self::lowest_value()
    }
}

/// 
/// Provides a trait for types that have an associated severity.
/// 
pub trait HasSeverity {
    type Severity: Severity;

    fn severity(&self) -> Self::Severity;
}

/// 
/// Provides a trait for types that have a mutable associated severity.
/// 
pub trait HasSeverityMut: HasSeverity {
    fn set_severity(&mut self, severity: Self::Severity);
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------
 
/// 
/// Provides a trait for structured error types.
/// 
pub trait StructuredError: CoreError + HasErrorCode + HasSeverity {
    type Url;

    fn help_url(&self) -> Option<Self::Url>;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragments
// ------------------------------------------------------------------------------------------------

/// 
/// Provides a builder mix-in fragment for types that can have an error code.
/// 
pub trait ErrorCodeBuilderFragment {
    type Code: ErrorCode;

    fn with_error_code(self, code: Self::Code) -> Self;
    fn and_code(self, code: Self::Code) -> Self;
}

/// 
/// Provides a builder mix-in fragment for types that can have a severity.
/// 
pub trait SeverityBuilderFragment {
    type Severity: Severity;

    fn with_severity(self, severity: Self::Severity) -> Self;

    fn warning(self) -> Self
    where
        Self: Sized,
    {
        self.with_severity(Self::Severity::warning())
    }

    fn error(self) -> Self
    where
        Self: Sized,
    {
        self.with_severity(Self::Severity::error())
    }

    fn critical(self) -> Self
    where
        Self: Sized,
    {
        self.with_severity(Self::Severity::critical())
    }
}
