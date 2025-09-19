use crate::{states::traits::State, validation::error::ValidationIssue};

// ------------------------------------------------------------------------------------------------
// Public Functions ❱ Validation Error(s)
// ------------------------------------------------------------------------------------------------

pub fn error_no_initial_state() -> ValidationIssue {
    ValidationIssue::new_error(None, "No initial state found in state-set.")
}

pub fn error_no_initial_state_in<S: State>(composite: &S) -> ValidationIssue {
    let name = composite.name();
    let message = format!("No initial state found in state-set for composite state `{name}`.");
    ValidationIssue::new_error(Some(name.to_string()), &message)
}

pub fn error_no_final_state() -> ValidationIssue {
    ValidationIssue::new_error(None, "No final state found in state-set.")
}

pub fn error_no_final_state_in<S: State>(composite: &S) -> ValidationIssue {
    let name = composite.name();
    let message = format!("No final state found in state-set for composite state `{name}`.");
    ValidationIssue::new_error(Some(name.to_string()), &message)
}
