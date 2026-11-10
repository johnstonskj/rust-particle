//!
//! Povides property traits for help-related metadata.
//!

use crate::{properties::identifiers::HasIdentifier, strings::LanguageString};
use core::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Error Code
// ------------------------------------------------------------------------------------------------

///
/// The core traits required of a URL type.
/// 
pub trait Url: Clone + Display + Eq {}

has_traits! {
    Url =>
    "Provides a mixin trait for types that have an `url` property."
    HasUrl { url }
    "Provides a mixin trait for types that have a mutable `url` property."
    HasUrlMut { set_url }
}

///
/// Provides a trait for structured help message types.
/// 
/// A help message is expected to have an associated identifier, should be localized, and 
/// optionally have an associated URL for more information.
/// 
pub trait HelpMessage: LanguageString + HasIdentifier + HasUrl {}
