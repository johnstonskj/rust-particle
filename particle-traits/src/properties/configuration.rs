//!
//! Provides configuration property traits.
//!

use crate::properties::values::Value;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Traits
// ------------------------------------------------------------------------------------------------

has_traits! {
    Value =>
    "Provides a mixin trait for types that have a `configuration` property."
    HasConfiguration { configuration }
    "Provides a mixin trait for types that have a mutable `configuration` property."
    HasConfigurationMut { set_configuration, configuration_mut }
}

has_traits! {
    #[optional] Value =>
    "Provides a mixin trait for types that have an optional `configuration` property."
    HasOptionalConfiguration { configuration, has_configuration }
    "Provides a mixin trait for types that have a mutable optional `configuration` property."
    HasOptionalConfigurationMut { set_configuration, unset_configuration, configuration_mut }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

builder_fragment! {
    "Provides a builder mix-in fragment for types that can have a `configuration` property." =>
    ConfigurationBuilderFragment ConfigValue: Value
        default with_configuration(configuration) {}
}
