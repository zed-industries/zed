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
