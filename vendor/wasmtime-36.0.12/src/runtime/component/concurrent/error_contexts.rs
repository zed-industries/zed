/// Error context reference count local to a given (sub)component
///
/// This reference count is localized to a single (sub)component,
/// rather than the global cross-component count (i.e. that determines
/// when a error context can be completely removed)
#[repr(transparent)]
pub struct LocalErrorContextRefCount(pub(crate) usize);

/// Error context reference count across a [`ComponentInstance`]
///
/// Contrasted to `LocalErrorContextRefCount`, this count is maintained
/// across all sub-components in a given component.
///
/// When this count is zero it is *definitely* safe to remove an error context.
#[repr(transparent)]
pub struct GlobalErrorContextRefCount(pub(crate) usize);
