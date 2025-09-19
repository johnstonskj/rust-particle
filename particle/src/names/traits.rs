//!
//! Particle name traits.
//!

use crate::{
    builders::BuilderRoot,
    classes::HasClassification,
    strings::{AsStr, CheckedFromStr, UncheckedFromStr},
};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Individual Name
// ------------------------------------------------------------------------------------------------

pub trait Name:
    HasClassification
    + CheckedFromStr
    + UncheckedFromStr
    + AsStr
    + Clone
    + Debug
    + Display
    + PartialEq
    + Eq
    + Hash
    + PartialOrd
    + Ord
{
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Name traits
// ------------------------------------------------------------------------------------------------

pub trait HasName {
    type Name: Name;

    /* const */
    fn name(&self) -> &Self::Name;
}

pub trait HasNameMut: HasName {
    fn set_name(&mut self, name: Self::Name);
}

pub trait HasOptionalName {
    type Name: Name;

    /* const */
    fn has_name(&self) -> bool;

    /* const */
    fn name(&self) -> Option<&Self::Name>;
}

pub trait HasOptionalNameMut: HasOptionalName {
    fn set_name(&mut self, name: Self::Name);

    fn unset_name(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

pub trait BuilderNameFragment: BuilderRoot {
    type Name: Name;

    fn named(self, name: Self::Name) -> Self;
}
