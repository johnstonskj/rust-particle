//!
//! Particle error _scope_.
//!

use crate::names::Name;
use core::error::Error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait ErrorHasScope: Error {
    type Name: Name;

    fn scope(&self) -> &Self::Name;
}

pub trait ErrorHasOptionalScope: Error {
    type Name: Name;

    fn is_scoped(&self) -> bool;
    fn scope(&self) -> Option<&Self::Name>;
}
