use crate::strings::common::LengthLimitedTextProperty;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Display Labels
// ------------------------------------------------------------------------------------------------

pub trait DisplayLabel: LengthLimitedTextProperty {}

pub trait HasDisplayLabel {
    fn display_label(&self) -> &impl DisplayLabel;
}

pub trait HasDisplayLabelMut: HasDisplayLabel {
    fn set_display_label(&mut self, display_label: impl DisplayLabel);
}

pub trait HasOptionalDisplayLabel {
    fn display_label(&self) -> Option<&impl DisplayLabel>;
}

pub trait HasOptionalDisplayLabelMut: HasOptionalDisplayLabel {
    fn display_label_mut(&mut self) -> Option<&mut impl DisplayLabel>;

    fn set_display_label(&mut self, display_label: impl DisplayLabel);

    fn unset_display_label(&mut self);
}

pub trait CanProvideDisplayLabel {
    fn display_label(&self) -> impl DisplayLabel;
}
