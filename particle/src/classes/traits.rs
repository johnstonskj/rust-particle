//!
//! Particle Builder traits.
//!

use crate::names::{HasName, Name};
use core::{fmt::Debug, hash::Hash};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Class
// ------------------------------------------------------------------------------------------------

pub trait Classifier:
    HasName + Clone + Debug + Default + PartialEq + Eq + Hash + PartialOrd + Ord
{
    type Super: Classifier;

    /* const */
    fn super_class(&self) -> Option<Self::Super>;

    fn class_path<N: Name>(&self) -> Vec<N> {
        let class_name = self.name();
        if let Some(super_class) = self.super_class() {
            let mut super_path = super_class.class_path();
            super_path.push(N::new_unchecked(class_name.as_ref()));
            super_path
        } else {
            vec![N::new_unchecked(class_name.as_ref())]
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Classification
// ------------------------------------------------------------------------------------------------

pub trait HasClassification {
    type Classifier: Classifier;

    /* const */
    fn classifiers(&self) -> impl Iterator<Item = &Self::Classifier>;
}
