//!
//! Provides traits for string-like types and string constraints such as length-limited text
//! properties.
//!

use core::{convert::Infallible, fmt::Display, hash::Hash, str::FromStr};

#[cfg(not(feature = "std"))]
use alloc::{
    string::{String},
};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ String Behaviors
// ------------------------------------------------------------------------------------------------

///
/// Provides the ability to obtain a string slice from the implementing type.
///
/// This is provided by the standard `String` type and often easier than using `AsRef` for types
/// with multiple reference implementations.
///
pub trait AsStr {
    fn as_str(&self) -> &str;
}

///
/// Provides the ability to check if a string can be converted into the implementing type.
///
/// The associated `is_valid` method checks if the given string is a valid representation of the
/// implementing type. It's default implementation calls `FromStr` to determine validity.
///
pub trait CheckedFromStr: FromStr {
    fn is_valid<S: AsRef<str>>(s: S) -> bool {
        Self::from_str(s.as_ref()).is_ok()
    }
}

///
/// Provides the ability to construct the implementing type from a string without the possibility of
/// failure.
///
pub trait InfallibleFromStr: FromStr<Err = Infallible> {
    fn new<S: AsRef<str>>(s: S) -> Self {
        Self::from_str(s.as_ref()).unwrap()
    }
}

///
/// Provides the ability to construct the implementing type from a string without performing any
/// validation. The caller must ensure that the string is valid for the type.
///
pub trait UncheckedNewStr {
    fn new_unchecked<S: AsRef<str>>(s: S) -> Self;
}

///
/// A base trait that represents types with string-like behavior.
///
/// This trait combines common string-related traits and behaviors, making it easier to work with
/// types that behave like strings.
///
pub trait StringLike:
    Clone
    + Display
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Hash
    + FromStr
    + AsRef<str>
    + AsStr
    + Into<String>
{
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ String Property Helpers
// ------------------------------------------------------------------------------------------------

///
/// Provides a trait for types that have a length constraint on their string representation.
/// 
/// The `is_valid_len` method checks if a given string's length falls within the defined range.
/// 
/// By default, the minimum length is `usize::MIN` and the maximum length is `usize::MAX`.
/// Implementors can override the default minimum and maximum lengths by specifying the `MIN_LENGTH`
/// and `MAX_LENGTH` associated constants.
/// 
pub trait LengthLimitedTextProperty: CheckedFromStr {
    const MIN_LENGTH: usize = usize::MIN;
    const MAX_LENGTH: usize = usize::MAX;

    #[inline(always)]
    fn is_valid_len<S: AsRef<str>>(s: S) -> bool {
        let len = s.as_ref().len();
        (Self::MIN_LENGTH..=Self::MAX_LENGTH).contains(&len)
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Simple string validation function for any type that implements `FromStr`.
///
/// # Example
///
/// ```rust
/// use particle_traits::strings::is_valid;
///
/// assert!(is_valid::<u8>("21"));
/// assert!(!is_valid::<u16>("hello"));
/// ```
///
#[inline(always)]
pub fn is_valid<T>(s: &str) -> bool
where
    T: FromStr,
{
    T::from_str(s).is_ok()
}
