pub use std::backtrace::Backtrace;

impl crate::GenerateImplicitData for Backtrace {
    fn generate() -> Self {
        Backtrace::force_capture()
    }
}

impl crate::AsBacktrace for Backtrace {
    fn as_backtrace(&self) -> Option<&Backtrace> {
        Some(self)
    }
}
