use gpui::{Action, SharedString};

use crate::{ActivateRegexMode, ActivateSemanticMode, ActivateTextMode};

// TODO: Update the default search mode to get from config
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum SearchMode {
    #[default]
    Text,
    Semantic,
    Regex,
}

impl SearchMode {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            SearchMode::Text => "Text",
            SearchMode::Semantic => "Semantic",
            SearchMode::Regex => "Regex",
        }
    }
    pub(crate) fn tooltip(&self) -> SharedString {
        format!("Activate {} Mode", self.label()).into()
    }
    pub(crate) fn action(&self) -> Box<dyn Action> {
        match self {
            SearchMode::Text => ActivateTextMode.boxed_clone(),
            SearchMode::Semantic => ActivateSemanticMode.boxed_clone(),
            SearchMode::Regex => ActivateRegexMode.boxed_clone(),
        }
    }
}

pub(crate) fn next_mode(mode: &SearchMode, semantic_enabled: bool) -> SearchMode {
    match mode {
        SearchMode::Text => SearchMode::Regex,
        SearchMode::Regex => {
            if semantic_enabled {
                SearchMode::Semantic
            } else {
                SearchMode::Text
            }
        }
        SearchMode::Semantic => SearchMode::Text,
    }
}
