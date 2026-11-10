//!
//! Provides wrapper-like behavior traits.
//!
//! This set of traits is akin to standard library  traits like `AsRef` and `AsMut`, but uses a
//! more explicit *inner* terminology.
//!

use core::{error::Error, result::Result};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Return a reference to the inner value.
///
pub trait AsInner {
    type Inner;

    fn as_inner(&self) -> &Self::Inner;
}

///
/// Return a mutable reference to the inner value.
///
pub trait AsInnerMut: AsInner {
    fn as_inner_mut(&mut self) -> &mut Self::Inner;
}

///
/// Consume the wrapper and return the inner value.
///
pub trait IntoInner {
    type Inner;

    fn into_inner(self) -> Self::Inner;
}

///
/// Construct a new wrapper from an inner value.
///
pub trait FromInner {
    type Inner;

    fn from_inner(inner: Self::Inner) -> Self;
}

///
/// Return a trusted version of the wrapper if the untrusted version is valid.
///
pub trait Untrusted: FromInner {
    /// The trusted version of this wrapper.
    type Trusted;
    /// The error type returned if the untrusted object cannot be trusted.
    type Error: Error;

    fn trust(self) -> Result<Self::Trusted, Self::Error>;
}

///
/// Return an untrusted version of the wrapper.
///
pub trait Trusted: IntoInner {
    /// The untrusted version of this wrapper.
    type Untrusted;

    fn untrust(self) -> Self::Untrusted;
}
