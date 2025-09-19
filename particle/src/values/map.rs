//!
//! Particle Map-like value trait.
//!

use crate::values::Value;
use core::fmt::Debug;

pub trait Map:
    Value + Default + Debug + FromIterator<(String, Self::Value)> + Extend<(String, Self::Value)>
{
    type Value: Value;

    fn append(&mut self, other: &mut Self);

    fn clear(&mut self);

    fn contains_key<S: AsRef<str>>(&self, key: S) -> bool;

    fn get<S: AsRef<str>>(&self, key: S) -> Option<&Self::Value>;

    fn get_key_value<S: AsRef<str>>(&self, key: S) -> Option<(&String, &Self::Value)>;

    fn get_mut<S: AsRef<str>>(&mut self, key: S) -> Option<&mut Self::Value>;

    fn insert(&mut self, key: String, value: Self::Value) -> Option<Self::Value>;

    fn is_empty(&self) -> bool;

    fn iter(&self) -> impl Iterator<Item = (&String, &Self::Value)>;

    fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Self::Value)>;

    fn keys(&self) -> impl Iterator<Item = &String>;

    fn len(&self) -> usize;

    fn new() -> Self;

    fn remove<S: AsRef<str>>(&mut self, key: S) -> Option<Self::Value>;

    fn retain<F: FnMut(&String, &mut Self::Value) -> bool>(&mut self, f: F);

    fn values(&self) -> impl Iterator<Item = &Self::Value>;

    fn values_mut(&mut self) -> impl Iterator<Item = &mut Self::Value>;
}
