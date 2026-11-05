//!
//! Particle Validation errors.
//!

use crate::{
    error::{ErrorHasScope, ErrorSeverity, HasErrorSeverity},
    names::Name,
    validation::issues::{
        ObjectHasIncompatibleProperties, ObjectMissingDependentProperties,
        ObjectMissingRequiredProperties, ValueInvalidRepresentation, ValueMissing,
        ValueMissingFrom, ValueOfDependentPropertyInvalid, ValueOutOfBounds,
    },
};
#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    slice,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! trace_error {
    ($err:expr) => {
        trace_error!("" => $err);
    };
    ($during:expr => $err:expr) => {
        let err_during = $during.to_string();
        let max_severity = $err.max_severity();
        let _err_span = match max_severity {
            ErrorSeverity::Critical => {
                trace_error!(
                    @error_match
                    error_span, error;
                    err_during, max_severity.to_string(), $err.scope.to_string(),
                    "Critical validation error"
                )
            }
            ErrorSeverity::Error => {
                trace_error!(
                    @error_match
                    error_span, error;
                    err_during, max_severity.to_string(), $err.scope.to_string(),
                    "Validation error"
                )
            }
            ErrorSeverity::Warning => {
                trace_error!(
                    @error_match
                    warn_span, warn;
                    err_during, max_severity.to_string(), $err.scope.to_string(),
                    "Validation warning"
                )
            }
            ErrorSeverity::Information => {
                trace_error!(
                    @error_match
                    info_span, info;
                    err_during, max_severity.to_string(), $err.scope.to_string(),
                    "Validation issue"
                )
            }
        };
        for issue in $err.issues {
            let severity = issue.severity();
            match severity {
                ErrorSeverity::Critical => trace_error!(@issue_match error; severity, issue),
                ErrorSeverity::Error => trace_error!(@issue_match error; severity, issue),
                ErrorSeverity::Warning => trace_error!(@issue_match warn; severity, issue),
                ErrorSeverity::Information => trace_error!(@issue_match info; severity, issue),
            }
        }
    };
    (
        @error_match
        $span_macro:ident, $trace_macro:ident;
        $during:expr, $max_severity:expr, $scope:expr, $prefix:expr
    ) => {{
        let _span = ::tracing::$span_macro!("validation");
        if $during.is_empty() {
            ::tracing::error!(
                max_severity = $max_severity,
                scope = $scope,
                "{} found for scope `{}`.",
                $prefix,
                $scope
            );
        } else {
            ::tracing::error!(
                max_severity = $max_severity,
                scope = $scope,
                during = $during,
                "{} found for scope `{}`.",
                $prefix,
                $scope
            );
        }
        _span
    }};
    (
        @issue_match
        $trace_macro:ident;
        $severity:expr, $issue:expr
    ) => {
        ::tracing::$trace_macro!(severity = $severity.to_string(), "{}", $issue)
    };
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Validation Errors
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ValidationError<N: Name> {
    scope: N,
    issues: Vec<ValidationIssue<N>>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ValidationIssue<N: Name> {
    ObjectMissingRequiredProperties(ObjectMissingRequiredProperties<N>),
    ObjectMissingDependentProperties(ObjectMissingDependentProperties<N>),
    ObjectHasIncompatibleProperties(ObjectHasIncompatibleProperties<N>),
    ValueInvalidRepresentation(ValueInvalidRepresentation<N>),
    ValueMissing(ValueMissing<N>),
    ValueMissingFrom(ValueMissingFrom<N>),
    ValueOutOfBounds(ValueOutOfBounds<N>),
    ValueOfDependentPropertyInvalid(ValueOfDependentPropertyInvalid<N>),
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Validation Error
// ------------------------------------------------------------------------------------------------

impl<N: Name> From<(N, ValidationIssue<N>)> for ValidationError<N> {
    fn from((scope, issues): (N, ValidationIssue<N>)) -> Self {
        Self::new(scope, &[issues])
    }
}

impl<N: Name> From<(N, &ValidationIssue<N>)> for ValidationError<N> {
    fn from((scope, issues): (N, &ValidationIssue<N>)) -> Self {
        Self::new(scope, slice::from_ref(issues))
    }
}

impl<N: Name> From<(N, Vec<ValidationIssue<N>>)> for ValidationError<N> {
    fn from((scope, issues): (N, Vec<ValidationIssue<N>>)) -> Self {
        Self::new(scope, &issues)
    }
}

impl<N: Name> From<(N, &[ValidationIssue<N>])> for ValidationError<N> {
    fn from((scope, issues): (N, &[ValidationIssue<N>])) -> Self {
        Self::new(scope, issues)
    }
}

impl<N: Name> Display for ValidationError<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Validation for the scope named `{}` failed with {} issue(s):\n{}",
            self.scope(),
            self.issue_count(),
            self.issues()
                .map(|issue| issue.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl<N: Name> Error for ValidationError<N> {}

impl<N: Name> ErrorHasScope for ValidationError<N> {
    type Name = N;

    fn scope(&self) -> &Self::Name {
        &self.scope
    }
}

impl<N: Name> ValidationError<N> {
    pub fn new(scope: N, issues: &[ValidationIssue<N>]) -> Self {
        Self {
            scope,
            issues: issues.to_vec(),
        }
    }

    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }

    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    pub fn issues(&self) -> impl Iterator<Item = &ValidationIssue<N>> {
        self.issues.iter()
    }

    pub fn max_severity(&self) -> ErrorSeverity {
        self.issues
            .iter()
            .map(|issue| issue.severity())
            .max()
            .unwrap_or(ErrorSeverity::Information)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Validation Issue
// ------------------------------------------------------------------------------------------------

impl<N: Name> Display for ValidationIssue<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}{}",
            if f.alternate() {
                format!("[{:#}] ", self.severity())
            } else {
                String::new()
            },
            match self {
                Self::ObjectMissingRequiredProperties(issue) => issue.to_string(),
                Self::ObjectMissingDependentProperties(issue) => issue.to_string(),
                Self::ObjectHasIncompatibleProperties(issue) => issue.to_string(),
                Self::ValueInvalidRepresentation(issue) => issue.to_string(),
                Self::ValueMissing(issue) => issue.to_string(),
                Self::ValueMissingFrom(issue) => issue.to_string(),
                Self::ValueOutOfBounds(issue) => issue.to_string(),
                Self::ValueOfDependentPropertyInvalid(issue) => issue.to_string(),
            }
        )
    }
}

