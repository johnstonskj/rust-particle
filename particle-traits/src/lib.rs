//!
//! Core traits for the particle component system.
//!
//! More detailed description, with
//!
//! # Feature flags
//!
//! * **std** — Includes the standard library.
//! * **alloc** (*enabled by default*) — Implies the use of the `alloc` crate, and the absence of
//!   the standard library.
//! 

#[cfg(not(feature = "std"))]
extern crate alloc;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod builders;
pub mod commands;
pub mod properties;
pub mod strings;
pub mod validation;
pub mod wrappers;
