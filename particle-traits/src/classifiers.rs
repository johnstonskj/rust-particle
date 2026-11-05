//!
//! Particle Builder traits.
//!

use crate::{names::Name, properties::Singleton};
use core::{fmt::Debug, hash::Hash};

#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Class
// ------------------------------------------------------------------------------------------------

pub trait Classifier:
    Singleton + Clone + Debug + Default + PartialEq + Eq + Hash + PartialOrd + Ord
{
    type Name: Name;
    type Super: Classifier<Name = Self::Name>;

    /* const */
    fn name(&self) -> &Self::Name;

    /* const */
    fn super_class(&self) -> Option<&Self::Super>;

    fn class_path(&self) -> Vec<<Self as Classifier>::Name> {
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

pub trait HasClassifier {
    type Classifier: Classifier;

    /* const */
    fn classifier(&self) -> &Self::Classifier;
}

pub trait HasClassifierMut: HasClassifier {
    /* const */
    fn set_classifier(&mut self, classifier: Self::Classifier);
}

pub trait HasClassifiers {
    type Classifier: Classifier;

    /* const */
    fn classifiers(&self) -> impl Iterator<Item = &Self::Classifier>;
}

pub trait HasClassifiersMut: HasClassifiers {
    /* const */
    fn classifiers_mut(&mut self) -> impl Iterator<Item = &mut Self::Classifier>;

    fn set_classifiers<I: IntoIterator<Item = Self::Classifier>>(&mut self, classifiers: I);

    fn add_classifier(&mut self, classifier: Self::Classifier);

    fn extend_classifiers<I: IntoIterator<Item = Self::Classifier>>(&mut self, classifiers: I);

    fn remove_classifier(&mut self, classifier: &Self::Classifier) -> Option<Self::Classifier>;

    fn clear_classifiers(&mut self);
}
