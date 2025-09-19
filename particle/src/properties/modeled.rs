use crate::{properties::name::HasName, values::Value};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::Hash,
    str::FromStr,
};

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
