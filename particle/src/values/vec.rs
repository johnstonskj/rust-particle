//!
//! Particle Vec-like value trait.
//!

use crate::values::Value;
use core::fmt::Debug;

pub trait Vec: Value + Default + Debug + FromIterator<Self::Value> + Extend<Self::Value> {
    type Value: Value;

    fn append(&mut self, other: &mut Self);

    fn clear(&mut self);

    fn contains(&self, value: &Self::Value) -> bool;

    fn first(&self) -> Option<&Self::Value>;

    fn first_mut(&mut self) -> Option<&mut Self::Value>;

    fn get(&self, index: usize) -> Option<&Self::Value>;

    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Value>;

    fn insert(&mut self, index: usize, element: Self::Value);

    fn is_empty(&self) -> bool;

    fn iter(&self) -> impl Iterator<Item = &Self::Value>;

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Value>;

    fn last(&self) -> Option<&Self::Value>;

    fn last_mut(&mut self) -> Option<&mut Self::Value>;

    fn len(&self) -> usize;

    fn new() -> Self;

    fn pop(&mut self) -> Option<Self::Value>;

    fn push(&mut self, value: Self::Value);

    fn remove(&mut self, index: usize) -> Self::Value;

    fn retain<F: FnMut(&Self::Value) -> bool>(&mut self, f: F);

    fn retain_mut<F: FnMut(&mut Self::Value) -> bool>(&mut self, f: F);
}
