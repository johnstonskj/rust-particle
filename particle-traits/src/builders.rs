//!
//! Provides traits for builder objects, and types that have associated builders.
//!
//! This module defines a pair of traits used as the basis for builder objects, [`Builder`] and
//! [`InfallibleBuilder`]. These consist of a standard builder pattern where `Builder` provides a
//! single `build` method that may fail with an error, and `InfallibleBuilder`'s `build` method
//! cannot fail.
//!
//! In some cases builder objects are separate from the types they construct, and so the object has
//! it's own constructors with `default` being preferred. However, in other cases the type to be
//! constructed provides its own `builder` method to construct a builder object, and the traits
//! [`HasBuilder`] and [`HasInfallibleBuilder`] provide a standard interface for such types.
//!
//! # Examples
//!
//! TBD
//!

use core::{convert::Infallible, error::Error as CoreError};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder
// ------------------------------------------------------------------------------------------------

///
/// A builder that can only be used once to construct an instance of `Output`, with the possibility
/// of returning an error of type `Error`.
///
pub trait OnceBuilder {
    /// The output type that this builder will construct.
    type Output;
    /// The error type that may be returned during construction.
    type Error: CoreError;

    ///
    /// Build a new instance of the type `Output`, returning `Error` if it cannot be constructed.
    ///
    fn build_once(self) -> Result<Self::Output, Self::Error>;
}

///
/// A builder that can only be used once to construct an instance of `Output`, with the guarantee
/// that it will not return an error.
///
pub trait InfallibleOnceBuilder {
    /// The output type that this builder will construct.
    type Output;

    fn build_once(self) -> Self::Output;
}

// ------------------------------------------------------------------------------------------------

///
/// A builder that may be used multiple times to construct instances of `Output`, with the possibility
/// of returning an error of type `Error`.
///
pub trait Builder: OnceBuilder + Clone {
    ///
    /// Build a new instance of the type `Output`, returning `Error` if it cannot be constructed.
    ///
    fn build(&self) -> Result<Self::Output, Self::Error>;
}

///
/// A builder that may be used multiple times to construct instances of `Output`, with the guarantee
/// that it will not return an error.
///
pub trait InfallibleBuilder: InfallibleOnceBuilder + Clone {
    fn build(&self) -> Self::Output;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Builder
// ------------------------------------------------------------------------------------------------

///
/// Denotes that this type provides a builder of type [`OnceBuilder`].
///
pub trait HasOnceBuilder {
    /// The output type that this builder will construct, most usually `Self`.
    type Output;
    /// The builder type used to construct instances of `Output`. Note, the builder returned may
    /// **only** be used once, and **may** return errors during construction.
    type Builder: OnceBuilder<Output = Self::Output>;

    fn builder() -> Self::Builder;
}

///
/// Denotes that this type provides a builder of type [`InfallibleOnceBuilder`].
///
pub trait HasInfallibleOnceBuilder {
    /// The output type that this builder will construct, most usually `Self`.
    type Output;
    /// The builder type used to construct instances of `Output`. Note, the builder returned may
    /// **only** be used once, and **does not** return errors during construction.
    type Builder: InfallibleOnceBuilder<Output = Self::Output>;

    fn builder() -> Self::Builder;
}

// ------------------------------------------------------------------------------------------------

///
/// Denotes that this type provides a builder of type [`Builder`].
///
pub trait HasBuilder {
    /// The output type that this builder will construct, most usually `Self`.
    type Output;
    /// The builder type used to construct instances of `Output`. Note, the builder returned may
    /// be used more than once, and **may** return errors during construction.
    type Builder: Builder<Output = Self::Output>;

    fn builder() -> Self::Builder;
}

///
/// Denotes that this type provides a builder of type [`InfallibleBuilder`].
///
pub trait HasInfallibleBuilder {
    /// The output type that this builder will construct, most usually `Self`.
    type Output;
    /// The builder type used to construct instances of `Output`. Note, the builder returned may
    /// be used more than once, and **does not** return errors during construction.
    type Builder: InfallibleBuilder<Output = Self::Output>;

    fn builder() -> Self::Builder;
}

// ------------------------------------------------------------------------------------------------
// Blanket Implementations
// ------------------------------------------------------------------------------------------------

impl<T, O> OnceBuilder for T
where
    T: InfallibleOnceBuilder<Output = O>,
{
    type Output = O;
    type Error = Infallible;

    fn build_once(self) -> Result<Self::Output, Self::Error> {
        Ok(self.build_once())
    }
}

impl<T, O> Builder for T
where
    T: InfallibleBuilder<Output = O>,
{
    fn build(&self) -> Result<Self::Output, Self::Error> {
        Ok(self.build())
    }
}
