use crate::properties::PropertyError;
use core::{convert::Infallible, error::Error, fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ String Helpers
// ------------------------------------------------------------------------------------------------

pub trait AsStr: AsRef<str> + Into<String> {
    #[inline(always)]
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

pub trait CheckedFromStr: FromStr<Err = Self::Error> {
    type Error: Error;

    fn is_valid<S: AsRef<str>>(s: S) -> bool;
}

pub trait UncheckedFromStr: FromStr {
    fn new_unchecked<S: AsRef<str>>(s: S) -> Self;
}

pub trait InfallibleFromStr: FromStr<Err = Infallible> {
    fn new_unchecked<S: AsRef<str>>(s: S) -> Self {
        Self::from_str(s.as_ref()).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Descriptions
// ------------------------------------------------------------------------------------------------

pub trait TextProperty:
    CheckedFromStr<Error = PropertyError>
    + AsStr
    + Clone
    + Display
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Into<String>
{
}

pub trait LengthLimitedTextProperty: TextProperty {
    fn new_truncate(s: &str, ellipsis: bool) -> Result<Self, PropertyError> {
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
