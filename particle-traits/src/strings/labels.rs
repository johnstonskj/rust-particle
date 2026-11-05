use crate::strings::properties::LengthLimitedTextProperty;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Display Labels
// ------------------------------------------------------------------------------------------------

pub trait DisplayLabel: LengthLimitedTextProperty {}

pub trait HasDisplayLabel {
    type DisplayLabel: DisplayLabel;

    fn display_label(&self) -> &Self::DisplayLabel;
}

pub trait HasDisplayLabelMut: HasDisplayLabel {
    fn set_display_label(&mut self, display_label: Self::DisplayLabel);
}

pub trait HasOptionalDisplayLabel {
    type DisplayLabel: DisplayLabel;

    fn display_label(&self) -> Option<&Self::DisplayLabel>;
}

pub trait HasOptionalDisplayLabelMut: HasOptionalDisplayLabel {
    fn display_label_mut(&mut self) -> Option<&mut Self::DisplayLabel>;

    fn set_display_label(&mut self, display_label: Self::DisplayLabel);

    fn unset_display_label(&mut self);
}
