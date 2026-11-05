//!
//! Particle name traits.
//!

use crate::strings::{AsStr, CheckedFromStr, UncheckedNewStr};
use core::fmt::Display;

#[cfg(not(feature = "std"))]
use alloc::string::String;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Individual Name
// ------------------------------------------------------------------------------------------------

pub trait Name:
    AsStr + CheckedFromStr + UncheckedNewStr + Into<String> + Clone + Display + Eq
{
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Name traits
// ------------------------------------------------------------------------------------------------

pub trait HasStaticName {
    /* const */
    fn name(&self) -> &'static str;
}

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

pub trait NameBuilderFragment {
    type Name: Name;

    fn with_name(self, name: Self::Name) -> Self;

    fn named(self, name: Self::Name) -> Self
    where
        Self: Sized,
    {
        self.with_name(name)
    }
}
