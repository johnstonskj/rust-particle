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

/// 
/// Provides a trait for types that have an associated textual icon.
/// 
pub trait HasTextIcon {
    /// The icon implementation used by the enclosing type.
    type Icon: TextIcon;

    /* const */
    fn icon(&self) -> &Self::Icon;
}

/// 
/// Provides a trait for types that have a mutable associated textual icon.
/// 
pub trait HasTextIconMut: HasTextIcon {
    fn set_icon(&mut self, icon: Self::Icon);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

///
/// Provides a builder mix-in fragment for types that can have textual icons.
/// 
pub trait TextIconBuilderFragment {
    /// The icon implementation to be included in the built object.
    type Icon: TextIcon;

    fn with_icon(self, icon: Self::Icon) -> Self;
}
    