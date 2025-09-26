use crate::{names::Name, strings::common::LengthLimitedTextProperty};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Display Labels
// ------------------------------------------------------------------------------------------------

pub trait DisplayLabel<N: Name>: LengthLimitedTextProperty<N> {}

pub trait HasDisplayLabel<N: Name> {
    type DisplayLabel: DisplayLabel<N>;

    fn display_label(&self) -> &Self::DisplayLabel;
}

pub trait HasDisplayLabelMut<N: Name>: HasDisplayLabel<N> {
    fn set_display_label(&mut self, display_label: Self::DisplayLabel);
}

pub trait HasOptionalDisplayLabel<N: Name> {
    type DisplayLabel: DisplayLabel<N>;

    fn display_label(&self) -> Option<&Self::DisplayLabel>;
}

pub trait HasOptionalDisplayLabelMut<N: Name>: HasOptionalDisplayLabel<N> {
    fn display_label_mut(&mut self) -> Option<&mut Self::DisplayLabel>;

    fn set_display_label(&mut self, display_label: Self::DisplayLabel);

    fn unset_display_label(&mut self);
}
