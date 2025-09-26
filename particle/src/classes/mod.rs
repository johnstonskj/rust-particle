//!
//! Provides traits, and classifiers, for Particle classification.
//!

use crate::{defn_class, impl_class, properties::Singleton};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Classifiers
// ------------------------------------------------------------------------------------------------

defn_class! { Top }

defn_class! { Class }

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Classifiers
// ------------------------------------------------------------------------------------------------

const TOP_CLASS_NAME_STR: &str = "TopClass";
const TOP_CLASS_NAME: ClassifierName = ClassifierName::new_unchecked(TOP_CLASS_NAME_STR);
pub const TOP_CLASS: TopClass = TopClass::new();

impl Classifier for TopClass {
    type Super = Self;

    fn super_class(&self) -> Option<&Self::Super> {
        None
    }

    fn name(&self) -> &ClassifierName {
        &TOP_CLASS_NAME
    }
}
impl Singleton for TopClass {
    fn instance() -> &'static Self {
        &TOP_CLASS
    }
}

impl TopClass {
    pub const fn new() -> Self {
        Self
    }
}

// ------------------------------------------------------------------------------------------------

impl_class! {
    Class : TopClass
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod macros;

pub mod name;
pub use name::ClassifierName;

pub mod traits;
pub use traits::{Classifier, HasClassification};
