//!
//! Provides classifier (class) property traits.
//!
//! A [`Classifier`] represents a class or category that an entity can belong to, in the Particle
//! context it is used to define hierarchical classifications for components and entities. As such
//! the intent is that classifiers are defined as singletons, they have a unique identity within the
//! system, both as an identifier path but also as a unique instance within the system.
//!

use crate::properties::{Singleton, identifiers::HasIdentifier};

#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Class
// ------------------------------------------------------------------------------------------------

///
/// Provides a trait for classifier types, these are global class-like values and as such are
/// expected to be unique within the system.
///
pub trait Classifier: Singleton + HasIdentifier {
    type Super: Classifier<Identifier = <Self as HasIdentifier>::Identifier>;

    /* const */
    fn super_class(&self) -> Option<&Self>;

    ///
    /// Returns a vector of identifiers representing the parh from this current instance through
    /// it's parent to the root.
    ///
    /// The first element in the vector is the root identifier of the path and the last element is
    /// the current instance identifier.
    ///
    fn class_path(&self) -> Vec<<Self as HasIdentifier>::Identifier> {
        let class_id = self.identifier();
        if let Some(super_class) = self.super_class() {
            let mut super_path = super_class.class_path();
            super_path.push(class_id.clone());
            super_path
        } else {
            vec![class_id.clone()]
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Classification
// ------------------------------------------------------------------------------------------------

has_traits! {
    Classifier =>
    "Provides a mixin trait for types that have a `classifier` property."
    HasClassifier { classifier }
    "Provides a mixin trait for types that have a mutable `classifier` property."
    HasClassifierMut { set_classifier }
}

has_traits! {
    #[iter] Classifier =>
    "Provides a mixin trait for types that have a `classifiers` property."
    HasClassifiers { classifiers, has_classifiers, classifier_count }
    "Provides a mixin trait for types that have a mutable `classifiers` property."
    HasClassifiersMut {
        set_classifiers, add_classifier, extend_classifiers, remove_classifier,
        clear_classifiers, classifiers_mut
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

builder_fragment! {
    "Provides a builder mix-in fragment for types that can have a `classifiers` property." =>
    ClassifierBuilderFragment impl Classifier
        default with_classifier(classifier) {

        fn_with! {
            "Synonym for [`with_classifier`](ClassifierBuilderFragment::with_classifier)." =>
            with_type(classifier: Self::Classifier) => with_classifier
        }

        fn_with! {
            "Synonym for [`with_classifier`](ClassifierBuilderFragment::with_classifier)." =>
            a(classifier: Self::Classifier) => with_classifier
        }
    }
}
