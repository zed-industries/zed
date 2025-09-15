mod ollama;

pub use ollama::*;

#[cfg(any(test, feature = "test-support"))]
pub use ollama::fake;
