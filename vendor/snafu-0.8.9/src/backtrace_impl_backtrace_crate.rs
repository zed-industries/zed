pub use backtrace::Backtrace;

impl crate::GenerateImplicitData for Backtrace {
    fn generate() -> Self {
        Backtrace::new()
    }
}

impl crate::AsBacktrace for Backtrace {
    fn as_backtrace(&self) -> Option<&Backtrace> {
        Some(self)
    }
}
