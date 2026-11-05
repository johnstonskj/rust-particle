//!
//! Particle `[ErrorCode]` trait.
//!

use core::{
    fmt::{Debug, Display},
    hash::Hash,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait ErrorCode: Clone + Copy + Debug + Display + PartialEq + Eq + Hash {}
