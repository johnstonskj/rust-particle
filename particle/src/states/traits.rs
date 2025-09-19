//!
//! Particle state traits.
//!

use crate::{
    builders::Builder,
    classes::HasClassification,
    names::traits::{BuilderNameFragment, HasName},
    properties::PropertyError,
    strings::{HasTextIcon, UncheckedFromStr},
    validation::HasValidation,
};
use core::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::Hash,
};

// ------------------------------------------------------------------------------------------------
// Public Types ❱  State / Set
// ------------------------------------------------------------------------------------------------

pub trait State:
    HasClassification + HasName + Clone + Debug + PartialEq + Eq + Hash + PartialOrd + Ord
{
    fn kind(&self) -> StateKind;

    #[inline]
    fn is_initial(&self) -> bool {
        matches!(self.kind(), StateKind::Initial)
    }

    #[inline]
    fn is_normal(&self) -> bool {
        matches!(self.kind(), StateKind::Normal)
    }

    #[inline]
    fn is_internal(&self) -> bool {
        matches!(self.kind(), StateKind::Internal)
    }

    #[inline]
    fn is_final(&self) -> bool {
        matches!(self.kind(), StateKind::Final)
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
    fn has_final(&self) -> bool {
        self.iter().any(|s| s.is_final())
    }
}

pub trait StateBuilder<T: State>:
    Builder<Output = T> + BuilderNameFragment<Name = <T as HasName>::Name>
{
    fn a(self, kind: StateKind) -> Self;

    #[inline(always)]
    fn an_initial_state(self) -> Self {
        self.a(StateKind::Initial)
    }

    #[inline(always)]
    fn an_internal_state(self) -> Self {
        self.a(StateKind::Internal)
    }

    #[inline(always)]
    fn a_normal_state(self) -> Self {
        self.a(StateKind::Normal)
    }

    #[inline(always)]
    fn a_final_state(self) -> Self {
        self.a(StateKind::Final)
    }

    #[inline(always)]
    fn new_initial_unchecked<S: AsRef<str>>(name: S) -> Self {
        Self::new_initial(<<T as HasName>::Name as UncheckedFromStr>::new_unchecked(
            name,
        ))
    }

    #[inline(always)]
    fn new_initial(name: <T as HasName>::Name) -> Self {
        Self::default().named(name).a(StateKind::Initial)
    }

    #[inline(always)]
    fn new_normal_unchecked<S: AsRef<str>>(name: S) -> Self {
        Self::new_normal(<<T as HasName>::Name as UncheckedFromStr>::new_unchecked(
            name,
        ))
    }

    #[inline(always)]
    fn new_normal(name: <T as HasName>::Name) -> Self {
        Self::default().named(name).a(StateKind::Normal)
    }

    #[inline(always)]
    fn new_internal_unchecked<S: AsRef<str>>(name: S) -> Self {
        Self::new_internal(<<T as HasName>::Name as UncheckedFromStr>::new_unchecked(
            name,
        ))
    }

    #[inline(always)]
    fn new_internal(name: <T as HasName>::Name) -> Self {
        Self::default().named(name).a(StateKind::Internal)
    }

    #[inline(always)]
    fn new_final_unchecked<S: AsRef<str>>(name: S) -> Self {
        Self::new_final(<<T as HasName>::Name as UncheckedFromStr>::new_unchecked(
            name,
        ))
    }

    #[inline(always)]
    fn new_final(name: <T as HasName>::Name) -> Self {
        Self::default().named(name).a(StateKind::Final)
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ State Kinds
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum StateKind {
    Initial,
    Normal,
    Internal,
    Final,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Has State / Set
// ------------------------------------------------------------------------------------------------

pub trait HasState {
    type State: State;

    fn current_state(&self) -> &Self::State;
}

pub trait HasStateMut: HasState {
    fn set_state(&mut self, state: Self::State) -> Result<(), PropertyError>;
}

pub trait HasPotentialStates {
    type State: State;
    type StateSet: StateSet<State = Self::State>;

    fn potential_states(&self) -> &Self::StateSet;
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ State Kind
// ------------------------------------------------------------------------------------------------

impl Display for StateKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Initial => "Initial".to_string(),
                Self::Normal => "Normal".to_string(),
                Self::Internal => "Internal".to_string(),
                Self::Final => "Final".to_string(),
            }
        )
    }
}

impl HasTextIcon for StateKind {
    fn text_icon(&self) -> &'static str {
        match self {
            Self::Initial => "●→",
            Self::Normal => "→▢→",
            Self::Internal => "▢⤾",
            Self::Final => "→⦿",
        }
    }
}
