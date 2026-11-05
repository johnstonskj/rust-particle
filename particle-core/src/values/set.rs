//!
//! Particle Set-like value trait.
//!

use crate::values::Value;
use core::fmt::Debug;

pub trait Set: Value + Default + Debug + FromIterator<Self::Value> + Extend<Self::Value> {
    type Value: Value;

    fn clear(&mut self);

    fn contains(&self, value: &Self::Value) -> bool;

    fn difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Self::Value>;

    fn get(&self, value: &Self::Value) -> Option<&Self::Value>;

    fn insert(&mut self, value: Self::Value) -> bool;

    fn intersection<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Self::Value>;

    fn is_disjoint(&self, other: &Self) -> bool;

    fn is_empty(&self) -> bool;

    fn is_subset(&self, other: &Self) -> bool;

    fn is_superset(&self, other: &Self) -> bool;

    fn iter(&self) -> impl Iterator<Item = &Self::Value>;

    fn len(&self) -> usize;

    fn new() -> Self;

    fn remove(&mut self, value: &Self::Value) -> bool;

    fn replace(&mut self, value: Self::Value) -> Option<Self::Value>;

    fn retain<F: FnMut(&Self::Value) -> bool>(&mut self, f: F);

    fn symmetric_difference<'a>(&'a self, other: &'a Self)
        -> impl Iterator<Item = &'a Self::Value>;

    fn take(&mut self, value: &Self::Value) -> Option<Self::Value>;

    fn union<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Self::Value>;
}
