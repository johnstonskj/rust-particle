//!
//! Provides description property traits.
//!
//! [`Description`] types are used as human-readable longer descriptors for objects, as such they
//! implement `LengthLimitedTextProperty`.
//!

use crate::strings::LengthLimitedTextProperty;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Descriptions
// ------------------------------------------------------------------------------------------------

///
/// Provides a trait for description types, these are human-readable longer descriptors for objects.
///
pub trait Description: LengthLimitedTextProperty {}

has_traits! {
    Description =>
    "Provides a mixin trait for types that have a `description` property."
    HasDescription {  description }
    "Provides a mixin trait for types that have a mutable `description` property."
    HasDescriptionMut { set_description }
}

has_traits! {
    #[optional] Description =>
    "Provides a mixin trait for types that have an optional `description` property."
    HasOptionalDescription { description, has_description }
    "Provides a mixin trait for types that have a mutable optional `description` property."
    HasOptionalDescriptionMut { set_description, unset_description, description_mut }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

builder_fragment! {
    "Provides a builder mix-in fragment for types that can have a `description` property." =>
    DescriptionBuilderFragment impl Description
        default with_description(description) {

        fn_with! {
            "Synonym for [`with_description`](DescriptionBuilderFragment::with_description)." =>
            described_by(description: Self::Description) => with_description
        }
    }
}
