//!
//! Particle prioritization traits.
//!

use crate::{classes::HasClassification, names::traits::HasName, strings::HasTextIcon};
use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Sub},
    str::FromStr,
};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Priority
// ------------------------------------------------------------------------------------------------

pub trait Priority:
    HasClassification
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
    + Sub<u32>
//+ FromStr + TryFrom<u32> + Add<u32, Output=Option<Self>> + Sub<u32, Output=Option<Self>>
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

pub trait HasPriority {
    type Priority: Priority;

    fn current_priority(&self) -> Self::Priority;
}

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
// Blanket Implementations
// ------------------------------------------------------------------------------------------------

impl<T: Priority> HasTextIcon for T {
    fn text_icon(&self) -> &'static str {
        if self.is_highest() {
            "▲"
        } else if *self < Self::highest_value() && *self > Self::default() {
            "▵"
        } else if self.is_normal() {
            "◻︎"
        } else if *self < Self::default() && *self > Self::lowest_value() {
            "▿"
        } else if self.is_lowest() {
            "▼"
        } else {
            "‼︎"
        }
    }
}
