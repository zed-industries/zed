use core::fmt;

/// A backtrace starting from the beginning of the thread.
///
/// Backtrace functionality is currently **disabled**. Please review
/// [the feature flags](crate::guide::feature_flags) to enable it.
#[derive(Debug)]
pub struct Backtrace(());

impl crate::GenerateImplicitData for Backtrace {
    fn generate() -> Self {
        Backtrace(())
    }
}

impl crate::AsBacktrace for Backtrace {
    fn as_backtrace(&self) -> Option<&Backtrace> {
        Some(self)
    }
}

impl fmt::Display for Backtrace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "disabled backtrace")
    }
}
