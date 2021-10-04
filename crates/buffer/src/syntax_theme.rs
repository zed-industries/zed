use std::collections::HashMap;

use crate::HighlightId;
use gpui::fonts::HighlightStyle;
use serde::Deserialize;

pub struct SyntaxTheme {
    pub(crate) highlights: Vec<(String, HighlightStyle)>,
}

impl SyntaxTheme {
    pub fn new(highlights: Vec<(String, HighlightStyle)>) -> Self {
        Self { highlights }
    }

    pub fn highlight_style(&self, id: HighlightId) -> Option<HighlightStyle> {
        self.highlights
            .get(id.0 as usize)
            .map(|entry| entry.1.clone())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn highlight_name(&self, id: HighlightId) -> Option<&str> {
        self.highlights.get(id.0 as usize).map(|e| e.0.as_str())
    }
}

impl<'de> Deserialize<'de> for SyntaxTheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let syntax_data: HashMap<String, HighlightStyle> = Deserialize::deserialize(deserializer)?;

        let mut result = Self::new(Vec::new());
        for (key, style) in syntax_data {
            match result
                .highlights
                .binary_search_by(|(needle, _)| needle.cmp(&key))
            {
                Ok(i) | Err(i) => {
                    result.highlights.insert(i, (key, style));
                }
            }
        }

        Ok(result)
    }
}
