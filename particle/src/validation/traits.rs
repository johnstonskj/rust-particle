//!
//! Particle Validation traits.
//!

use crate::validation::ValidationError;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Validation
// ------------------------------------------------------------------------------------------------

pub trait HasValidation {
    // TODO: auto implemented for any type with HasFastValidation to call the
    // `false` option (in simple).
    fn is_valid(&self) -> bool;
    fn validate(&self) -> Result<(), ValidationError>;
}

pub trait HasFastValidation {
    fn is_valid(&self, fast: bool) -> bool;
    fn validate(&self, fast: bool) -> Result<(), ValidationError>;
}
