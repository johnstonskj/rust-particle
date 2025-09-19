//!
//! Provides traits and an error for *values* in Particles.
//!

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod map;
pub use map::Map;

pub mod set;
pub use set::Set;

pub mod timestamp;
pub use timestamp::Timestamp;

pub mod traits;
pub use traits::{HasValue, HasValueMut, Value};

pub mod vec;
pub use vec::Vec;
