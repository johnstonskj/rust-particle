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

/// 
/// Provides a trait for types that have an associated scope.
/// 
pub trait HasScope {
    /// The scope implementation used by the enclosing type.
    type Scope: Scope;

    fn scope(&self) -> &Self::Scope;
}

/// 
/// Provides a trait for types that have a mutable associated scope.
/// 
pub trait HasScopeMut: HasScope {
    fn set_scope(&mut self, scope: Self::Scope);
}

/// 
/// Provides a trait for types that have an optional associated scope.
/// 
pub trait HasOptionalScope {
    /// The scope implementation used by the enclosing type.
    type Scope: Scope;

    fn is_scoped(&self) -> bool {
        self.scope().is_some()
    }
    fn scope(&self) -> Option<&Self::Scope>;
}

/// 
/// Provides a trait for types that have a mutable optional associated scope.
/// 
pub trait HasOptionalScopeMut: HasOptionalScope {
    fn set_scope(&mut self, scope: Self::Scope);

    fn unset_scope(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

///
/// Provides a builder mix-in fragment for types that can have scopes.
/// 
pub trait ScopeBuilderFragment {
    /// The scope implementation to be included in the built object.
    type Scope: Scope;

    fn with_scope(self, scope: Self::Scope) -> Self;
}
