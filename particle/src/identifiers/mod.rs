//!
//! Provides traits for Particles.
//!

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Classifiers
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdentifierClass;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod macros;

pub mod traits;
pub use traits::{HasIdentifier, HasIdentifierMut, Identifier};
