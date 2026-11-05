//!
//! Particle core.
//!
//! More detailed description, with
//!
//! # Example
//!
//! ```rust
//! ```
//!
//! # Features
//!
//! - **feature-name**; Feature description
//!

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod builders;

pub mod classifiers;

pub mod configuration;

pub mod errors;

pub mod identifiers;

pub mod names;

pub mod priorities;

pub mod properties;

pub mod scopes;

pub mod states;

pub mod strings;

pub mod validation;

pub mod values;

pub mod wrappers;
