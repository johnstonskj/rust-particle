/*!
Particle core.

More detailed description, with

# Example

```rust
```

# Features

- **feature-name**; Feature description

*/

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Particle {}

// ------------------------------------------------------------------------------------------------
// Test Code Examples in README file.
// ------------------------------------------------------------------------------------------------

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDocTests;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod builders;

pub mod classes;

pub mod configuration;

pub mod error;

pub mod identifiers;

pub mod names;

pub mod priorities;

pub mod properties;

pub mod states;

pub mod strings;

pub mod validation;

pub mod values;

pub mod wrappers;
