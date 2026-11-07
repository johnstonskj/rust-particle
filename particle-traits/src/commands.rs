//!
//! Provides traits for defining and executing `Command`s with various ownership and borrowing
//! semantics.
//!
//! # Commands and Fn Traits
//!
//! This table summarizes the relationship between the custom command traits and the standard library
//! `Fn` traits, along with their expected parameters. This similarity is not simply an inspiration
//! but provided blanket implementations allows functions to be used as commands directly, including
//! closures.
//!
//! | Trait             | std::op   | Parameters                 |
//! |-------------------|-----------|----------------------------|
//! | `CommandOnce`     | `FnOnce`  | `(self)`                   |
//! | `CommandOnceWith` | `FnOnce`  | `(self, Self::Value)`      |
//! | `Command`         | `Fn`      | `(&self)`                  |
//! | `CommandWith`     | `Fn`      | `(&self, Self::Value)`     |
//! | `CommandMut`      | `FnMut`   | `(&mut self) `             |
//! | `CommandMutWith`  | `FnMut `  | `(&mut self, Self::Value)` |
//!
//! # Example
//!
//! ```rust
//! use particle_traits::commands::CommandOnce;
//!
//! fn do_and_print<C: CommandOnce>(command: C) {
//!     match command.execute_once() {
//!         Ok(_) => println!("Command executed successfully!"),
//!         Err(err) => eprintln!("Command execution failed: {}", err),
//!     }
//! }
//!
//! do_and_print(|| Ok::<&str, std::convert::Infallible>("Hello, world!"));
//! ```
//!

use core::error::Error as CoreError;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Validation
// ------------------------------------------------------------------------------------------------

///
/// The version of the `Command` trait that takes a by-value receiver.
///
/// Instances of `CommandOnce` can be called, but might not be callable multiple times. Because of
/// this, if the only thing known about a type is that it implements `CommandOnce`, it can only be
/// called once. This is analogous to the `FnOnce` trait in the standard library.
///
/// Since both `Command` and `CommandMut` are subtraits of `CommandOnce`, any instance of `Command`
/// or `CommandMut` can be used where a `CommandOnce` is expected. Use `CommandOnce` as a bound when
/// you want to accept a command and only need to call it once. If you need to call the command
/// repeatedly, use `CommandMut` as a bound; if you also need it to not mutate state, use `Command`
/// as a bound.
///
pub trait CommandOnce {
    /// The type returned on successful execution of this command.
    type Output;
    /// The error type returned on failure of this command.
    type Error: CoreError;

    /// Executes this command, consuming self.
    fn execute_once(self) -> Result<Self::Output, Self::Error>;
}

///
/// The version of the `Command` trait that takes a by-value receiver and an additional argument.
///
/// Instances of `CommandOnceWith` can be called with an additional argument, but might not be
/// callable multiple times. See [`CommandOnce`] for more details.
///
pub trait CommandOnceWith {
    /// The type returned on successful execution of this command.
    type Output;
    /// The error type returned on failure of this command.
    type Error: CoreError;
    /// The type of the additional argument passed to this command.
    type Value;

    /// Executes this command, with the provided argument reference, consuming self.
    fn execute_once_with(self, value: &Self::Value) -> Result<Self::Output, Self::Error>;
}

// ------------------------------------------------------------------------------------------------

///
/// The version of the `Command` trait that takes a mutable receiver.
///
/// Instances of `CommandMut` can be called repeatedly and may mutate state. This is analogous to
/// the `FnMut` trait in the standard library.
///
/// Since `CommandOnce` is a supertrait of `CommandMut`, any instance of `CommandMut` can be used
/// where a `CommandOnce` is expected. Use `CommandMut` as a bound when you want to accept a
/// parameter of command-like type and need to call it repeatedly, while allowing it to mutate
/// state. If you don’t want the parameter to mutate state, use `Command` as a bound; if you don’t
/// need to call it repeatedly, use `CommandOnce`.
///
pub trait CommandMut: CommandOnce {
    /// Executes this command, mutably borrowing self.
    fn execute_mut(&mut self) -> Result<Self::Output, Self::Error>;
}

///
/// The version of the `CommandMut` trait that takes a by-value receiver and an additional argument.
///
/// Instances of `CommandMutWith` can be called with an additional argument, and may mutate state.
/// See [`CommandMut`] for more details.
///
pub trait CommandMutWith: CommandOnceWith {
    /// Executes this command, with the provided argument reference, mutably borrowing self.
    fn execute_mut_with(&mut self, value: &Self::Value) -> Result<Self::Output, Self::Error>;
}

// ------------------------------------------------------------------------------------------------

///
/// The version of the `Command` trait that takes an immutable receiver.
///
/// Instances of `Command` can be called repeatedly without mutating state. This is analogous to the
/// `Fn` trait in the standard library.
///
/// Since both `CommandMut` and `CommandOnce` are supertraits of `Command`, any instance of
/// `Command` can be used as a parameter where a `CommandMut` or `CommandOnce` is expected. Use
/// `Command` as a bound when you want to accept a parameter of command-like type and need to call
/// it repeatedly and without mutating state (e.g., when calling it concurrently). If you do not
/// need such strict requirements, use `CommandMut` or `CommandOnce` as bounds.
///
pub trait Command: CommandMut {
    /// Executes this command, immutably borrowing self.
    fn execute(&self) -> Result<Self::Output, Self::Error>;
}

///
/// The version of the `Command` trait that takes a by-value receiver and an additional argument.
///
/// Instances of `CommandWith` can be called with an additional argument, and can be called
/// repeatedly without mutating state. See [`Command`] for more details.
///
pub trait CommandWith: CommandOnceWith {
    /// Executes this command, with the provided argument reference, immutably borrowing self.
    fn execute_with(&self, value: &Self::Value) -> Result<Self::Output, Self::Error>;
}

// ------------------------------------------------------------------------------------------------
// Blanket Implementations for Fn, FnMut, and FnOnce
// ------------------------------------------------------------------------------------------------

impl<F, O, E> CommandOnce for F
where
    E: CoreError,
    F: FnOnce() -> Result<O, E>,
{
    type Output = O;
    type Error = E;

    fn execute_once(self) -> Result<Self::Output, Self::Error> {
        self()
    }
}

// FIXME: impl<F, V, O, E> CommandOnceWith for F
// FIXME: where
// FIXME:     E: CoreError,
// FIXME:     F: FnOnce(V) -> Result<O, E>,
// FIXME: {
// FIXME:     type Output = O;
// FIXME:     type Error = E;
// FIXME:     type Value = V;
// FIXME:
// FIXME:     fn execute_with(self, value: Self::Value) -> Result<Self::Output, Self::Error> {
// FIXME:         self(value)
// FIXME:     }
// FIXME: }

// ------------------------------------------------------------------------------------------------

impl<F, O, E> CommandMut for F
where
    E: CoreError,
    F: FnMut() -> Result<O, E>,
{
    fn execute_mut(&mut self) -> Result<Self::Output, Self::Error> {
        self()
    }
}

// ------------------------------------------------------------------------------------------------

impl<F, O, E> Command for F
where
    E: CoreError,
    F: Fn() -> Result<O, E>,
{
    fn execute(&self) -> Result<Self::Output, Self::Error> {
        self()
    }
}
