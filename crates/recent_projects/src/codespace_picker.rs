use std::sync::Arc;

use gpui::{EventEmitter, SharedString, Task};
use picker::Picker;
use remote::CodespaceConnectionOptions;
use ui::{
    App, Context, HighlightedLabel, Icon, IconName, ListItem, ParentElement, Styled, Toggleable,
    Window, h_flex, v_flex,
};
use util::ResultExt as _;

#[derive(Clone, Debug)]
pub struct CodespaceSelected {
    pub codespace: CodespaceConnectionOptions,
}

#[derive(Clone, Debug)]
pub struct CodespacePickerDismissed;

pub(crate) struct CodespacePickerDelegate {
    selected_index: usize,
    codespaces: Option<Vec<CodespaceConnectionOptions>>,
    matches: Vec<fuzzy_nucleo::StringMatch>,
    query: String,
    fetch_started: bool,
    fetch_error: Option<String>,
}

impl CodespacePickerDelegate {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            codespaces: None,
            matches: Vec::new(),
            query: String::new(),
            fetch_started: false,
            fetch_error: None,
        }
    }

    pub fn selected_codespace(&self) -> Option<CodespaceConnectionOptions> {
        let matched = self.matches.get(self.selected_index)?;
        self.codespaces.as_ref()?.get(matched.candidate_id).cloned()
    }

    fn rebuild_matches(&mut self) {
        use fuzzy_nucleo::StringMatchCandidate;
        use ordered_float::OrderedFloat;

        let Some(codespaces) = &self.codespaces else {
            return;
        };

        let candidates = codespaces
            .iter()
            .enumerate()
            .map(|(id, codespace)| StringMatchCandidate::new(id, codespace.name.clone()))
            .collect::<Vec<_>>();

        let query = self.query.trim_start();
        let case = fuzzy_nucleo::Case::smart_if_uppercase_in(query);
        self.matches = fuzzy_nucleo::match_strings(
            &candidates,
            query,
            case,
            fuzzy_nucleo::LengthPenalty::On,
            100,
        );
        self.matches
            .sort_unstable_by_key(|matched| matched.candidate_id);

        self.selected_index = self
            .matches
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, matched)| OrderedFloat(matched.score))
            .map(|(index, _)| index)
            .unwrap_or(0);
    }
}

impl EventEmitter<CodespaceSelected> for Picker<CodespacePickerDelegate> {}

impl EventEmitter<CodespacePickerDismissed> for Picker<CodespacePickerDelegate> {}

impl picker::PickerDelegate for CodespacePickerDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "codespace-picker"
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        Arc::from("Search GitHub Codespaces")
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.query = query;

        if self.codespaces.is_some() || self.fetch_error.is_some() {
            self.rebuild_matches();
            return Task::ready(());
        }

        if self.fetch_started {
            return Task::ready(());
        }

        self.fetch_started = true;
        cx.spawn_in(window, async move |picker, cx| {
            let result = remote::list_codespaces().await;
            picker
                .update(cx, |picker, cx| {
                    match result {
                        Ok(codespaces) => picker.delegate.codespaces = Some(codespaces),
                        Err(error) => {
                            picker.delegate.fetch_error = Some(format!("{error:#}"));
                            log::error!("failed to list GitHub Codespaces: {error:#}");
                        }
                    }
                    picker.delegate.rebuild_matches();
                    cx.notify();
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(codespace) = self.selected_codespace() {
            cx.emit(CodespaceSelected { codespace });
        }
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(CodespacePickerDismissed);
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        if let Some(error) = &self.fetch_error {
            Some(error.clone().into())
        } else if self.fetch_started && self.codespaces.is_none() {
            Some("Loading GitHub Codespaces...".into())
        } else {
            Some("No GitHub Codespaces found".into())
        }
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let matched = self.matches.get(ix)?;
        let codespace = self.codespaces.as_ref()?.get(matched.candidate_id)?;
        let name = codespace.name.clone();
        let name_len = name.len();
        let positions = matched
            .positions
            .iter()
            .copied()
            .filter(|position| *position < name_len)
            .collect::<Vec<_>>();

        Some(
            ListItem::new(ix)
                .toggle_state(selected)
                .inset(true)
                .spacing(ui::ListItemSpacing::Sparse)
                .child(
                    h_flex()
                        .flex_grow_1()
                        .gap_3()
                        .child(Icon::new(IconName::Server))
                        .child(v_flex().child(HighlightedLabel::new(name, positions))),
                ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn codespace() -> CodespaceConnectionOptions {
        CodespaceConnectionOptions {
            name: "octocat-hello-123".to_string(),
        }
    }

    fn selected_for_query(query: &str) -> Option<CodespaceConnectionOptions> {
        let mut delegate = CodespacePickerDelegate::new();
        delegate.codespaces = Some(vec![codespace()]);
        delegate.query = query.to_string();
        delegate.rebuild_matches();
        delegate.selected_codespace()
    }

    #[test]
    fn matches_codespaces_by_name() {
        assert_eq!(selected_for_query("octocat-hello"), Some(codespace()));
    }
}
