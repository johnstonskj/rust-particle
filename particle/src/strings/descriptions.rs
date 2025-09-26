use crate::{names::Name, strings::common::LengthLimitedTextProperty};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Descriptions
// ------------------------------------------------------------------------------------------------

pub trait Description<N: Name>: LengthLimitedTextProperty<N> {}

pub trait HasDescription<N: Name> {
    type Description: Description<N>;

    fn description(&self) -> &Self::Description;
}

pub trait HasDescriptionMut<N: Name>: HasDescription<N> {
    fn set_description(&mut self, description: Self::Description);
}

pub trait HasOptionalDescription<N: Name> {
    type Description: Description<N>;

    fn description(&self) -> Option<&Self::Description>;
}

pub trait HasOptionalDescriptionMut<N: Name>: HasOptionalDescription<N> {
    fn description_mut(&mut self) -> Option<&mut Self::Description>;

    fn set_description(&mut self, description: Self::Description);

    fn unset_description(&mut self);
}
