//!
//! Particle Timestamp value trait.
//!

use crate::{values::traits::Value, wrappers::IntoInner};
use core::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Timestamp
// ------------------------------------------------------------------------------------------------

///
/// 1. Time is expressed in UTC.
/// 2. Time is based on a delta from the Unix Epoch.
/// 3. Time can be negative representing time before the epoch.
///
pub trait Timestamp: Value + IntoInner<i128> {
    fn now() -> Self;

    fn precision(&self) -> TimestampPrecision;

    fn value(&self) -> i128;
}

// impl Binary, Decimal, Octal, LowerHex, UpperHex, ??

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Timestamp Precision
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum TimestampPrecision {
    Second = 0,
    Millisecond = 3,
    Microsecond = 6,
    #[default]
    Nanosecond = 9,
    Picosecond = 12,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Timestamp Precision
// ------------------------------------------------------------------------------------------------

impl Display for TimestampPrecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let exponent = *self as u8;
            let actual: i32 = if exponent == 0 { 0 } else { -(exponent as i32) };
            write!(f, "10^{actual}")
        } else {
            write!(
                f,
                "{}",
                match self {
                    Self::Second => "s",
                    Self::Millisecond => "ms",
                    Self::Microsecond => "µs",
                    Self::Nanosecond => "ns",
                    Self::Picosecond => "ps",
                }
            )
        }
    }
}

impl TimestampPrecision {}
