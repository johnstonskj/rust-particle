//!
//! Particle Builder errors.
//!

use crate::names::Name;
#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Errors
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ObjectMissingRequiredProperties<N: Name> {
    property_names: Vec<N>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ObjectMissingDependentProperties<N: Name> {
    property_names: Vec<N>,
    required_if: N,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ObjectHasIncompatibleProperties<N: Name> {
    property_names: Vec<N>,
    incompatible_with: N,
    reason: PropertyIncompatibilityReason,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PropertyIncompatibilityReason {
    #[default]
    NotApplicable,
    Nonsensical,
    Other(String),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ValueInvalidRepresentation<N: Name> {
    property_name: Option<N>,
    value_type: N,
    value_as_string: String,
    reason: ValueInvalidRepresentationReason,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ValueInvalidRepresentationReason {
    Empty,
    TooShort,
    TooLong,
    InvalidCharacter,
    InvalidEncoding,
    Other(String),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ValueMissing<N: Name> {
    property_name: Option<N>,
    value_type: N,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ValueMissingFrom<N: Name> {
    property_name: Option<N>,
    value_type: N,
    container_name: Option<N>,
    container_type: N,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ValueOutOfBounds<N: Name> {
    property_name: Option<N>,
    value_type: N,
    exceeded: ValueBound,
    value_as_string: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ValueBound {
    Min,
    Max,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ValueOfDependentPropertyInvalid<N: Name> {
    property_name: N,
    depends_on: N,
    value_as_string: String,
    reason: String,
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! impl_into_validation_issue {
    ($impl_type:ident => $variant_name:ident) => {
        impl<N: $crate::names::Name> From<$impl_type<N>>
            for $crate::validation::error::ValidationIssue<N>
        {
            fn from(value: $impl_type<N>) -> Self {
                $crate::validation::error::ValidationIssue::$variant_name(value)
            }
        }
    };
}

macro_rules! impl_has_severity {
    ($impl_type:ident => $severity:ident) => {
        impl<N: $crate::names::Name> $crate::error::HasErrorSeverity for $impl_type<N> {
            fn severity(&self) -> $crate::error::ErrorSeverity {
                $crate::error::ErrorSeverity::$severity
            }
        }
    };
}

macro_rules! impl_display {
    ($impl_type:ident, $formatter:expr) => {
        impl<N: $crate::names::Name> ::core::fmt::Display for $impl_type<N> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", $formatter(self))
            }
        }
    };
}

macro_rules! impl_error {
    ($impl_type:ident) => {
        impl<N: $crate::names::Name> ::core::error::Error for $impl_type<N> {}
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ MissingRequiredProperties
// ------------------------------------------------------------------------------------------------

impl_into_validation_issue!(ObjectMissingRequiredProperties => ObjectMissingRequiredProperties);
impl_has_severity!(ObjectMissingRequiredProperties => Error);
impl_display!(
    ObjectMissingRequiredProperties,
    |issue: &ObjectMissingRequiredProperties<N>| format!(
        "Missing required fields: {}.",
        issue
            .property_names
            .iter()
            .map(|n| format!("`{n}`"))
            .collect::<Vec<_>>()
            .join(", ")
    )
);
impl_error!(ObjectMissingRequiredProperties);

impl<N: Name> ObjectMissingRequiredProperties<N> {
    pub fn new(property_names: &[N]) -> Self {
        Self {
            property_names: property_names.to_vec(),
        }
    }

    pub fn property_names(&self) -> impl Iterator<Item = &N> {
        self.property_names.iter()
    }

    pub fn property_name_count(&self) -> usize {
        self.property_names.len()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ MissingDependentProperties
// ------------------------------------------------------------------------------------------------

impl_into_validation_issue!(ObjectMissingDependentProperties => ObjectMissingDependentProperties);
impl_has_severity!(ObjectMissingDependentProperties => Error);
impl_display!(
    ObjectMissingDependentProperties,
    |issue: &ObjectMissingDependentProperties<N>| format!(
        "Missing dependent fields: {}; required if field `{}` is present.",
        issue
            .property_names
            .iter()
            .map(|n| format!("`{n}`"))
            .collect::<Vec<_>>()
            .join(", "),
        issue.required_if
    )
);
impl_error!(ObjectMissingDependentProperties);

impl<N: Name> ObjectMissingDependentProperties<N> {
    pub fn new(property_names: &[N], required_if: N) -> Self {
        Self {
            property_names: property_names.to_vec(),
            required_if,
        }
    }

    pub fn property_names(&self) -> impl Iterator<Item = &N> {
        self.property_names.iter()
    }

    pub fn property_name_count(&self) -> usize {
        self.property_names.len()
    }

    pub fn required_if(&self) -> &N {
        &self.required_if
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ IncompatibleProperties
// ------------------------------------------------------------------------------------------------

impl_into_validation_issue!(ObjectHasIncompatibleProperties => ObjectHasIncompatibleProperties);
impl_has_severity!(ObjectHasIncompatibleProperties => Error);
impl_display!(
    ObjectHasIncompatibleProperties,
    |issue: &ObjectHasIncompatibleProperties<N>| format!(
        "Fields: {} are incompatible with the field `{}` because {}.",
        issue
            .property_names
            .iter()
            .map(|n| format!("`{n}`"))
            .collect::<Vec<_>>()
            .join(", "),
        issue.incompatible_with,
        issue.reason
    )
);
impl_error!(ObjectHasIncompatibleProperties);

impl<N: Name> ObjectHasIncompatibleProperties<N> {
    pub fn new(
        property_names: &[N],
        incompatible_with: N,
        reason: PropertyIncompatibilityReason,
    ) -> Self {
        Self {
            property_names: property_names.to_vec(),
            incompatible_with,
            reason,
        }
    }

    pub fn property_names(&self) -> impl Iterator<Item = &N> {
        self.property_names.iter()
    }

    pub fn property_name_count(&self) -> usize {
        self.property_names.len()
    }

    pub fn incompatible_with(&self) -> &N {
        &self.incompatible_with
    }

    pub fn reason(&self) -> &PropertyIncompatibilityReason {
        &self.reason
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ IncompatibilityReason
// ------------------------------------------------------------------------------------------------

impl Display for PropertyIncompatibilityReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                PropertyIncompatibilityReason::NotApplicable => "Not applicable".to_string(),
                PropertyIncompatibilityReason::Nonsensical => "Nonsensical".to_string(),
                PropertyIncompatibilityReason::Other(reason) => reason.to_string(),
            },
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ InvalidValueRepresentation
// ------------------------------------------------------------------------------------------------

impl_into_validation_issue!(ValueInvalidRepresentation => ValueInvalidRepresentation);
impl_has_severity!(ValueInvalidRepresentation => Error);
impl_display!(
    ValueInvalidRepresentation,
    |issue: &ValueInvalidRepresentation<N>| format!(
        "Invalid string representation{} of type `{}`; given: '{}'",
        issue
            .property_name
            .as_ref()
            .map(|n| format!(", for property named `{n}`,"))
            .unwrap_or_default(),
        issue.value_type,
        issue.value_as_string
    )
);
impl_error!(ValueInvalidRepresentation);

impl<N: Name> ValueInvalidRepresentation<N> {
    pub fn new_property<S: Into<String>>(
        property_name: N,
        value_type: N,
        value_as_string: S,
        reason: ValueInvalidRepresentationReason,
    ) -> Self {
        Self {
            property_name: Some(property_name),
            value_type,
            value_as_string: value_as_string.into(),
            reason,
        }
    }
    pub fn new_type<S: Into<String>>(
        value_type: N,
        value_as_string: S,
        reason: ValueInvalidRepresentationReason,
    ) -> Self {
        Self {
            property_name: None,
            value_type,
            value_as_string: value_as_string.into(),
            reason,
        }
    }

    pub fn property_name(&self) -> Option<&N> {
        self.property_name.as_ref()
    }

    pub fn value_type(&self) -> &N {
        &self.value_type
    }

    pub fn value_as_string(&self) -> &String {
        &self.value_as_string
    }

    pub fn reason(&self) -> &ValueInvalidRepresentationReason {
        &self.reason
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ ValueMissing
// ------------------------------------------------------------------------------------------------

impl_into_validation_issue!(ValueMissing => ValueMissing);
impl_has_severity!(ValueMissing => Error);
impl_display!(ValueMissing, |issue: &ValueMissing<N>| format!(
    "Value{} of type `{}` is missing.",
    issue
        .property_name
        .as_ref()
        .map(|n| format!(", for property named `{n}`,"))
        .unwrap_or_default(),
    issue.value_type
));
impl_error!(ValueMissing);

impl<N: Name> ValueMissing<N> {
    pub fn new_property(property_name: N, value_type: N) -> Self {
        Self {
            property_name: Some(property_name),
            value_type,
        }
    }
    pub fn new_type(value_type: N) -> Self {
        Self {
            property_name: None,
            value_type,
        }
    }

    pub fn property_name(&self) -> Option<&N> {
        self.property_name.as_ref()
    }

    pub fn value_type(&self) -> &N {
        &self.value_type
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ ValueMissingFrom
// ------------------------------------------------------------------------------------------------

impl_into_validation_issue!(ValueMissingFrom => ValueMissingFrom);
impl_has_severity!(ValueMissingFrom => Error);
impl_display!(ValueMissingFrom, |issue: &ValueMissingFrom<N>| format!(
    "Value{} of type `{}` is missing from the container type `{}` named `{}`.",
    issue
        .property_name
        .as_ref()
        .map(|n| format!(", for property named `{n}`,"))
        .unwrap_or_default(),
    issue.value_type,
    issue.container_type,
    issue
        .container_name
        .as_ref()
        .map(|n| format!(", named `{n}`."))
        .unwrap_or(".".to_string()),
));
impl_error!(ValueMissingFrom);

impl<N: Name> ValueMissingFrom<N> {
    pub fn new_property(property_name: N, value_type: N, container_type: N) -> Self {
        Self {
            property_name: Some(property_name),
            value_type,
            container_name: None,
            container_type,
        }
    }
    pub fn new_type(property_name: N, value_type: N, container_type: N) -> Self {
        Self {
            property_name: Some(property_name),
            value_type,
            container_name: None,
            container_type,
        }
    }
    pub fn in_container_named(mut self, container_name: N) -> Self {
        self.container_name = Some(container_name);
        self
    }

    pub fn property_name(&self) -> Option<&N> {
        self.property_name.as_ref()
    }

    pub fn value_type(&self) -> &N {
        &self.value_type
    }

    pub fn container_name(&self) -> Option<&N> {
        self.container_name.as_ref()
    }

    pub fn container_type(&self) -> &N {
        &self.container_type
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ ValueOutOfBounds
// ------------------------------------------------------------------------------------------------

impl_into_validation_issue!(ValueOutOfBounds => ValueOutOfBounds);
impl_has_severity!(ValueOutOfBounds => Error);
impl_display!(ValueOutOfBounds, |issue: &ValueOutOfBounds<N>| format!(
    "Value{} of type `{}`, exceeded the {} bound; given: '{}'.",
    issue
        .property_name
        .as_ref()
        .map(|n| format!(", for property named `{n}`,"))
        .unwrap_or_default(),
    issue.value_type,
    issue.exceeded,
    issue.value_as_string
));
impl_error!(ValueOutOfBounds);

impl<N: Name> ValueOutOfBounds<N> {
    pub fn new_property<S: Into<String>>(
        property_name: N,
        value_type: N,
        exceeded: ValueBound,
        value_as_string: S,
    ) -> Self {
        Self {
            property_name: Some(property_name),
            value_type,
            exceeded,
            value_as_string: value_as_string.into(),
        }
    }
    pub fn new_type<S: Into<String>>(
        value_type: N,
        exceeded: ValueBound,
        value_as_string: S,
    ) -> Self {
        Self {
            property_name: None,
            value_type,
            exceeded,
            value_as_string: value_as_string.into(),
        }
    }

    pub fn property_name(&self) -> Option<&N> {
        self.property_name.as_ref()
    }

    pub fn value_type(&self) -> &N {
        &self.value_type
    }

    pub fn exceeded(&self) -> ValueBound {
        self.exceeded
    }

    pub fn value_as_string(&self) -> &str {
        self.value_as_string.as_str()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ ValueBound
// ------------------------------------------------------------------------------------------------

impl Display for ValueBound {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match (self, f.alternate()) {
                (Self::Min, false) => "minimum",
                (Self::Min, true) => "∧",
                (Self::Max, false) => "maximum",
                (Self::Max, true) => "∨",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ InvalidDependentPropertyValue
// ------------------------------------------------------------------------------------------------

impl_into_validation_issue!(ValueOfDependentPropertyInvalid => ValueOfDependentPropertyInvalid);
impl_has_severity!(ValueOfDependentPropertyInvalid => Error);
impl_display!(
    ValueOfDependentPropertyInvalid,
    |issue: &ValueOfDependentPropertyInvalid<N>| {
        format!("Invalid value '{}' (as string) for field `{}` because it depends on field `{}`; actual rule: {}.",
            issue.value_as_string, issue.property_name, issue.depends_on, issue.reason
)
    }
);
impl_error!(ValueOfDependentPropertyInvalid);

impl<N: Name> ValueOfDependentPropertyInvalid<N> {
    pub fn new<V: Into<String>, S: Into<String>>(
        property_name: N,
        depends_on: N,
        value_as_string: V,
        reason: S,
    ) -> Self {
        Self {
            property_name,
            depends_on,
            value_as_string: value_as_string.into(),
            reason: reason.into(),
        }
    }

    pub fn property_name(&self) -> &N {
        &self.property_name
    }

    pub fn depends_on(&self) -> &N {
        &self.depends_on
    }

    pub fn value_as_string(&self) -> &String {
        &self.value_as_string
    }

    pub fn reason(&self) -> &str {
        self.reason.as_str()
    }
}
