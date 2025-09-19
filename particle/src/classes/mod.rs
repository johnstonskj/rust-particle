//!
//! Provides traits, and classifiers, for Particle classification.
//!

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Classifiers
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TopClass;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassClass;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod macros;

pub mod traits;
pub use traits::{Classifier, HasClassification};
