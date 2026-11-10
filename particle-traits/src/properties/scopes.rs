//!
//! Provides scope property traits.
//!

use crate::properties::names::HasName;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Provides a trait for scope types.
///
pub trait Scope: HasName {}

has_traits! {
    Scope =>
    "Provides a mixin trait for types that have a `scope` property."
    HasScope { scope }
    "Provides a mixin trait for types that have a mutable `scope` property."
    HasScopeMut { set_scope }
}

has_traits! {
    #[optional] Scope =>
    "Provides a mixin trait for types that have an optional `scope` property."
    HasOptionalScope { scope, has_scope }
    "Provides a mixin trait for types that have a mutable optional `scope` property."
    HasOptionalScopeMut { set_scope, unset_scope }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

builder_fragment! {
    "Provides a builder mix-in fragment for types that can have scopes." =>
    ScopeBuilderFragment impl Scope
        default with_scope(scope) {}
}
