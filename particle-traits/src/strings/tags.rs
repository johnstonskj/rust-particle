//!
//! Particle string-like property traits.
//!

use crate::strings::properties::LengthLimitedTextProperty;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Tags
// ------------------------------------------------------------------------------------------------

pub trait Tag: LengthLimitedTextProperty {}

pub trait HasTags {
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

pub trait HasTagsMut: HasTags {
    fn add_tag(&mut self, tag: Self::Tag);
    fn remove_tag(&mut self, tag: &Self::Tag);
    fn clear_tags(&mut self);
    fn tags_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Self::Tag>
    where
        <Self as HasTags>::Tag: 'a;
}
