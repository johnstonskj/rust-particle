//!
//! Particle string-like property traits.
//!

use crate::strings::common::LengthLimitedTextProperty;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Tags
// ------------------------------------------------------------------------------------------------

pub trait Tag: LengthLimitedTextProperty {}

pub trait HasTags {
    fn with_tags(self, tags: impl IntoIterator<Item = impl Tag>) -> Self
    where
        Self: Sized;

    fn tags_count(&self) -> usize;
    fn has_tags(&self) -> bool;
    fn tags(&self) -> impl Iterator<Item = &impl Tag>;
}

pub trait HasTagsMut: HasTags {
    fn tags_mut(&mut self) -> impl Iterator<Item = &mut impl Tag>;
    fn add_tag(&mut self, tag: impl Tag);
    fn remove_tag(&mut self, tag: &impl Tag);
    fn clear_tags(&mut self);
}
