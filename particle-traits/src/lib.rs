//!
//! Core traits for the particle component system.
//!
//! More detailed description, with
//!
//! # Feature flags
//!
//! * **alloc** (*enabled by default*) — Implies the use of the `alloc` crate, and the absence of
//!   the standard library.
//! * **doc-macros** — **if** enabled during rustdoc generation, internal macros **will be** exported
//!   for documentation purposes. This is useful for understanding internal implementation details.
//!

#[cfg(not(feature = "std"))]
extern crate alloc;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

pub mod builders;
pub mod commands;
pub mod properties;
pub mod strings;
pub mod validation;
pub mod wrappers;
