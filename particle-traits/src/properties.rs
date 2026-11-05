//!
//! Provides traits for Particle properties.
//!

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Singleton {
    fn instance() -> &'static Self;
}

pub mod modeled {
    use crate::{names::HasName, values::Value};
    use core::{cmp::PartialEq, fmt::Debug};

    // ------------------------------------------------------------------------------------------------
    // Public Types ❱ Property
    // ------------------------------------------------------------------------------------------------

    pub trait Property: HasName + Clone + Debug + PartialEq {
        type Value: Value;

        fn value(&self) -> &Self::Value;
    }

    pub trait OptionalProperty: HasName + Clone + Debug + PartialEq {
        type Value: Value;

        fn value(&self) -> Option<&Self::Value>;
    }

    pub trait AsProperty {
        type Property: Property;

        fn as_property(&self) -> &Self::Property;
    }

    pub trait AsOptionalProperty {
        type Property: OptionalProperty;

        fn as_property(&self) -> &Self::Property;
    }
}
