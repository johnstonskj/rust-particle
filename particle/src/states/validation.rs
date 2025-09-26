use crate::{
    names::{HasName, Name},
    states::{traits::State, COMPOSITE_STATE_CLASS_NAME, STATE_CLASS_NAME},
    strings::{common::AsStr, UncheckedNewStr},
    validation::{
        error::ValidationIssue,
        issues::{ValueMissing, ValueMissingFrom},
    },
};

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Validation Error(s)
// ------------------------------------------------------------------------------------------------

const INITIAL_STATE_NAME: &str = "initial_state";
const FINAL_STATE_NAME: &str = "final_state";

pub fn error_no_initial_state<N: Name>() -> ValidationIssue<N> {
    ValueMissing::new_property(
        N::new_unchecked(INITIAL_STATE_NAME),
        N::new_unchecked(STATE_CLASS_NAME.as_str()),
    )
    .into()
}

pub fn error_no_initial_state_in<S: State>(composite: &S) -> ValidationIssue<<S as HasName>::Name> {
    let name = composite.name().clone();
    ValueMissingFrom::new_property(
        <S as HasName>::Name::new_unchecked(INITIAL_STATE_NAME),
        <S as HasName>::Name::new_unchecked(STATE_CLASS_NAME.as_str()),
        <S as HasName>::Name::new_unchecked(COMPOSITE_STATE_CLASS_NAME.as_str()),
    )
    .in_container_named(name)
    .into()
}

pub fn error_no_final_state<N: Name>() -> ValidationIssue<N> {
    ValueMissing::new_property(
        N::new_unchecked(FINAL_STATE_NAME),
        N::new_unchecked(STATE_CLASS_NAME.as_str()),
    )
    .into()
}

pub fn error_no_final_state_in<S: State>(composite: &S) -> ValidationIssue<<S as HasName>::Name> {
    let name = composite.name().clone();
    ValueMissingFrom::new_property(
        <S as HasName>::Name::new_unchecked(FINAL_STATE_NAME),
        <S as HasName>::Name::new_unchecked(STATE_CLASS_NAME.as_str()),
        <S as HasName>::Name::new_unchecked(COMPOSITE_STATE_CLASS_NAME.as_str()),
    )
    .in_container_named(name)
    .into()
}
