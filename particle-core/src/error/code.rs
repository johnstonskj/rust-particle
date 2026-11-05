//!
//! Particle `[StructuredError]` trait.
//!

use crate::wrappers::IntoInner;
use core::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::Hash,
    num::NonZeroU32,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait ErrorCode: Clone + Copy + Debug + Display + PartialEq + Eq + Hash {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ErrorCodeUInt(NonZeroU32);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ErrorCodeUInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:05}", self.0)
    }
}

impl ErrorCode for ErrorCodeUInt {}

impl From<ErrorCodeUInt> for NonZeroU32 {
    fn from(value: ErrorCodeUInt) -> Self {
        value.0
    }
}

impl IntoInner<NonZeroU32> for ErrorCodeUInt {
    fn into_inner(self) -> NonZeroU32 {
        self.0
    }
}

impl ErrorCodeUInt {
    pub const fn new(code: NonZeroU32) -> Self {
        Self(code)
    }
}
