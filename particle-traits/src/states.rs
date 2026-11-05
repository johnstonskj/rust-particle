//!
//! Particle state traits.
//!

use crate::{
    builders::Builder,
    classifiers::HasClassifiers,
    names::{HasName, Name, NameBuilderFragment},
    strings::UncheckedNewStr,
    validation::HasValidation,
};
#[cfg(not(feature = "std"))]
use alloc::{format, string::String};
use core::{
    error::Error as CoreError,
    fmt::{Debug, Display},
    hash::Hash,
};

// ------------------------------------------------------------------------------------------------
// Public Types ❱  State / Set
// ------------------------------------------------------------------------------------------------

pub trait State:
    HasClassifiers + HasName + Clone + Debug + PartialEq + Eq + Hash + PartialOrd + Ord
{
    type Kind: StateKind;

    fn kind(&self) -> Self::Kind;

    #[inline]
    fn is_initial(&self) -> bool {
        self.kind().is_initial()
    }

    #[inline]
    fn is_normal(&self) -> bool {
        self.kind().is_normal()
    }

    #[inline]
    fn is_internal(&self) -> bool {
        self.kind().is_internal()
    }

    #[inline]
    fn is_terminal(&self) -> bool {
        self.kind().is_terminal()
    }

    fn label(&self) -> String {
        format!("[{}: {}]", self.name(), self.kind())
    }
}

pub trait StateSet: HasValidation + Clone + Debug + PartialEq + Eq + PartialOrd + Ord {
    type State: State;

    fn contains(&self, name: &<Self::State as HasName>::Name) -> bool;

    fn get(&self, name: &<Self::State as HasName>::Name) -> Option<&Self::State>;

    fn get_mut(&mut self, name: &<Self::State as HasName>::Name) -> Option<&mut Self::State>;

    fn insert(&mut self, state: Self::State) -> Option<Self::State>;

    fn remove(&mut self, name: &<Self::State as HasName>::Name) -> Option<Self::State>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn clear(&mut self);

    fn iter(&self) -> impl Iterator<Item = &Self::State>;

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Self::State>;

    fn into_iter(self) -> impl Iterator<Item = Self::State>;

    #[inline]
    fn has_initial(&self) -> bool {
        self.iter().any(|s| s.is_initial())
    }

    #[inline]
    fn has_normal(&self) -> bool {
        self.iter().any(|s| s.is_normal())
    }

    #[inline]
    fn has_internal(&self) -> bool {
        self.iter().any(|s| s.is_internal())
    }

    #[inline]
    fn has_terminal(&self) -> bool {
        self.iter().any(|s| s.is_terminal())
    }
}

pub trait StateBuilder<T: State>:
    Builder<Output = T> + NameBuilderFragment<Name = <T as HasName>::Name>
{
    type Kind: StateKind;

    fn a(self, kind: Self::Kind) -> Self;

    #[inline(always)]
    fn an_initial_state(self) -> Self {
        self.a(Self::Kind::initial())
    }

    #[inline(always)]
    fn an_internal_state(self) -> Self {
        self.a(Self::Kind::internal())
    }

    #[inline(always)]
    fn a_normal_state(self) -> Self {
        self.a(Self::Kind::normal())
    }

    #[inline(always)]
    fn a_terminal_state(self) -> Self {
        self.a(Self::Kind::terminal())
    }

    #[inline(always)]
    fn new_initial_unchecked<S: AsRef<str>>(name: S) -> Self {
        Self::new_initial(<<T as HasName>::Name as UncheckedNewStr>::new_unchecked(
            name,
        ))
    }

    #[inline(always)]
    fn new_initial(name: <T as HasName>::Name) -> Self {
        Self::default().named(name).an_initial_state()
    }

    #[inline(always)]
    fn new_normal_unchecked<S: AsRef<str>>(name: S) -> Self {
        Self::new_normal(<<T as HasName>::Name as UncheckedNewStr>::new_unchecked(
            name,
        ))
    }

    #[inline(always)]
    fn new_normal(name: <T as HasName>::Name) -> Self {
        Self::default().named(name).a_normal_state()
    }

    #[inline(always)]
    fn new_internal_unchecked<S: AsRef<str>>(name: S) -> Self {
        Self::new_internal(<<T as HasName>::Name as UncheckedNewStr>::new_unchecked(
            name,
        ))
    }

    #[inline(always)]
    fn new_internal(name: <T as HasName>::Name) -> Self {
        Self::default().named(name).an_internal_state()
    }

    #[inline(always)]
    fn new_terminal_unchecked<S: AsRef<str>>(name: S) -> Self {
        Self::new_terminal(<<T as HasName>::Name as UncheckedNewStr>::new_unchecked(
            name,
        ))
    }

    #[inline(always)]
    fn new_terminal(name: <T as HasName>::Name) -> Self {
        Self::default().named(name).a_terminal_state()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ State Kinds
// ------------------------------------------------------------------------------------------------

pub trait StateKind: Clone + Copy + Debug + Default + Display + PartialEq + Eq + Hash {
    fn initial() -> Self;
    fn is_initial(&self) -> bool;
    fn try_as_initial(&self) -> Option<Self> {
        if self.is_initial() { Some(*self) } else { None }
    }

    fn normal() -> Self;
    fn is_normal(&self) -> bool;
    fn try_as_normal(&self) -> Option<Self> {
        if self.is_normal() { Some(*self) } else { None }
    }

    fn internal() -> Self;
    fn is_internal(&self) -> bool;
    fn try_as_internal(&self) -> Option<Self> {
        if self.is_internal() {
            Some(*self)
        } else {
            None
        }
    }

    fn terminal() -> Self;
    fn is_terminal(&self) -> bool;
    fn try_as_terminal(&self) -> Option<Self> {
        if self.is_terminal() {
            Some(*self)
        } else {
            None
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has State / Set
// ------------------------------------------------------------------------------------------------

pub trait HasState {
    type State: State;

    fn current_state(&self) -> &Self::State;
}

pub trait HasStateMut: HasState {
    type Error: CoreError;

    fn set_state<N: Name>(&mut self, state: Self::State) -> Result<(), Self::Error>;
}

pub trait HasOptionalState {
    type State: State;

    fn current_state(&self) -> Option<&Self::State>;
}

pub trait HasOptionalStateMut: HasOptionalState {
    fn set_state(&mut self, state: Self::State);

    fn unset_state(&mut self);
}

pub trait HasPotentialStates {
    type State: State;
    type StateSet: StateSet<State = Self::State>;

    fn potential_states(&self) -> &Self::StateSet;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Builder Fragment
// ------------------------------------------------------------------------------------------------

pub trait StateBuilderFragment {
    type State: State;

    fn with_state(self, state: Self::State) -> Self;

    fn in_state(self, state: Self::State) -> Self
    where
        Self: Sized,
    {
        self.with_state(state)
    }
}
