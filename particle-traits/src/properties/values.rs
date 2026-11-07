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

/// 
/// Provides a trait for types that have an associated value.
/// 
pub trait HasValue {
    /// The value implementation used by the enclosing type.
    type Value: Value;

    fn get(&self) -> &Self::Value;
}

/// 
/// Provides a trait for types that have a mutable associated value.
/// 
pub trait HasValueMut: HasValue {
    fn get_mut(&mut self) -> &mut Self::Value;

    fn set(&mut self, value: Self::Value);
}

/// 
/// Provides a trait for types that have an optional associated value.
/// 
pub trait HasOptionalValue {
    /// The value implementation used by the enclosing type.
    type Value: Value;

    fn get(&self) -> Option<&Self::Value>;
}

/// 
/// Provides a trait for types that have a mutable optional associated value.
/// 
pub trait HasOptionalValueMut: HasOptionalValue {
    fn get_mut(&mut self) -> Option<&mut Self::Value>;

    fn set(&mut self, value: Self::Value);

    fn unset(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

///
/// Provides a builder mix-in fragment for types that can have classifiers.
/// 
pub trait ValueBuilderFragment {
    /// The value implementation to be included in the built object.
    type Value: Value;

    fn with_value(self, value: Self::Value) -> Self;
    
    /// 
    /// Synonym for [`with_value`](ValueBuilderFragment::with_value).
    /// 
    fn with(self, value: Self::Value) -> Self 
    where
        Self: Sized,
    {
        self.with_value(value)
    }
}
