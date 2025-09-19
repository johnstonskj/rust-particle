//!
//! Provides traits and an error for creating common builder constructs for Particles.
//!

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;
pub use error::BuilderError;

pub mod traits;
pub use traits::{Builder, BuilderRoot, HasBuilder, HasInfallibleBuilder, InfallibleBuilder};
