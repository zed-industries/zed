//! Type-based states to represent code generation phases.
//! These states help enforce code generation invariants at compile time.
//!
//! Currently two phases are defined for code generation:
//!
//! * Prologue: responsible of setting up the function's frame.
//! * Emission: emission of Wasm code to machine code.

/// A code generation phase.
pub trait CodeGenPhase {}

/// The prologue phase.
///
/// Its main responsibility is to setup the function's frame, by creating the
/// well known local slots. In this phase, writes to such slots is allowed.
/// After this phase, the frame is considered immutable.
pub struct Prologue;
/// The code emission phase.
///
/// Its main responsibility is to emit Wasm code to machine code.
pub struct Emission;

impl CodeGenPhase for Prologue {}
impl CodeGenPhase for Emission {}
