//!
//! Particle Builder errors.
//!

use core::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Errors
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuilderError {
    MissingFields {
        field_names: Vec<String>,
    },
    IncompatibleFields {
        field_name: String,
        incompatible_with: Vec<String>,
    },
    InvalidValue {
        field_name: String,
        value_as_string: String,
    },
    //    Validation(ValidationError),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for BuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::MissingFields { field_names } => {
                write!(f, "Missing required fields: {}", field_names.join(", "))
            }
            Self::IncompatibleFields {
                field_name,
                incompatible_with,
            } => write!(
                f,
                "Field '{field_name}' is incompatible with: {}",
                incompatible_with.join(", ")
            ),
            Self::InvalidValue {
                field_name,
                value_as_string,
            } => write!(
                f,
                "Invalid value '{value_as_string}' for field '{field_name}'"
            ),
            //            Self::Validation(e) => write!(
            //                f,
            //                "{} validation failed with {} issue(s):\n{}",
            //                e.class_name(),
            //                e.issue_count(),
            //                e.issues()
            //                    .map(|issue| issue.to_string())
            //                    .collect::<Vec<String>>()
            //                    .join("\n")
            //            ),
        }
    }
}

impl Error for BuilderError {}

impl BuilderError {
    pub fn missing_field<S: Into<String>>(field_name: S) -> Self {
        Self::MissingFields {
            field_names: vec![field_name.into()],
        }
    }
    pub fn missing_fields<V: Into<Vec<String>>>(field_names: V) -> Self {
        Self::MissingFields {
            field_names: field_names.into().iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn incompatible_fields<S: Into<String>, V: Into<Vec<String>>>(
        field_name: S,
        incompatible_with: V,
    ) -> Self {
        Self::IncompatibleFields {
            field_name: field_name.into(),
            incompatible_with: incompatible_with
                .into()
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }

    pub fn invalid_value<S: Into<String>, V: Into<String>>(
        field_name: S,
        value_as_string: V,
    ) -> Self {
        Self::InvalidValue {
            field_name: field_name.into(),
            value_as_string: value_as_string.into(),
        }
    }

    //    pub fn validation_failed(error: ValidationError) -> Self {
    //        Self::Validation(error)
    //    }
}
