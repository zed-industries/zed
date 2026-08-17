//! The set of possible iteration methods for the different iterators.

use crate::cursor::MoveOperation;

/// The trait used to define the way iterators behave.
pub trait IterationMethod {
    /// The internal operation to move the cursor through entries.
    const MOVE_OPERATION: MoveOperation;
}

/// Moves to the next or previous key if there
/// are no more values associated with the current key.
#[derive(Debug, Clone, Copy)]
pub enum MoveThroughDuplicateValues {}

impl IterationMethod for MoveThroughDuplicateValues {
    const MOVE_OPERATION: MoveOperation = MoveOperation::Any;
}

/// Moves between keys and ignores the duplicate values of keys.
#[derive(Debug, Clone, Copy)]
pub enum MoveBetweenKeys {}

impl IterationMethod for MoveBetweenKeys {
    const MOVE_OPERATION: MoveOperation = MoveOperation::NoDup;
}

/// Moves only on the duplicate values of a given key and ignores other keys.
#[derive(Debug, Clone, Copy)]
pub enum MoveOnCurrentKeyDuplicates {}

impl IterationMethod for MoveOnCurrentKeyDuplicates {
    const MOVE_OPERATION: MoveOperation = MoveOperation::Dup;
}
