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

has_traits! {
    Tag =>
    "Provides a mixin trait for types that have a `tag` property."
    HasTag { tag }
    "Provides a mixin trait for types that have a mutable `tag` property."
    HasTagMut { set_tag }
}

has_traits! {
    #[iter]
    Tag =>
    "Provides a mixin trait for types that have a `tags` property."
    HasTags { tags, has_tags, tags_count }
    "Provides a mixin trait for types that have a mutable `tags` property."
    HasTagsMut { set_tags, add_tag, extend_tags, remove_tag, clear_tags }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

builder_fragment! {
    "Provides a builder mix-in fragment for types that can have tags." =>
    TagBuilderFragment impl Tag
        default with_tag(tag) {

        fn_with!{
            with_tags(tags : #[iter] Self::Tag)
        }
    }
}
