//!
//! Provides traits for wrapper types in Particles.
//!

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod macros;

pub mod traits;
pub use traits::{AsInner, AsInnerMut, FromInner, IntoInner};
