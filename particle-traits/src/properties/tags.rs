//!
//! Particle string-like property traits.
//!
//! [`Tag`] types are used as human-readable longer descriptors for objects, as such they 
//! implement `LengthLimitedTextProperty`.
//!

use crate::strings::LengthLimitedTextProperty;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Tags
// ------------------------------------------------------------------------------------------------

/// 
/// Provides a trait for tag types.
/// 
pub trait Tag: LengthLimitedTextProperty {}

/// 
/// Provides a trait for types that have tags.
/// 
pub trait HasTags {
    /// The tag implementation used by the enclosing type.
    type Tag: Tag;

    fn with_tags(self, tags: impl IntoIterator<Item = Self::Tag>) -> Self
    where
        Self: Sized;

    fn tags_count(&self) -> usize;
    fn has_tags(&self) -> bool;
    fn tags<'a>(&'a self) -> impl Iterator<Item = &'a Self::Tag>
    where
        <Self as HasTags>::Tag: 'a;
}

/// 
/// Provides a trait for types that have mutable tags.
/// 
pub trait HasTagsMut: HasTags {
    fn add_tag(&mut self, tag: Self::Tag);
    fn remove_tag(&mut self, tag: &Self::Tag);
    fn clear_tags(&mut self);
    fn tags_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Self::Tag>
    where
        <Self as HasTags>::Tag: 'a;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

///
/// Provides a builder mix-in fragment for types that can have tags.
/// 
pub trait TagBuilderFragment {
    /// The tag implementation to be included in the built object.
    type Tag: Tag;

    fn with_tag(self, tag: Self::Tag) -> Self;

    fn with_tags(self, tags: impl IntoIterator<Item = Self::Tag>) -> Self;
}
