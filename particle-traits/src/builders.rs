//!
//! Particle Builder traits.
//!

use core::convert::Infallible;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder
// ------------------------------------------------------------------------------------------------

pub trait Builder: Clone + Default {
    type Output;
    type Error: core::error::Error;

    fn build(self) -> Result<Self::Output, Self::Error>;
}

pub trait InfallibleBuilder: Clone + Default {
    type Output;
    fn build(self) -> Self::Output;
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

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<T, O> Builder for T
where
    T: InfallibleBuilder<Output = O>,
{
    type Output = O;
    type Error = Infallible;

    fn build(self) -> Result<Self::Output, Self::Error> {
        Ok(self.build())
    }
}
