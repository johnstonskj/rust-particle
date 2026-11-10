//!
//! Provides validation property, and validator object, traits.
//!

use crate::properties::{descriptions::HasDescription, names::HasName};
use core::error::Error as CoreError;

#[cfg(not(feature = "std"))]
use alloc::{rc::Rc, vec::Vec};

#[cfg(feature = "std")]
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Validation
// ------------------------------------------------------------------------------------------------

///
/// A Validator is responsible for validating *subjects* against a set of *validation rules*.
///
/// Validators are typically associated with a specific type of subject and a set of rules that
/// define valid states for that subject.
///
pub trait Validator: HasName {
    /// The rule group implementation used by the enclosing type.
    type Group: RuleGroup<Owner = Self, Rule = Self::Rule>;
    /// The rule implementation used by the enclosing type.
    type Rule: Rule<Group = Self::Group>;
    type Subject;
    /// The error type returned when validation fails.
    type Error: CoreError;

    fn validate(&self, subject: &Self::Subject) -> Result<(), Self::Error>;

    fn validate_with_filter(
        &self,
        subject: &Self::Subject,
        filters: &[RuleFilter<Self::Group, Self::Rule>],
    ) -> Result<(), Self::Error>;

    fn groups(&self) -> impl Iterator<Item = &Rc<Self::Rule>>;
    fn all_rules(&self) -> impl Iterator<Item = &Rc<Self::Rule>>;
}

///
/// A RuleGroup represents a collection of validation rules that belong to a specific validator.
///
pub trait RuleGroup: HasName + HasDescription {
    /// The validator implementation used as the owner of this rule group.
    type Owner: Validator<Group = Self>;
    /// The rule implementation used as members of this rule group.
    type Rule: Rule;

    fn rules(&self) -> impl Iterator<Item = &Rc<Self::Rule>>;
}

///
/// A Rule represents a single validation rule within a rule group.
///
pub trait Rule: HasName + HasDescription {
    /// The group implementation in which this rule belongs.
    type Group: RuleGroup;

    fn in_group(&self) -> Option<Rc<Self::Group>>;
}

///
/// Represents a filter over a set of validation rules, and the action to be taken on those rules.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuleFilter<G: RuleGroup<Rule = R>, R: Rule<Group = G>> {
    target: TargetRules<G, R>,
    action: Action,
}

///
/// Determines the outcome of a [`TargetRules`] evaluation.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Allow,
    Warn,
    Deny,
}

///
/// Specifies the target rules for a rule filter action.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TargetRules<G: RuleGroup<Rule = R>, R: Rule<Group = G>> {
    All,
    Group(Vec<Rc<G>>),
    Rule(Vec<Rc<R>>),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Validation
// ------------------------------------------------------------------------------------------------

///
/// Provides a trait for types that have validation capabilities.
///
pub trait Validatable {
    /// The error type returned when validation fails.
    type Error: CoreError;

    // `false` option (in simple).
    fn is_valid(&self) -> bool;
    fn validate(&self) -> Result<(), Self::Error>;
}

///
/// Provides a trait for types that have an associated validator.
///
pub trait HasValidator {
    /// The group implementation used by the enclosing type.
    type Group: RuleGroup<Owner = Self, Rule = Self::Rule>;
    /// The rule implementation used by the enclosing type.
    type Rule: Rule<Group = Self::Group>;
    /// The error type returned when validation fails.
    type Error: CoreError;
    /// The validator implementation used by the enclosing type.
    type Validator: Validator<Group = Self::Group, Rule = Self::Rule, Subject = Self, Error = Self::Error>;

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
    /// The group implementation used by the enclosing type.
    type Group: RuleGroup<Owner = Self, Rule = Self::Rule>;
    /// The rule implementation used by the enclosing type.
    type Rule: Rule<Group = Self::Group>;
    /// The error type returned when validation fails.
    type Error: CoreError;
    /// The validator implementation used by the enclosing type.
    type Validator: Validator<Group = Self::Group, Rule = Self::Rule, Subject = Self, Error = Self::Error>;

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
