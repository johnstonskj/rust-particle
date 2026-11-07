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

/// 
/// Provides a trait for types that have an associated name.
/// 
pub trait HasName {
    /// The name implementation to be included in the built object.
    type Name: Name;

    /* const */
    fn name(&self) -> &Self::Name;
}

/// 
/// Provides a trait for types that have a mutable associated name.
/// 
pub trait HasNameMut: HasName {
    fn set_name(&mut self, name: Self::Name);
}

/// 
/// Provides a trait for types that have an optional associated name.
/// 
pub trait HasOptionalName {
    /// The name implementation to be included in the built object.
    type Name: Name;

    /* const */
    fn has_name(&self) -> bool;

    /* const */
    fn name(&self) -> Option<&Self::Name>;
}

/// 
/// Provides a trait for types that have a mutable optional associated name.
/// 
pub trait HasOptionalNameMut: HasOptionalName {
    fn set_name(&mut self, name: Self::Name);

    fn unset_name(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

///
/// Provides a builder mix-in fragment for types that can have names.
/// 
pub trait NameBuilderFragment {
    /// The name implementation to be included in the built object.
    type Name: Name;

    fn with_name(self, name: Self::Name) -> Self;

    /// 
    /// Synonym for [`with_name`](NameBuilderFragment::with_name).
    /// 
    fn named(self, name: Self::Name) -> Self
    where
        Self: Sized,
    {
        self.with_name(name)
    }
}
