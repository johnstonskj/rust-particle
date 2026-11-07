//!
//! Provides description property traits.
//!
//! [`Description`] types are used as human-readable longer descriptors for objects, as such they 
//! implement `LengthLimitedTextProperty`.
//! 

use crate::strings::LengthLimitedTextProperty;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Descriptions
// ------------------------------------------------------------------------------------------------

/// 
/// Provides a trait for description types, these are human-readable longer descriptors for objects.
/// 
pub trait Description: LengthLimitedTextProperty {}


/// 
/// Provides a trait for types that have an associated description.
/// 
pub trait HasDescription {
    /// The description implementation to be included in the built object.
    type Description: Description;

    fn description(&self) -> &Self::Description;
}

/// 
/// Provides a trait for types that have a mutable associated description.
/// 
pub trait HasDescriptionMut: HasDescription {
    fn set_description(&mut self, description: Self::Description);
}

/// 
/// Provides a trait for types that have an optional associated description.
/// 
pub trait HasOptionalDescription {
    /// The description implementation to be included in the built object.
    type Description: Description;

    fn description(&self) -> Option<&Self::Description>;
}

/// 
/// Provides a trait for types that have mutable access to an optional associated description.
/// 
pub trait HasOptionalDescriptionMut: HasOptionalDescription {
    fn description_mut(&mut self) -> Option<&mut Self::Description>;

    fn set_description(&mut self, description: Self::Description);

    fn unset_description(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

///
/// Provides a builder mix-in fragment for types that can have descriptions.
/// 
pub trait DescriptionBuilderFragment {
    /// The description implementation to be included in the built object.
    type Description: Description;

    fn with_description(self, description: Self::Description) -> Self;

    /// 
    /// Synonym for [`with_description`](DescriptionBuilderFragment::with_description).
    /// 
    fn described_by(self, description: Self::Description) -> Self
    where
        Self: Sized,
    {
        self.with_description(description)
    }
}
