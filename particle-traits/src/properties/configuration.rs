//!
//! Provides configuration property traits.
//!

use crate::properties::values::Value;
use core::error::Error as CoreError;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Traits
// ------------------------------------------------------------------------------------------------

/// 
/// Provides a trait for types that have an associated configuration.
/// 
pub trait HasConfiguration {
    /// The configuration value implementation to be included in the built object.
    type ConfigValue: Value;

    fn configuration(&self) -> &Self::ConfigValue;
}

/// 
/// Provides a trait for types that have a mutable associated configuration.
/// 
pub trait HasConfigurationMut: HasConfiguration {
    type Error: CoreError;

    fn configuration_mut(&mut self) -> &mut Self::ConfigValue;

    fn set_configuration(&mut self, configuration: Self::ConfigValue) -> Result<(), Self::Error>;
}

/// 
/// Provides a trait for types that have an optional associated configuration.
/// 
pub trait HasOptionalConfiguration {
    /// The configuration value implementation to be included in the built object.
    type ConfigValue: Value;

    /* const */
    fn has_configuration(&self) -> bool;

    fn configuration(&self) -> Option<&Self::ConfigValue>;
}

/// 
/// Provides a trait for types that have mutable access to an optional associated configuration.
/// 
pub trait HasOptionalConfigurationMut: HasOptionalConfiguration {
    type Error: CoreError;

    fn configuration_mut(&mut self) -> Option<&mut Self::ConfigValue>;

    fn set_configuration(&mut self, configuration: Self::ConfigValue) -> Result<(), Self::Error>;

    fn unset_configuration(&mut self) -> Result<(), Self::Error>;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

///
/// Provides a builder mix-in fragment for types that can have configuration.
/// 
pub trait ConfigurationBuilderFragment {
    /// The configuration value implementation to be included in the built object.
    type ConfigValue: Value;

    fn with_configuration(self, configuration: Self::ConfigValue) -> Self;
}
