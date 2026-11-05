//!
//! Particle error _scope_.
//!

use crate::names::Name;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Scope {
    type Name: Name;

    fn scope_name(&self) -> &Self::Name;
}

pub trait HasScope {
    type Name: Name;
    type Scope: Scope<Name = Self::Name>;

    fn scope(&self) -> &Self::Scope;
}

pub trait HasScopeMut: HasScope {
    fn set_scope(&mut self, scope: Self::Scope);
}

pub trait HasOptionalScope {
    type Name: Name;
    type Scope: Scope<Name = Self::Name>;

    fn is_scoped(&self) -> bool {
        self.scope().is_some()
    }
    fn scope(&self) -> Option<&Self::Scope>;
}

pub trait HasOptionalScopeMut: HasOptionalScope {
    fn set_scope(&mut self, scope: Self::Scope);

    fn unset_scope(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

pub trait ScopeBuilderFragment {
    type Scope: Scope;

    fn with_scope_named(self, scope: Self::Scope) -> Self;
}
