use std::{fmt::Debug, hash::Hash};

use gpui::SharedString;

/// Identifies a picker item in a stable way across match updates.
///
/// Implementations should be cheap to clone and compare, as stable IDs
/// are used to preserve manual selections when the picker's matches are updated.
///
/// # Performance Considerations
///
/// - Cloning should be cheap (e.g., using reference-counted strings like `SharedString`)
/// - Equality comparison should avoid allocations
/// - Consider using structured data instead of formatted strings for comparison
pub trait StableId: Clone + Eq + Hash + Debug + Send + 'static {}

/// Unit type implements StableId as a no-op default for delegates that don't need stable IDs
impl StableId for () {}

/// SharedString implements StableId for string-based stable identifiers
impl StableId for SharedString {}
