//!
//! Provides name property traits.
//!
//! [`Name`] types are used as human-readable longer descriptors for objects, as such they
//! implement `LengthLimitedTextProperty`.
//!

use crate::strings::{AsStr, CheckedFromStr, UncheckedNewStr};
use core::fmt::Display;

#[cfg(not(feature = "std"))]
use alloc::string::String;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Individual Name
// ------------------------------------------------------------------------------------------------

///
/// Provides a trait for name types.
///
pub trait Name:
    AsStr + CheckedFromStr + UncheckedNewStr + Into<String> + Clone + Display + Eq
{
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Name traits
// ------------------------------------------------------------------------------------------------

///
/// Provides a trait for types that have an associated static name.
///
pub trait HasStaticName {
    /* const */
    fn name(&self) -> &'static str;
}

has_traits! {
    Name =>
    "Provides a mixin trait for types that have a `name` property."
    HasName { name }
    "Provides a mixin trait for types that have a mutable `name` property."
    HasNameMut { set_name }
}

has_traits! {
    #[optional] Name =>
    "Provides a mixin trait for types that have an optional `name` property."
    HasOptionalName { name, has_name }
    "Provides a mixin trait for types that have a mutable optional `name` property."
    HasOptionalNameMut { set_name, unset_name }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

builder_fragment! {
    "Provides a builder mix-in fragment for types that can have names." =>
    NameBuilderFragment impl Name
        default with_name(name) {

        fn_with! {
            "Synonym for [`with_name`](NameBuilderFragment::with_name)." =>
            named(name: Self::Name) => with_name
        }
    }
}
