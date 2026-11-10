//!
//! Provides value property traits.
//!

use core::{fmt::Debug, hash::Hash};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Value Traits
// ------------------------------------------------------------------------------------------------

///
/// Provides a trait for value types.
///
pub trait Value: Clone + Debug + PartialEq + Eq + Hash + PartialOrd + Ord {}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Value traits
// ------------------------------------------------------------------------------------------------

has_traits! {
    Value =>
    "Provides a mixin trait for types that have a `value` property."
    HasValue { value}
    "Provides a mixin trait for types that have a mutable `value` property."
    HasValueMut { set_value }
}

has_traits! {
    #[optional] Value =>
    "Provides a mixin trait for types that have an optional `value` property."
    HasOptionalValue { value, has_value }
    "Provides a mixin trait for types that have a mutable optional `value` property."
    HasOptionalValueMut { set_value, unset_value, value_mut }
}

has_traits! {
    #[iter] Value =>
    "Provides a mixin trait for types that have a `values` property."
    HasValues { values, has_values, values_count }
    "Provides a mixin trait for types that have a mutable `values` property."
    HasValuesMut { set_values, add_value, extend_values, remove_value, clear_values }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

builder_fragment! {
    "Provides a builder mix-in fragment for types that can have a `value` property." =>
    ValueBuilderFragment impl Value
        default with_value(value) {

        fn_with! {
            "Synonym for [`with_value`](ValueBuilderFragment::with_value)." =>
            with(value: Self::Value) => with_value
        }
    }
}
