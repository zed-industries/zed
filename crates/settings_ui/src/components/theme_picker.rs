use std::sync::Arc;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{AnyElement, App, Context, DismissEvent, SharedString, Task, Window};
use picker::{Picker, PickerDelegate};
use theme::ThemeRegistry;
use ui::{ListItem, ListItemSpacing, prelude::*};

type ThemePicker = Picker<ThemePickerDelegate>;

pub struct ThemePickerDelegate {
    themes: Vec<SharedString>,
    filtered_themes: Vec<StringMatch>,
    selected_index: usize,
    current_theme: SharedString,
    on_theme_changed: Arc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>,
}

impl ThemePickerDelegate {
    fn new(
        current_theme: SharedString,
        on_theme_changed: impl Fn(SharedString, &mut Window, &mut App) + 'static,
        cx: &mut Context<ThemePicker>,
    ) -> Self {
        let theme_registry = ThemeRegistry::global(cx);

        let themes = theme_registry.list_names();
        let selected_index = themes
            .iter()
            .position(|theme| *theme == current_theme)
            .unwrap_or(0);

        let filtered_themes = themes
            .iter()
            .enumerate()
            .map(|(index, theme)| StringMatch {
                candidate_id: index,
                string: theme.to_string(),
                positions: Vec::new(),
                score: 0.0,
            })
            .collect();

        Self {
            themes,
            filtered_themes,
            selected_index,
            current_theme,
            on_theme_changed: Arc::new(on_theme_changed),
        }
    }
}

impl PickerDelegate for ThemePickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.filtered_themes.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<ThemePicker>) {
        self.selected_index = ix.min(self.filtered_themes.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search theme…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<ThemePicker>,
    ) -> Task<()> {
        let themes = self.themes.clone();
        let current_theme = self.current_theme.clone();

        let matches: Vec<StringMatch> = if query.is_empty() {
            themes
                .iter()
                .enumerate()
                .map(|(index, theme)| StringMatch {
                    candidate_id: index,
                    string: theme.to_string(),
                    positions: Vec::new(),
                    score: 0.0,
                })
                .collect()
        } else {
            let _candidates: Vec<StringMatchCandidate> = themes
                .iter()
                .enumerate()
                .map(|(id, theme)| StringMatchCandidate::new(id, theme.as_ref()))
                .collect();

            themes
                .iter()
                .enumerate()
                .filter(|(_, theme)| theme.to_lowercase().contains(&query.to_lowercase()))
                .map(|(index, theme)| StringMatch {
                    candidate_id: index,
                    string: theme.to_string(),
                    positions: Vec::new(),
                    score: 0.0,
                })
                .collect()
        };

        let selected_index = if query.is_empty() {
            themes
                .iter()
                .position(|theme| *theme == current_theme)
                .unwrap_or(0)
        } else {
            matches
                .iter()
                .position(|m| themes[m.candidate_id] == current_theme)
                .unwrap_or(0)
        };

        self.filtered_themes = matches;
        self.selected_index = selected_index;
        cx.notify();

        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<ThemePicker>) {
        if let Some(theme_match) = self.filtered_themes.get(self.selected_index) {
            let theme = theme_match.string.clone();
            (self.on_theme_changed)(theme.into(), window, cx);
        }
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<ThemePicker>) {
        cx.defer_in(window, |picker, window, cx| {
            picker.set_query("", window, cx);
        });
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<ThemePicker>,
    ) -> Option<Self::ListItem> {
        let theme_match = self.filtered_themes.get(ix)?;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(Label::new(theme_match.string.clone()))
                .into_any_element(),
        )
    }
}

pub fn theme_picker(
    current_theme: SharedString,
    on_theme_changed: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    window: &mut Window,
    cx: &mut Context<ThemePicker>,
) -> ThemePicker {
    let delegate = ThemePickerDelegate::new(current_theme, on_theme_changed, cx);

    Picker::uniform_list(delegate, window, cx)
        .show_scrollbar(true)
        .width(rems_from_px(210.))
        .max_height(Some(rems(18.).into()))
}
