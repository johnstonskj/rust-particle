//!
//! Provides traits for Particle string properties.
//!

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod descriptions;
pub use descriptions::{Description, HasDescription, HasDescriptionMut};

pub mod icons;
pub use icons::{HasFontIcon, HasTextIcon};

pub mod labels;
pub use labels::{DisplayLabel, HasDisplayLabel, HasDisplayLabelMut};

pub mod tags;
pub use tags::{HasTags, HasTagsMut, Tag};
