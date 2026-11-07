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
/// 
/// Provides a trait for types that have an associated display label.
/// 
pub trait HasDisplayLabel {
    /// The display label implementation to be included in the built object.
    type DisplayLabel: DisplayLabel;

    /* const */
    fn display_label(&self) -> &Self::DisplayLabel;
}

/// 
/// Provides a trait for types that have a mutable associated display label.
/// 
pub trait HasDisplayLabelMut: HasDisplayLabel {
    fn set_display_label(&mut self, display_label: Self::DisplayLabel);
}

/// 
/// Provides a trait for types that have an optional associated display label.
/// 
pub trait HasOptionalDisplayLabel {
    /// The display label implementation to be included in the built object.
    type DisplayLabel: DisplayLabel;

    /* const */
    fn display_label(&self) -> Option<&Self::DisplayLabel>;
}

/// 
/// Provides a trait for types that have a mutable optional associated display label.
/// 
pub trait HasOptionalDisplayLabelMut: HasOptionalDisplayLabel {
    fn set_display_label(&mut self, display_label: Self::DisplayLabel);

    fn unset_display_label(&mut self);
}

/// 
/// Provides a trait for types that have associated display labels.
/// 
pub trait HasDisplayLabels {
    /// The display label implementation to be included in the built object.
    type DisplayLabel: DisplayLabel;

    /* const */
    fn display_labels(&self) -> impl Iterator<Item = &Self::DisplayLabel>;
}

/// 
/// Provides a trait for types that have mutable associated display labels.
/// 
pub trait HasDisplayLabelsMut: HasDisplayLabels {
    fn display_labels_mut(&mut self) -> impl Iterator<Item = &mut Self::DisplayLabel>;

    fn set_display_labels<I: IntoIterator<Item = Self::DisplayLabel>>(&mut self, display_labels: I);

    fn add_display_label(&mut self, display_label: Self::DisplayLabel);

    fn extend_display_labels<I: IntoIterator<Item = Self::DisplayLabel>>(
        &mut self,
        display_labels: I,
    );

    fn remove_display_label(
        &mut self,
        display_label: &Self::DisplayLabel,
    ) -> Option<Self::DisplayLabel>;

    fn clear_display_labels(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

///
/// Provides a builder mix-in fragment for types that can have display labels.
/// 
pub trait DisplayLabelBuilderFragment {
    /// The display label implementation to be included in the built object.
    type DisplayLabel: DisplayLabel;

    fn with_display_label(self, display_label: Self::DisplayLabel) -> Self;

    fn with_display_labels(self, display_labels: impl IntoIterator<Item = Self::DisplayLabel>) -> Self;
}
