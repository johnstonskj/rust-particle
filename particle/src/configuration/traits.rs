use crate::{error::Error, validation::HasValidation, values::Value};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Traits
// ------------------------------------------------------------------------------------------------

pub trait HasConfiguration: HasValidation {
    type ConfigValue: Value;

    fn configuration(&self) -> &Self::ConfigValue;

    fn configuration_mut(&mut self) -> &mut Self::ConfigValue;

    fn set_configuration(&mut self, configuration: Self::ConfigValue) -> Result<(), Error>;
}

pub trait HasOptionalConfiguration: HasValidation {
    type ConfigValue: Value;

    /* const */
    fn has_configuration(&self) -> bool;

    fn configuration(&self) -> Option<&Self::ConfigValue>;

    fn configuration_mut(&mut self) -> Option<&mut Self::ConfigValue>;

    fn set_configuration(&mut self, configuration: Self::ConfigValue) -> Result<(), Error>;

    fn unset_configuration(&mut self) -> Result<(), Error>;
}
