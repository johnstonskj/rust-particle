//!
//! Provides validation property, and validator object, traits.
//!

prelude!(rust_2024);

use crate::properties::{
    classifiers::HasClassifier, errors::HasSeverity, identifiers::HasIdentifier,
};
use core::{error::Error as CoreError, fmt::Display};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Validation
// ------------------------------------------------------------------------------------------------

///
/// A Validator is responsible for validating *subjects* and returning validation results. Results
/// take the form of a sequence of issues, each representing a specific validation problem.
///
/// Validators are typically associated with a specific type of subject.
///
pub trait Validator {
    /// The type of subject that this validator can validate.
    type Subject;
    /// The error type returned when validation fails.
    type Error: CoreError;
    /// The type of issue returned when a validation problem is encountered.
    type Issue: Issue;

    /// Called to validate the given subject and return any validation issues encountered.
    fn validate(
        &self,
        subject: &Self::Subject,
    ) -> Result<impl Iterator<Item = Self::Issue>, Self::Error>;

    /// Returns `true` if the given subject is valid according to the same rules as the method
    /// [`validate`], else `false`.
    fn is_valid(&self, subject: &Self::Subject) -> bool;
}

///
/// Provides a trait for types that rather than being validated externally, can validate themselves.
///
pub trait Validatable {
    /// The error type returned when validation fails.
    type Error: CoreError;
    type Issue: Issue;

    fn validate(&self) -> Result<impl Iterator<Item = Self::Issue>, Self::Error>;

    /// Returns `true` if the this subject is valid according to the same rules as the method
    /// [`validate`], else `false`.
    fn is_valid(&self) -> bool;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Issues
// ------------------------------------------------------------------------------------------------

///
/// An issue reports a specific problem encountered during the validation process.
///
/// Issues, like errors, have identifiers so that they may be easily identified and referenced. They
/// also have severities to indicate the importance, or impact, of the issue.
///
pub trait Issue: Display + HasIdentifier + HasSeverity {}

///
/// This extension to [`Issue`] adds the ability to return the rules referenced by the issue.
///
pub trait IssueWithRuleReferences: Issue {
    type Rule: Rule;

    fn referenced_rules(&self) -> impl Iterator<Item = &Self::Rule>;
}

///
/// A rule represents some stipulation or condition that must be satisfied for validation to
/// succeed. An issue is typically reported when when one or more rules are violated.
///
pub trait Rule: Display + HasClassifier {}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Validation
// ------------------------------------------------------------------------------------------------

///
/// Provides a trait for types that have an associated validator.
///
pub trait HasValidator {
    type Subject;
    /// The error type returned when validation fails.
    type Error: CoreError;
    /// The validator implementation used by the enclosing type.
    type Validator: Validator<Subject = Self, Error = Self::Error>;

    fn validator(&self) -> &Self::Validator;
}

///
/// Provides a trait for types that have a mutable associated validator.
///
pub trait HasValidatorMut: HasValidator {
    fn set_validator(&mut self, validator: Self::Validator);
}

///
/// Provides a trait for types that have an optional associated validator.
///
pub trait HasOptionalValidator {
    type Subject;
    /// The error type returned when validation fails.
    type Error: CoreError;
    /// The validator implementation used by the enclosing type.
    type Validator: Validator<Subject = Self, Error = Self::Error>;

    fn validator(&self) -> Option<&Self::Validator>;
}

///
/// Provides a trait for types that have a mutable optional associated validator.
///
pub trait HasOptionalValidatorMut: HasOptionalValidator {
    fn set_validator(&mut self, validator: Self::Validator);
    fn unset_validator(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

builder_fragment! {
    "Provides a builder mix-in fragment for types that can have validators." =>
    ValidatorBuilderFragment impl Validator
        default with_validator(validator) {}
}
