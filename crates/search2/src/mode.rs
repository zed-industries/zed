use gpui::Action;

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
    pub(crate) fn activate_action(&self) -> Box<dyn Action> {
        match self {
            SearchMode::Text => Box::new(ActivateTextMode),
            SearchMode::Semantic => Box::new(ActivateSemanticMode),
            SearchMode::Regex => Box::new(ActivateRegexMode),
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
