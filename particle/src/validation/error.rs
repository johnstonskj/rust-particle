//!
//! Particle Validation errors.
//!

use core::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Validation Errors
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ValidationError {
    class_name: String,
    issues: Vec<ValidationIssue>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ValidationIssue {
    severity: ValidationSeverity,
    scope: Option<String>,
    message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValidationSeverity {
    Information,
    Warning,
    Error,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Validation Error
// ------------------------------------------------------------------------------------------------

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} validation failed with {} issue(s):\n{}",
            self.class_name(),
            self.issue_count(),
            self.issues()
                .map(|issue| issue.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Error for ValidationError {}

impl ValidationError {
    //??    pub fn new<C: Class>(for_class: &C, issues: &[ValidationIssue]) -> Self {
    //??        Self::new_for_name(for_class.name(), issues)
    //??    }
    //??
    //??    pub fn new_for_name<N: Name>(for_class: &N, issues: &[ValidationIssue]) -> Self {
    //??        Self::new_unchecked(for_class.as_str(), issues)
    //??    }

    pub fn new_unchecked<S: Into<String>>(for_class: S, issues: &[ValidationIssue]) -> Self {
        Self {
            class_name: for_class.into(),
            issues: issues.to_vec(),
        }
    }

    pub fn class_name(&self) -> &String {
        &self.class_name
    }

    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }

    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    pub fn issues(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Validation Issue
// ------------------------------------------------------------------------------------------------

impl Display for ValidationIssue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "[{}] {}{}.",
            self.severity(),
            self.message(),
            match self.scope() {
                Some(name) => format!(" @ {name}"),
                None => "".to_string(),
            }
        )
    }
}

impl ValidationIssue {
    pub fn new<M: Display>(
        severity: ValidationSeverity,
        scope: Option<String>,
        message: M,
    ) -> Self {
        Self {
            severity,
            scope,
            message: message.to_string(),
        }
    }

    pub fn new_information<M: Display>(scope: Option<String>, message: M) -> Self {
        Self::new(ValidationSeverity::Information, scope, message)
    }

    pub fn new_warning<M: Display>(scope: Option<String>, message: M) -> Self {
        Self::new(ValidationSeverity::Warning, scope, message)
    }

    pub fn new_error<M: Display>(scope: Option<String>, message: M) -> Self {
        Self::new(ValidationSeverity::Error, scope, message)
    }

    pub fn severity(&self) -> ValidationSeverity {
        self.severity
    }

    pub fn scope(&self) -> Option<&String> {
        self.scope.as_ref()
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Validation Severity
// ------------------------------------------------------------------------------------------------

impl Display for ValidationSeverity {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Information => "INFO",
                Self::Warning => "WARN",
                Self::Error => "ERROR",
            }
        )
    }
}
