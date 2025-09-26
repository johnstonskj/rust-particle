//!
//! Particle Builder traits.
//!

use crate::{classes::ClassifierName, properties::Singleton};
#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};
use core::{fmt::Debug, hash::Hash};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Class
// ------------------------------------------------------------------------------------------------

pub trait Classifier:
    Singleton + Clone + Debug + Default + PartialEq + Eq + Hash + PartialOrd + Ord
{
    type Super: Classifier;

    /* const */
    fn name(&self) -> &ClassifierName;

    /* const */
    fn super_class(&self) -> Option<&Self::Super>;

    fn class_path(&self) -> Vec<ClassifierName> {
        let class_name = self.name();
        if let Some(super_class) = self.super_class() {
            let mut super_path = super_class.class_path();
            super_path.push(class_name.clone());
            super_path
        } else {
            vec![class_name.clone()]
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
