use crate::strings::CheckedFromStr;
use core::str::FromStr;

#[cfg(not(feature = "std"))]
use alloc::{format, string::ToString};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Descriptions
// ------------------------------------------------------------------------------------------------

pub trait LengthLimitedTextProperty: CheckedFromStr {
    fn new_truncate(s: &str, ellipsis: bool) -> Result<Self, <Self as FromStr>::Err> {
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
