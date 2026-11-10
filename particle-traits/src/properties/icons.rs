//!
//! Provides textual icon property traits.
//!
//! [`TextIcon`] types are used as human-readable short descriptors for objects, as such they
//! implement `LengthLimitedTextProperty`.
//!

use core::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Icons
// ------------------------------------------------------------------------------------------------

///
/// Provides a trait for textual icon types.
///
pub trait TextIcon: Clone + Display {}

has_traits! {
    TextIcon =>
    "Provides a mixin trait for types that have an `icon` property."
    HasTextIcon { icon }
    "Provides a mixin trait for types that have a mutable `icon` property."
    HasTextIconMut { set_icon }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

builder_fragment! {
    "Provides a builder mix-in fragment for types that can have textual icons." =>
    TextIconBuilderFragment Icon: TextIcon
        default with_icon(icon) {}
}
