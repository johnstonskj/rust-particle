//!
//! Provides traits, and classifiers, for Particle states.
//!

// ------------------------------------------------------------------------------------------------
// Public Types ❱  State / Set
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StateClass;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod macros;

pub mod error;

pub mod traits;
pub use traits::{HasState, HasStateMut, State, StateBuilder, StateKind};
