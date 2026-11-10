//!
//! Povides property traits for structured error handling, errors codes, severites, and help URLs.
//!

use crate::properties::help::HasUrl;
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

has_traits! {
    ErrorCode =>
    "Provides a mixin trait for types that have an `error_code` property."
    HasErrorCode { error_code }
    "Provides a mixin trait for types that have a mutable `error_code` property."
    HasErrorCodeMut { set_error_code }
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

has_traits! {
    Severity =>
    "Provides a mixin trait for types that have a `severity` property."
    HasSeverity { severity }
    "Provides a mixin trait for types that have a mutable `severity` property."
    HasSeverityMut { set_severity }
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Provides a trait for structured error types.
///
pub trait StructuredError: CoreError + HasErrorCode + HasSeverity + HasUrl {}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragments
// ------------------------------------------------------------------------------------------------

builder_fragment! {
    "Provides a builder mix-in fragment for types that can have an error code." =>
    ErrorCodeBuilderFragment Code: ErrorCode
        default with_error_code(code) {

        fn_with! {
            and_code(code: Self::Code) => with_error_code
        }
    }
}

builder_fragment! {
    "Provides a builder mix-in fragment for types that can have a severity." =>
    SeverityBuilderFragment impl Severity
        default with_severity(severity) {

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
}
