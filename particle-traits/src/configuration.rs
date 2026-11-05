use crate::values::Value;
use core::error::Error as CoreError;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Traits
// ------------------------------------------------------------------------------------------------

pub trait HasConfiguration {
    type ConfigValue: Value;

    fn configuration(&self) -> &Self::ConfigValue;
}

pub trait HasConfigurationMut: HasConfiguration {
    type Error: CoreError;

    fn configuration_mut(&mut self) -> &mut Self::ConfigValue;

    fn set_configuration(&mut self, configuration: Self::ConfigValue) -> Result<(), Self::Error>;
}

pub trait HasOptionalConfiguration {
    type ConfigValue: Value;

    /* const */
    fn has_configuration(&self) -> bool;

    fn configuration(&self) -> Option<&Self::ConfigValue>;
}

pub trait HasOptionalConfigurationMut: HasOptionalConfiguration {
    type Error: CoreError;

    fn configuration_mut(&mut self) -> Option<&mut Self::ConfigValue>;

    fn set_configuration(&mut self, configuration: Self::ConfigValue) -> Result<(), Self::Error>;

    fn unset_configuration(&mut self) -> Result<(), Self::Error>;
}
