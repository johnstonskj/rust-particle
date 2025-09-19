use crate::classes::HasClassification;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Identifier
// ------------------------------------------------------------------------------------------------

pub trait Identifier:
    HasClassification + Clone + Debug + Display + PartialEq + Eq + Hash + PartialOrd + Ord
{
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Identifier Generators
// ------------------------------------------------------------------------------------------------

pub trait IdGenerator {
    type Identifier: Identifier;

    fn generate(&self) -> Self::Identifier;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Identifier traits
// ------------------------------------------------------------------------------------------------

pub trait HasIdentifier {
    type Identifier: Identifier;

    /* const */
    fn id(&self) -> &Self::Identifier;
}

pub trait HasIdentifierMut: HasIdentifier {
    /* const */
    fn set_id(&mut self, id: Self::Identifier);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Parent Identifier traits
// ------------------------------------------------------------------------------------------------

pub trait HasParentIdentifier: HasIdentifier {
    /* const */
    fn has_parent_id(&self) -> bool;

    /* const */
    fn parent_id(&self) -> Option<&Self::Identifier>;
}

pub trait HasParentIdentifierMut: HasParentIdentifier {
    fn set_parent_id(&mut self, parent_id: Self::Identifier);

    fn unset_parent_id(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has Child Identifier traits
// ------------------------------------------------------------------------------------------------

pub trait HasChildIdentifiers: HasIdentifier {
    /* const */
    fn child_ids_count(&self) -> usize;

    /* const */
    fn has_child_ids(&self) -> bool;

    fn child_ids(&self) -> impl Iterator<Item = &Self::Identifier>;
}

pub trait HasChildIdentifiersMut: HasChildIdentifiers {
    fn child_ids_mut(&mut self) -> impl Iterator<Item = &mut Self::Identifier>;

    fn add_child_id(&mut self, child_id: Self::Identifier);

    fn remove_child_id(&mut self, child_id: &Self::Identifier);

    fn clear_child_ids(&mut self);
}
