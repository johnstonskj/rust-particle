//!
//! Provides error type for Particle properties.
//!

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::Hash,
};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Value Error
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PropertyError {
    ValueOutOfBounds {
        value_type: &'static str,
        exceeded: ValueBound,
    },
    InvalidValueRepresentation {
        value_type: &'static str,
        invalid_repr: String,
        reason: Option<String>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValueBound {
    Min,
    Max,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Value Error / Bound
// ------------------------------------------------------------------------------------------------

impl Display for PropertyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ValueOutOfBounds {
                value_type,
                exceeded,
            } => write!(
                f,
                "Value specified for `{value_type}` is outside the {exceeded} bounds."
            ),
            Self::InvalidValueRepresentation {
                value_type,
                invalid_repr,
                reason,
            } => write!(
                f,
                "String representation '{invalid_repr}' is not valid for type `{value_type}`.{}",
                match reason {
                    Some(reason) => format!(" Reason: {reason}."),
                    None => String::new(),
                },
            ),
        }
    }
}

impl Error for PropertyError {}

impl PropertyError {
    pub fn beyond_maximum_value(value_type: &'static str) -> Self {
        Self::ValueOutOfBounds {
            value_type,
            exceeded: ValueBound::Max,
        }
    }
    pub fn beyond_minimum_value(value_type: &'static str) -> Self {
        Self::ValueOutOfBounds {
            value_type,
            exceeded: ValueBound::Min,
        }
    }
    pub fn invalid_representation<S>(value_type: &'static str, invalid_repr: S) -> Self
    where
        S: Into<String>,
    {
        Self::InvalidValueRepresentation {
            value_type,
            invalid_repr: invalid_repr.into(),
            reason: None,
        }
    }
    pub fn invalid_representation_because<S1, S2>(
        value_type: &'static str,
        invalid_repr: S1,
        reason: S2,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self::InvalidValueRepresentation {
            value_type,
            invalid_repr: invalid_repr.into(),
            reason: Some(reason.into()),
        }
    }
}

impl Display for ValueBound {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Min => "minimum",
                Self::Max => "maximum",
            }
        )
    }
}
