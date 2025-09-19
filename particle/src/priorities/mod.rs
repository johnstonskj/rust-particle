//!
//! Provides traits, and classifiers, for Particle prioritization.
//!

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Priority Classifier
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PriorityClass;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod macros;

pub mod traits;
pub use traits::{HasPriority, HasPriorityMut, Priority};
