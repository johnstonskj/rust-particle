//!
//! Particle Validation traits.
//!

use crate::{
    names::{HasName, Name},
    strings::HasDescription,
    validation::ValidationError,
};
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

    fn validate(
        &self,
        subject: &impl HasValidation,
    ) -> Result<(), ValidationError<<Self as HasName>::Name>>;

    fn validate_with_filter(
        &self,
        subject: &impl HasValidation,
        filters: &[RuleFilter<Self::Group, Self::Rule>],
    ) -> Result<(), ValidationError<<Self as HasName>::Name>>;

    fn groups(&self) -> impl Iterator<Item = &Rc<Self::Rule>>;
    fn all_rules(&self) -> impl Iterator<Item = &Rc<Self::Rule>>;
}

pub trait RuleGroup: HasName + HasDescription<<Self as HasName>::Name> {
    type Owner: Validator<Group = Self>;
    type Rule: Rule;

    fn rules(&self) -> impl Iterator<Item = &Rc<Self::Rule>>;
}

pub trait Rule: HasName + HasDescription<<Self as HasName>::Name> {
    type Group: RuleGroup;

    fn in_group(&self) -> Option<Rc<Self::Group>>;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RuleFilter<G: RuleGroup<Rule = R>, R: Rule<Group = G>> {
    action: Action,
    target: TargetRules<G, R>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Action {
    Allow,
    Warn,
    Deny,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TargetRules<G: RuleGroup<Rule = R>, R: Rule<Group = G>> {
    All,
    Group(Vec<Rc<G>>),
    Rule(Vec<Rc<R>>),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Validation
// ------------------------------------------------------------------------------------------------

pub trait HasValidation {
    // TODO: auto implemented for any type with HasFastValidation to call the
    // `false` option (in simple).
    fn is_valid(&self) -> bool;
    fn validate<N: Name>(&self) -> Result<(), ValidationError<N>>;
}

pub trait HasFastValidation {
    fn is_valid(&self, fast: bool) -> bool;
    fn validate<N: Name>(&self, fast: bool) -> Result<(), ValidationError<N>>;
}
