//!
//! Particle Builder traits.
//!

use crate::builders::BuilderError;
use core::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder
// ------------------------------------------------------------------------------------------------

pub trait BuilderRoot: Debug + Default {
    type Output;
}

pub trait Builder: BuilderRoot {
    fn build(self) -> Result<<Self as BuilderRoot>::Output, BuilderError>;
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
