//!
//! Provides traits for Particle string properties.
//!
use core::{convert::Infallible, fmt::Display, hash::Hash, str::FromStr};

#[cfg(not(feature = "std"))]
use alloc::string::String;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ String Helpers
// ------------------------------------------------------------------------------------------------

pub trait AsStr {
    fn as_str(&self) -> &str;
}

pub trait CheckedFromStr: FromStr {
    fn is_valid<S: AsRef<str>>(s: S) -> bool {
        Self::from_str(s.as_ref()).is_ok()
    }
}

pub trait InfallibleFromStr: FromStr<Err = Infallible> {
    fn new<S: AsRef<str>>(s: S) -> Self {
        Self::from_str(s.as_ref()).unwrap()
    }
}

pub trait UncheckedNewStr {
    fn new_unchecked<S: AsRef<str>>(s: S) -> Self;
}

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
    + Into<String>
    + AsStr
{
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// TBD
///
/// # Example
///
/// ```rust
/// assert!(is_valid::<u8>("21"));
/// assert!(is_valid::<u16>("hello"));
/// ```
///
pub fn is_valid<S, T>(s: S) -> bool
where
    S: AsRef<str>,
    T: FromStr,
{
    T::from_str(s.as_ref()).is_ok()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod properties;

pub mod descriptions;

pub mod icons;

pub mod labels;

pub mod tags;
