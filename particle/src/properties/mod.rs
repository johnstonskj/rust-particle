//!
//! Provides traits for Particle properties.
//!

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "modeled-properties")]
pub mod modeled;

pub mod traits;
pub use traits::Singleton;

pub mod error;
pub use error::PropertyError;
