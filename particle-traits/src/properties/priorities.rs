//!
//! Provides priority property traits.
//!

use crate::properties::{classifiers::HasClassifier, names::HasName};
use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Sub},
    str::FromStr,
};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Priority
// ------------------------------------------------------------------------------------------------

///
/// Provides a trait for priority types.
///
pub trait Priority:
    HasClassifier
    + HasName
    + Clone
    + Copy
    + Debug
    + Default
    + Display
    + PartialEq
    + Eq
    + Hash
    + PartialOrd
    + Ord
    + FromStr
    + Into<u32>
    + TryFrom<u32>
    + Add<u32>
    + Add<u32, Output = Option<Self>>
    + Sub<u32>
    + Sub<u32, Output = Option<Self>>
{
    fn highest_value() -> Self;
    fn lowest_value() -> Self;
    fn is_highest(&self) -> bool {
        *self == Self::highest_value()
    }
    fn is_normal(&self) -> bool {
        *self == Self::default()
    }
    fn is_lowest(&self) -> bool {
        *self == Self::lowest_value()
    }

    fn increase(&self) -> Self {
        self.increase_by(1)
    }
    fn increase_or(&self) -> Option<Self> {
        self.increase_by_or(1)
    }
    fn increase_by(&self, levels: u32) -> Self {
        self.increase_by_or(levels).unwrap_or(*self)
    }
    fn increase_by_or(&self, levels: u32) -> Option<Self>;
    fn decrease(&self) -> Self {
        self.decrease_by(1)
    }
    fn decrease_or(&self) -> Option<Self> {
        self.decrease_by_or(1)
    }
    fn decrease_by(&self, levels: u32) -> Self {
        self.decrease_by_or(levels).unwrap_or(*self)
    }
    fn decrease_by_or(&self, levels: u32) -> Option<Self>;
}

///
/// Provides a trait for types that have an associated priority.
///
pub trait HasPriority {
    /// The priority implementation to be included in the built object.
    type Priority: Priority;

    /* const */
    fn current_priority(&self) -> Self::Priority;
}

///
/// Provides a trait for types that have a mutable associated priority.
///
pub trait HasPriorityMut: HasPriority {
    fn set_priority(&mut self, priority: Self::Priority);

    fn priority_increase(&mut self) {
        self.set_priority(self.current_priority().increase())
    }
    fn priority_increase_by(&mut self, levels: u32) {
        self.set_priority(self.current_priority().increase_by(levels))
    }
    fn priority_decrease(&mut self) {
        self.set_priority(self.current_priority().decrease())
    }
    fn priority_decrease_by(&mut self, levels: u32) {
        self.set_priority(self.current_priority().decrease_by(levels))
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

builder_fragment! {
    "Provides a builder mix-in fragment for types that can have priorities." =>
    PriorityBuilderFragment impl Priority
        default with_priority(priority) {

        fn_with! {
            "Synonym for [`with_priority`](PriorityBuilderFragment::with_priority)." =>
            prioritized_as(priority: Self::Priority) => with_priority
        }
    }
}
