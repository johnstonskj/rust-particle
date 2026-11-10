//!
//! Provides display label property traits.
//!
//! [`DisplayLabel`] types are used as human-readable short descriptors for objects, as such they
//! implement `LengthLimitedTextProperty`.
//!

use crate::strings::LengthLimitedTextProperty;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Display Labels
// ------------------------------------------------------------------------------------------------

///
/// Provides a trait for display label types.
///
pub trait DisplayLabel: LengthLimitedTextProperty {}

// ------------------------------------------------------------------------------------------------

has_traits! {
    DisplayLabel =>
    "Provides a mixin trait for types that have a `display_label` property."
    HasDisplayLabel { display_label }
    "Provides a mixin trait for types that have a mutable `display_label` property."
    HasDisplayLabelMut { set_display_label, display_label_mut }
}

has_traits! {
    #[optional] DisplayLabel =>
    "Provides a mixin trait for types that have an optional `display_label` property."
    HasOptionalDisplayLabel { display_label, has_display_label }
    "Provides a mixin trait for types that have a mutable optional `display_label` property."
    HasOptionalDisplayLabelMut { set_display_label, display_label_mut }
}

has_traits! {
    #[iter] DisplayLabel =>
    "Provides a mixin trait for types that have a `display_labels` property."
    HasDisplayLabels { display_labels, has_display_labels, display_label_count }
    "Provides a mixin trait for types that have a mutable `display_labels` property."
    HasDisplayLabelsMut { set_display_labels, add_display_label, extend_display_labels,
        remove_display_label, clear_display_labels, display_labels_mut }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

builder_fragment! {
    "Provides a builder mix-in fragment for types that can have display labels." =>
    DisplayLabelBuilderFragment impl DisplayLabel {

        fn_with! {
            with_display_label(display_label: Self::DisplayLabel)
        }

        fn_with! {
            with_display_labels(display_labels: #[iter] Self::DisplayLabel)
        }
    }
}