impl<N: Name> Error for ValidationIssue<N> {}

impl<N: Name> HasErrorSeverity for ValidationIssue<N> {
    fn severity(&self) -> ErrorSeverity {
        match self {
            Self::ObjectMissingRequiredProperties(issue) => issue.severity(),
            Self::ObjectMissingDependentProperties(issue) => issue.severity(),
            Self::ObjectHasIncompatibleProperties(issue) => issue.severity(),
            Self::ValueInvalidRepresentation(issue) => issue.severity(),
            Self::ValueMissing(issue) => issue.severity(),
            Self::ValueMissingFrom(issue) => issue.severity(),
            Self::ValueOutOfBounds(issue) => issue.severity(),
            Self::ValueOfDependentPropertyInvalid(issue) => issue.severity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error::ErrorHasScope,
        names::testing::TestingName,
        strings::UncheckedNewStr,
        validation::{issues::ObjectMissingRequiredProperties, ValidationError, ValidationIssue},
    };

    #[test]
    fn test_validation_error() {
        let err = ValidationError::new(
            TestingName::new_unchecked("Thing"),
            &[ValidationIssue::ObjectMissingRequiredProperties(
                ObjectMissingRequiredProperties::new(&[TestingName::new_unchecked("a_thing")]),
            )],
        );
        assert_eq!(err.scope(), &TestingName::new_unchecked("Thing"));
        assert_eq!(err.issue_count(), 1);
    }
}

#[cfg(all(test, feature = "tracing", not(feature = "std")))]
mod tracing_macro_tests {
    use crate::{
        error::{ErrorSeverity, HasErrorSeverity},
        names::testing::TestingName,
        strings::UncheckedNewStr,
        validation::{issues::ObjectMissingRequiredProperties, ValidationError, ValidationIssue},
    };
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_trace_error_macro() {
        let err = ValidationError::new(
            TestingName::new_unchecked("Thing"),
            &[ValidationIssue::ObjectMissingRequiredProperties(
                ObjectMissingRequiredProperties::new(&[TestingName::new_unchecked("a_thing")]),
            )],
        );
        trace_error!("Rule = data_model.required" => err);

        // For the outer error:
        assert!(logs_contain("Validation error found for scope `Thing`."));
        assert!(logs_contain(r#"max_severity="Error""#));
        assert!(logs_contain(r#"scope="Thing""#));
        assert!(logs_contain(r#"during="Rule = data_model.required""#));

        // For the inner issue:
        assert!(logs_contain("Missing required fields: `a_thing`."));
        assert!(logs_contain(r#"severity="Error""#));
    }
}
