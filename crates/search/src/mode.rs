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

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Side {
    Left,
    Right,
}

impl SearchMode {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            SearchMode::Text => "Text",
            SearchMode::Semantic => "Semantic",
            SearchMode::Regex => "Regex",
        }
    }

    pub(crate) fn region_id(&self) -> usize {
        match self {
            SearchMode::Text => 3,
            SearchMode::Semantic => 4,
            SearchMode::Regex => 5,
        }
    }

    pub(crate) fn tooltip_text(&self) -> &'static str {
        match self {
            SearchMode::Text => "Activate Text Search",
            SearchMode::Semantic => "Activate Semantic Search",
            SearchMode::Regex => "Activate Regex Search",
        }
    }

    pub(crate) fn activate_action(&self) -> Box<dyn Action> {
        match self {
            SearchMode::Text => Box::new(ActivateTextMode),
            SearchMode::Semantic => Box::new(ActivateSemanticMode),
            SearchMode::Regex => Box::new(ActivateRegexMode),
        }
    }

    pub(crate) fn border_right(&self) -> bool {
        match self {
            SearchMode::Regex => true,
            SearchMode::Text => true,
            SearchMode::Semantic => true,
        }
    }

    pub(crate) fn border_left(&self) -> bool {
        match self {
            SearchMode::Text => true,
            _ => false,
        }
    }

    pub(crate) fn button_side(&self) -> Option<Side> {
        match self {
            SearchMode::Text => Some(Side::Left),
            SearchMode::Semantic => None,
            SearchMode::Regex => Some(Side::Right),
        }
    }
}

pub(crate) fn next_mode(mode: &SearchMode, semantic_enabled: bool) -> SearchMode {
    let next_text_state = if semantic_enabled {
        SearchMode::Semantic
    } else {
        SearchMode::Regex
    };

    match mode {
        SearchMode::Text => next_text_state,
        SearchMode::Semantic => SearchMode::Regex,
        SearchMode::Regex => SearchMode::Text,
    }
}
