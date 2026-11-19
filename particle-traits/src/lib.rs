//!
//! Core traits for the particle component system.
//!
//! More detailed description, with
//!
//! # Feature flags
//!
//! * **doc-macros** — **if** enabled during rustdoc generation, internal macros **will be** exported
//!   for documentation purposes. This is useful for understanding internal implementation details.
//!
#![no_std]

extern crate alloc;

// ------------------------------------------------------------------------------------------------
// Meta-Macro
// ------------------------------------------------------------------------------------------------

macro_rules! prelude {
    ($edition:ident) => {
        #[cfg(target_os = "none")]
        use core::prelude::$edition::*;
    };
}

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
