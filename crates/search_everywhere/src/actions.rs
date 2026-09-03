use command_palette_hooks::CommandPaletteFilter;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{Action, SharedString, Task, Window};

use crate::SearchEverywhereDelegate;
use crate::providers::{SearchResult, SearchResultCategory};

pub struct ActionProvider {
    commands: Vec<Command>,
}

struct Command {
    name: String,
    action: Box<dyn Action>,
}

impl ActionProvider {
    pub fn new<T: 'static>(window: &mut Window, cx: &mut gpui::Context<T>) -> Self {
        let filter = CommandPaletteFilter::try_global(cx);

        let commands = window
            .available_actions(cx)
            .into_iter()
            .filter_map(|action| {
                if filter.is_some_and(|filter| filter.is_hidden(&*action)) {
                    return None;
                }

                Some(Command {
                    name: humanize_action_name(action.name()),
                    action,
                })
            })
            .collect();

        Self { commands }
    }

    pub fn search(
        &self,
        query: &str,
        _window: &mut Window,
        cx: &mut gpui::Context<picker::Picker<SearchEverywhereDelegate>>,
    ) -> Task<Vec<(SearchResult, StringMatch)>> {
        if query.is_empty() {
            return Task::ready(Vec::new());
        }

        let candidates: Vec<StringMatchCandidate> = self
            .commands
            .iter()
            .enumerate()
            .map(|(id, c)| StringMatchCandidate::new(id, &c.name))
            .collect();

        let query = query.to_string();
        let commands: Vec<_> = self
            .commands
            .iter()
            .map(|c| (c.name.clone(), c.action.boxed_clone()))
            .collect();

        cx.spawn(async move |_, cx| {
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                true,
                true,
                100,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;

            matches
                .into_iter()
                .filter_map(|m| {
                    let (name, action) = commands.get(m.candidate_id)?;

                    let result = SearchResult {
                        label: SharedString::from(name.clone()),
                        detail: None,
                        category: SearchResultCategory::Action,
                        path: None,
                        action: Some(action.boxed_clone()),
                        symbol: None,
                        document_symbol: None,
                    };

                    Some((result, m))
                })
                .collect()
        })
    }
}

fn humanize_action_name(name: &str) -> String {
    let capacity = name.len() + name.chars().filter(|c| c.is_uppercase()).count();
    let mut result = String::with_capacity(capacity);

    for char in name.chars() {
        if char == ':' {
            if result.ends_with(':') {
                result.push(' ');
            } else {
                result.push(':');
            }
        } else if char == '_' {
            result.push(' ');
        } else if char.is_uppercase() {
            if !result.ends_with(' ') && !result.ends_with(':') {
                result.push(' ');
            }
            result.extend(char.to_lowercase());
        } else {
            result.push(char);
        }
    }

    let mut title_cased = String::with_capacity(result.len());
    let mut should_capitalize = true;

    for char in result.chars() {
        if should_capitalize && char.is_alphabetic() {
            title_cased.extend(char.to_uppercase());
            should_capitalize = false;
        } else {
            title_cased.push(char);
            if char == ' ' || char == ':' {
                should_capitalize = true;
            }
        }
    }

    title_cased
}
