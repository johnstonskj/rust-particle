use crate::{names::Name, validation::ValidationError};
#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
};
use core::{convert::Infallible, fmt::Display, str::FromStr};

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

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Descriptions
// ------------------------------------------------------------------------------------------------

pub trait TextProperty<N: Name>:
    CheckedFromStr<Err = ValidationError<N>>
    + Clone
    + Display
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Into<String>
{
}

pub trait LengthLimitedTextProperty<N: Name>: TextProperty<N> {
    fn new_truncate(s: &str, ellipsis: bool) -> Result<Self, ValidationError<N>> {
        let truncated = match (Self::max_length(), ellipsis) {
            (Some(max_length), false) if s.len() > max_length => s[..max_length].to_string(),
            (Some(max_length), true) if s.len() > max_length - 1 => {
                format!("{}…", &s[..max_length - 1])
            }
            _ => s.to_string(),
        };
        Self::from_str(&truncated)
    }

    fn max_length() -> Option<usize>;

    fn is_valid_len<S: AsRef<str>>(s: S) -> bool {
        Self::max_length()
            .map(|max_length| s.as_ref().len() <= max_length)
            .unwrap_or_default()
    }
}
