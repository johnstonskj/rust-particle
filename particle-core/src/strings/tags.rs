//!
//! Particle string-like property traits.
//!

use crate::{names::Name, strings::common::LengthLimitedTextProperty};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Tags
// ------------------------------------------------------------------------------------------------

pub trait Tag<N: Name>: LengthLimitedTextProperty<N> {}

pub trait HasTags<N: Name> {
    type Tag: Tag<N>;

    fn with_tags(self, tags: impl IntoIterator<Item = Self::Tag>) -> Self
    where
        Self: Sized;

    fn tags_count(&self) -> usize;
    fn has_tags(&self) -> bool;
    fn tags<'a>(&'a self) -> impl Iterator<Item = &'a Self::Tag>
    where
        <Self as HasTags<N>>::Tag: 'a;
}

pub trait HasTagsMut<N: Name>: HasTags<N> {
    fn add_tag(&mut self, tag: Self::Tag);
    fn remove_tag(&mut self, tag: &Self::Tag);
    fn clear_tags(&mut self);
    fn tags_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Self::Tag>
    where
        <Self as HasTags<N>>::Tag: 'a;
}
