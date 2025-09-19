//!
//! Particle Value traits.
//!

use std::{fmt::Debug, hash::Hash};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Value Traits
// ------------------------------------------------------------------------------------------------

pub trait Value: Clone + Debug + PartialEq + Eq + Hash + PartialOrd + Ord {}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Value traits
// ------------------------------------------------------------------------------------------------

pub trait HasValue {
    type Value: Value;

    fn with_value(self, value: Self::Value) -> Self;

    fn get(&self) -> &Self::Value;
}

pub trait HasValueMut: HasValue {
    fn get_mut(&mut self) -> &mut Self::Value;

    fn set(&mut self, value: Self::Value);
}

pub trait HasOptionalValue {
    type Value: Value;

    fn with_value(self, value: Self::Value) -> Self;

    fn get(&self) -> Option<&Self::Value>;
}

pub trait HasOptionalValueMut: HasOptionalValue {
    fn get_mut(&mut self) -> Option<&mut Self::Value>;

    fn set(&mut self, value: Self::Value);

    fn unset(&mut self);
}
