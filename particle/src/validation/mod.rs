//!
//! Provides traits and an error for creating common validation constructs for Particles.
//!

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod macros;

pub mod error;
pub use error::ValidationError;

pub mod traits;
pub use traits::{HasFastValidation, HasValidation};
