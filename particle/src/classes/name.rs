//!
//! Provides traits, and classifiers, for Particle names.
//!

use crate::{
    names::Name,
    strings::AsStr,
    validation::{
        error::ValidationIssue,
        issues::{ValueInvalidRepresentation, ValueInvalidRepresentationReason},
    },
};
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};
use core::fmt::{Display, Formatter, Result as FmtResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Name Implementation for Testing
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ClassifierName(&'static str);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const CLASSIFIER_NAME_STR: &str = "ClassifierName";
const CLASSIFIER_NAME: ClassifierName = ClassifierName::new_unchecked(CLASSIFIER_NAME_STR);

impl Display for ClassifierName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl From<ClassifierName> for String {
    #[inline(always)]
    fn from(name: ClassifierName) -> Self {
        name.0.to_string()
    }
}

impl AsStr for ClassifierName {
    #[inline(always)]
    fn as_str(&self) -> &str {
        self.0
    }
}

impl ClassifierName {
    pub const fn new_unchecked(s: &'static str) -> Self {
        Self(s)
    }
    pub fn from_static_str<N: Name>(s: &'static str) -> Result<Self, ValidationIssue<N>> {
        if s.is_empty() {
            #[cfg(feature = "tracing")]
            tracing::error!("Empty string");
            Err(ValueInvalidRepresentation::new_type(
                CLASSIFIER_NAME.to_name::<N>(),
                s.to_string(),
                ValueInvalidRepresentationReason::Empty,
            )
            .into())
        } else if !s.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
            #[cfg(feature = "tracing")]
            tracing::error!("Invalid first character: {s:?}");
            Err(ValueInvalidRepresentation::new_type(
                CLASSIFIER_NAME.to_name::<N>(),
                s.to_string(),
                ValueInvalidRepresentationReason::InvalidCharacter,
            )
            .into())
        } else if s
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ':')
            && s.chars().any(|c| c.is_ascii_alphabetic())
        {
            Ok(Self(s))
        } else {
            #[cfg(feature = "tracing")]
            tracing::error!("Invalid character in {s:?}");
            Err(ValueInvalidRepresentation::new_type(
                CLASSIFIER_NAME.to_name::<N>(),
                s.to_string(),
                ValueInvalidRepresentationReason::InvalidCharacter,
            )
            .into())
        }
    }
    pub fn is_valid<N: Name>(s: &'static str) -> bool {
        Self::from_static_str::<N>(s).is_ok()
    }
    pub fn to_name<N: Name>(&self) -> N {
        N::new_unchecked(self.0)
    }
}
