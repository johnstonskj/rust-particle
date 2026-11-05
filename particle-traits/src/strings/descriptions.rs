use crate::strings::properties::LengthLimitedTextProperty;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Descriptions
// ------------------------------------------------------------------------------------------------

pub trait Description: LengthLimitedTextProperty {}

pub trait HasDescription {
    type Description: Description;

    fn description(&self) -> &Self::Description;
}

pub trait HasDescriptionMut: HasDescription {
    fn set_description(&mut self, description: Self::Description);
}

pub trait HasOptionalDescription {
    type Description: Description;

    fn description(&self) -> Option<&Self::Description>;
}

pub trait HasOptionalDescriptionMut: HasOptionalDescription {
    fn description_mut(&mut self) -> Option<&mut Self::Description>;

    fn set_description(&mut self, description: Self::Description);

    fn unset_description(&mut self);
}
