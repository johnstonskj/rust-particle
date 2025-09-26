//!
//! Particle Builder traits.
//!

use crate::{names::Name, validation::ValidationError};
use core::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder
// ------------------------------------------------------------------------------------------------

pub trait BuilderRoot: Debug + Default {
    type Output;
}

pub trait Builder: BuilderRoot {
    fn build<N: Name>(self) -> Result<<Self as BuilderRoot>::Output, ValidationError<N>>;
}

pub trait InfallibleBuilder: BuilderRoot {
    fn build(self) -> <Self as BuilderRoot>::Output;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Builder
// ------------------------------------------------------------------------------------------------

pub trait HasBuilder {
    type Output;
    type Builder: Builder<Output = Self::Output>;

    fn builder() -> Self::Builder {
        Self::Builder::default()
    }
}

pub trait HasInfallibleBuilder {
    type Output;
    type Builder: InfallibleBuilder<Output = Self::Output>;

    fn builder() -> Self::Builder {
        Self::Builder::default()
    }
}
