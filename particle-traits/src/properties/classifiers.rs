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

/// 
/// Provides a trait for types that have an associated classifier.
/// 
pub trait HasClassifier {
    /// The classifier implementation to be included in the built object.
    type Classifier: Classifier;

    /* const */
    fn classifier(&self) -> &Self::Classifier;
}

/// 
/// Provides a trait for types that have a mutable associated classifier.
/// 
pub trait HasClassifierMut: HasClassifier {
    fn set_classifier(&mut self, classifier: Self::Classifier);
}

/// 
/// Provides a trait for types that have multiple associated classifiers.
/// 
pub trait HasClassifiers {
    /// The classifier implementation to be included in the built object.
    type Classifier: Classifier;

    /* const */
    fn classifiers(&self) -> impl Iterator<Item = &Self::Classifier>;
}

/// 
/// Provides a trait for types that have mutable access to multiple associated classifiers.
/// 
pub trait HasClassifiersMut: HasClassifiers {
    /* const */
    fn classifiers_mut(&mut self) -> impl Iterator<Item = &mut Self::Classifier>;

    fn set_classifiers<I: IntoIterator<Item = Self::Classifier>>(&mut self, classifiers: I);

    fn add_classifier(&mut self, classifier: Self::Classifier);

    fn extend_classifiers<I: IntoIterator<Item = Self::Classifier>>(&mut self, classifiers: I);

    fn remove_classifier(&mut self, classifier: &Self::Classifier) -> Option<Self::Classifier>;

    fn clear_classifiers(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

///
/// Provides a builder mix-in fragment for types that can have classifiers.
/// 
pub trait ClassifierBuilderFragment {
    /// The classifier implementation to be included in the built object.
    type Classifier: Classifier;

    fn with_classifier(self, classifier: Self::Classifier) -> Self;

    /// 
    /// Synonym for [`with_classifier`](ClassifierBuilderFragment::with_classifier).
    /// 
    fn with_type(self, classifier: Self::Classifier) -> Self
    where
        Self: Sized,
    {
        self.with_classifier(classifier)
    }

    /// 
    /// Synonym for [`with_classifier`](ClassifierBuilderFragment::with_classifier).
    /// 
    fn a(self, classifier: Self::Classifier) -> Self
    where
        Self: Sized,
    {
        self.with_classifier(classifier)
    }
}
