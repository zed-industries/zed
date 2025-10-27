use std::sync::Arc;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{AnyElement, App, Context, DismissEvent, SharedString, Task, Window};
use picker::{Picker, PickerDelegate};
use theme::ThemeRegistry;
use ui::{ListItem, ListItemSpacing, prelude::*};

type IconThemePicker = Picker<IconThemePickerDelegate>;

pub struct IconThemePickerDelegate {
    icon_themes: Vec<SharedString>,
    filtered_themes: Vec<StringMatch>,
    selected_index: usize,
    current_theme: SharedString,
    on_theme_changed: Arc<dyn Fn(SharedString, &mut App) + 'static>,
}

impl IconThemePickerDelegate {
    fn new(
        current_theme: SharedString,
        on_theme_changed: impl Fn(SharedString, &mut App) + 'static,
        cx: &mut Context<IconThemePicker>,
    ) -> Self {
        let theme_registry = ThemeRegistry::global(cx);

        let icon_themes: Vec<SharedString> = theme_registry
            .list_icon_themes()
            .into_iter()
            .map(|theme_meta| theme_meta.name)
            .collect();

        let selected_index = icon_themes
            .iter()
            .position(|icon_themes| *icon_themes == current_theme)
            .unwrap_or(0);

        let filtered_themes = icon_themes
            .iter()
            .enumerate()
            .map(|(index, icon_themes)| StringMatch {
                candidate_id: index,
                string: icon_themes.to_string(),
                positions: Vec::new(),
                score: 0.0,
            })
            .collect();

        Self {
            icon_themes,
            filtered_themes,
            selected_index,
            current_theme,
            on_theme_changed: Arc::new(on_theme_changed),
        }
    }
}

impl PickerDelegate for IconThemePickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.filtered_themes.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<IconThemePicker>) {
        self.selected_index = ix.min(self.filtered_themes.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search icon theme…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<IconThemePicker>,
    ) -> Task<()> {
        let icon_themes = self.icon_themes.clone();
        let current_theme = self.current_theme.clone();

        let matches: Vec<StringMatch> = if query.is_empty() {
            icon_themes
                .iter()
                .enumerate()
                .map(|(index, icon_theme)| StringMatch {
                    candidate_id: index,
                    string: icon_theme.to_string(),
                    positions: Vec::new(),
                    score: 0.0,
                })
                .collect()
        } else {
            let _candidates: Vec<StringMatchCandidate> = icon_themes
                .iter()
                .enumerate()
                .map(|(id, icon_theme)| StringMatchCandidate::new(id, icon_theme.as_ref()))
                .collect();

            icon_themes
                .iter()
                .enumerate()
                .filter(|(_, icon_theme)| icon_theme.to_lowercase().contains(&query.to_lowercase()))
                .map(|(index, icon_theme)| StringMatch {
                    candidate_id: index,
                    string: icon_theme.to_string(),
                    positions: Vec::new(),
                    score: 0.0,
                })
                .collect()
        };

        let selected_index = if query.is_empty() {
            icon_themes
                .iter()
                .position(|icon_theme| *icon_theme == current_theme)
                .unwrap_or(0)
        } else {
            matches
                .iter()
                .position(|m| icon_themes[m.candidate_id] == current_theme)
                .unwrap_or(0)
        };

        self.filtered_themes = matches;
        self.selected_index = selected_index;
        cx.notify();

        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        _window: &mut Window,
        cx: &mut Context<IconThemePicker>,
    ) {
        if let Some(theme_match) = self.filtered_themes.get(self.selected_index) {
            let theme = theme_match.string.clone();
            (self.on_theme_changed)(theme.into(), cx);
        }
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<IconThemePicker>) {
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
        _cx: &mut Context<IconThemePicker>,
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

pub fn icon_theme_picker(
    current_theme: SharedString,
    on_theme_changed: impl Fn(SharedString, &mut App) + 'static,
    window: &mut Window,
    cx: &mut Context<IconThemePicker>,
) -> IconThemePicker {
    let delegate = IconThemePickerDelegate::new(current_theme, on_theme_changed, cx);

    Picker::uniform_list(delegate, window, cx)
        .show_scrollbar(true)
        .width(rems_from_px(210.))
        .max_height(Some(rems(18.).into()))
}
