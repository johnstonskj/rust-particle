//!
//! Provides traits, and classifiers, for Particle states.
//!

use crate::{defn_class, impl_class};

// ------------------------------------------------------------------------------------------------
// Public Types ❱  Classifiers
// ------------------------------------------------------------------------------------------------

defn_class! { State }

defn_class! { CompositeState }

// ------------------------------------------------------------------------------------------------
// Implementations ❱  Classifiers
// ------------------------------------------------------------------------------------------------

impl_class! { State }

impl_class! { CompositeState }

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod macros;

pub mod traits;
pub use traits::{HasState, HasStateMut, State, StateBuilder, StateKind};

pub mod validation;
