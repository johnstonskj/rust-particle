//!
//! Provides traits, and classifiers, for Particle names.
//!

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Individual Name
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NameClass;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod macros;

pub mod traits;
pub use traits::{HasName, HasNameMut, Name};
