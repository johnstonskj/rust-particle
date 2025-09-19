use crate::strings::common::LengthLimitedTextProperty;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Descriptions
// ------------------------------------------------------------------------------------------------

pub trait Description: LengthLimitedTextProperty {}

pub trait HasDescription {
    fn description(&self) -> &impl Description;
}

pub trait HasDescriptionMut: HasDescription {
    fn set_description(&mut self, description: impl Description);
}

pub trait HasOptionalDescription {
    fn description(&self) -> Option<&impl Description>;
}

pub trait HasOptionalDescriptionMut: HasOptionalDescription {
    fn description_mut(&mut self) -> Option<&mut impl Description>;

    fn set_description(&mut self, description: impl Description);

    fn unset_description(&mut self);
}
