//!
//! Provides traits, and classifiers, for Particle names.
//!

use crate::{
    names::Name,
    strings::{AsStr, CheckedFromStr, UncheckedNewStr},
    validation::{
        error::ValidationIssue,
        issues::{ValueInvalidRepresentation, ValueInvalidRepresentationReason},
    },
};
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Name Implementation for Testing
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TestingName(String);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl AsRef<str> for TestingName {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for TestingName {
    type Err = ValidationIssue<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ValueInvalidRepresentation::new_type(
                TestingName::new_unchecked("TestingName"),
                s.to_string(),
                ValueInvalidRepresentationReason::Empty,
            )
            .into())
        } else if s
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ':')
            && s.chars().any(|c| c.is_ascii_alphabetic())
        {
            Ok(Self(s.to_string()))
        } else {
            Err(ValueInvalidRepresentation::new_type(
                TestingName::new_unchecked("TestingName"),
                s.to_string(),
                ValueInvalidRepresentationReason::InvalidCharacter,
            )
            .into())
        }
    }
}

impl From<TestingName> for String {
    #[inline(always)]
    fn from(name: TestingName) -> Self {
        name.0
    }
}

impl Display for TestingName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl AsStr for TestingName {
    fn as_str(&self) -> &str {
        &self.0
    }
}

impl CheckedFromStr for TestingName {
    /* default */
}

impl UncheckedNewStr for TestingName {
    fn new_unchecked<S: AsRef<str>>(s: S) -> Self {
        Self(s.as_ref().to_string())
    }
}

impl Name for TestingName {}
