//!
//! Provides traits for property-like things.
//!

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Denotes a type that has a single, globally accessible instance, or at least the appearance of
/// one.
///
pub trait Singleton {
    fn instance() -> &'static Self;
}

///
/// Denotes a type whose internal details are hidden or not exposed.
///
pub trait Opaque {}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod classifiers;
pub mod configuration;
pub mod descriptions;
pub mod errors;
pub mod help;
pub mod icons;
pub mod identifiers;
pub mod labels;
pub mod names;
pub mod priorities;
pub mod scopes;
pub mod states;
pub mod tags;
pub mod values;
