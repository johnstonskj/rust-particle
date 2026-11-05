//!
//! Particle error _severity_.
//!

use crate::{classifiers::HasClassifier, names::HasName};
use core::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Severity:
    HasClassifier
    + HasName
    + Clone
    + Copy
    + Debug
    + Display
    + PartialEq
    + Eq
    + Hash
    + PartialOrd
    + Ord
    + FromStr
{
    fn warning() -> Self;
    fn is_warning(self) -> bool;

    fn error() -> Self;
    fn is_error(self) -> bool;

    fn critical() -> Self;
    fn is_critical(self) -> bool;

    fn highest_value() -> Self;
    fn lowest_value() -> Self;
    fn is_highest(&self) -> bool {
        *self == Self::highest_value()
    }
    fn is_lowest(&self) -> bool {
        *self == Self::lowest_value()
    }
}

pub trait HasSeverity {
    type Severity: Severity;

    fn severity(&self) -> Self::Severity;
}
