//!
//! Particle Validation traits.
//!

use crate::{names::HasName, strings::descriptions::HasDescription};
use core::error::Error as CoreError;

#[cfg(not(feature = "std"))]
use alloc::{rc::Rc, vec::Vec};

#[cfg(feature = "std")]
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Validation
// ------------------------------------------------------------------------------------------------

pub trait Validator: HasName {
    type Group: RuleGroup<Owner = Self, Rule = Self::Rule>;
    type Rule: Rule<Group = Self::Group>;
    type Error: CoreError;

    fn validate(&self, subject: &impl HasValidation) -> Result<(), Self::Error>;

    fn validate_with_filter(
        &self,
        subject: &impl HasValidation,
        filters: &[RuleFilter<Self::Group, Self::Rule>],
    ) -> Result<(), Self::Error>;

    fn groups(&self) -> impl Iterator<Item = &Rc<Self::Rule>>;
    fn all_rules(&self) -> impl Iterator<Item = &Rc<Self::Rule>>;
}

pub trait RuleGroup: HasName + HasDescription {
    type Owner: Validator<Group = Self>;
    type Rule: Rule;

    fn rules(&self) -> impl Iterator<Item = &Rc<Self::Rule>>;
}

pub trait Rule: HasName + HasDescription {
    type Group: RuleGroup;

    fn in_group(&self) -> Option<Rc<Self::Group>>;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuleFilter<G: RuleGroup<Rule = R>, R: Rule<Group = G>> {
    action: Action,
    target: TargetRules<G, R>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Allow,
    Warn,
    Deny,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TargetRules<G: RuleGroup<Rule = R>, R: Rule<Group = G>> {
    All,
    Group(Vec<Rc<G>>),
    Rule(Vec<Rc<R>>),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Validation
// ------------------------------------------------------------------------------------------------

pub trait HasValidation {
    type Error: CoreError;

    // TODO: auto implemented for any type with HasFastValidation to call the
    // `false` option (in simple).
    fn is_valid(&self) -> bool;
    fn validate(&self) -> Result<(), Self::Error>;
}

pub trait HasFastValidation {
    type Error: CoreError;

    fn is_valid(&self, fast: bool) -> bool;
    fn validate(&self, fast: bool) -> Result<(), Self::Error>;
}

pub trait HasValidator {
    type Group: RuleGroup<Owner = Self, Rule = Self::Rule>;
    type Rule: Rule<Group = Self::Group>;
    type Error: CoreError;
    type Validator: Validator<Group = Self::Group, Rule = Self::Rule, Error = Self::Error>;

    fn validator(&self) -> &Self::Validator;
}

pub trait HasValidatorMut: HasValidator {
    fn validator_mut(&mut self) -> &mut Self::Validator;
    fn set_validator(&mut self, validator: Self::Validator);
}
