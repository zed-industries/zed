use std::sync::Arc;

/// The fallback fonts that can be configured for a given font.
#[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FontFallbacks(pub Arc<Vec<String>>);

impl FontFallbacks {
    /// Get the fallback fonts family names
    pub fn fallback_list(&self) -> &[String] {
        &self.0.as_slice()
    }
}
