use std::sync::Arc;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{AnyElement, App, Context, DismissEvent, SharedString, Task, Window};
use picker::{Picker, PickerDelegate};
use theme::FontFamilyCache;
use ui::{ListItem, ListItemSpacing, prelude::*};

type FontPicker = Picker<FontPickerDelegate>;

pub struct FontPickerDelegate {
    fonts: Vec<SharedString>,
    filtered_fonts: Vec<StringMatch>,
    selected_index: usize,
    current_font: SharedString,
    on_font_changed: Arc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>,
}

impl FontPickerDelegate {
    fn new(
        current_font: SharedString,
        on_font_changed: impl Fn(SharedString, &mut Window, &mut App) + 'static,
        cx: &mut Context<FontPicker>,
    ) -> Self {
        let font_family_cache = FontFamilyCache::global(cx);

        let fonts = font_family_cache
            .try_list_font_families()
            .unwrap_or_else(|| vec![current_font.clone()]);
        let selected_index = fonts
            .iter()
            .position(|font| *font == current_font)
            .unwrap_or(0);

        let filtered_fonts = fonts
            .iter()
            .enumerate()
            .map(|(index, font)| StringMatch {
                candidate_id: index,
                string: font.to_string(),
                positions: Vec::new(),
                score: 0.0,
            })
            .collect();

        Self {
            fonts,
            filtered_fonts,
            selected_index,
            current_font,
            on_font_changed: Arc::new(on_font_changed),
        }
    }
}

impl PickerDelegate for FontPickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.filtered_fonts.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<FontPicker>) {
        self.selected_index = ix.min(self.filtered_fonts.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search fonts…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<FontPicker>,
    ) -> Task<()> {
        let fonts = self.fonts.clone();
        let current_font = self.current_font.clone();

        let matches: Vec<StringMatch> = if query.is_empty() {
            fonts
                .iter()
                .enumerate()
                .map(|(index, font)| StringMatch {
                    candidate_id: index,
                    string: font.to_string(),
                    positions: Vec::new(),
                    score: 0.0,
                })
                .collect()
        } else {
            let _candidates: Vec<StringMatchCandidate> = fonts
                .iter()
                .enumerate()
                .map(|(id, font)| StringMatchCandidate::new(id, font.as_ref()))
                .collect();

            fonts
                .iter()
                .enumerate()
                .filter(|(_, font)| font.to_lowercase().contains(&query.to_lowercase()))
                .map(|(index, font)| StringMatch {
                    candidate_id: index,
                    string: font.to_string(),
                    positions: Vec::new(),
                    score: 0.0,
                })
                .collect()
        };

        let selected_index = if query.is_empty() {
            fonts
                .iter()
                .position(|font| *font == current_font)
                .unwrap_or(0)
        } else {
            matches
                .iter()
                .position(|m| fonts[m.candidate_id] == current_font)
                .unwrap_or(0)
        };

        self.filtered_fonts = matches;
        self.selected_index = selected_index;
        cx.notify();

        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<FontPicker>) {
        if let Some(font_match) = self.filtered_fonts.get(self.selected_index) {
            let font = font_match.string.clone();
            (self.on_font_changed)(font.into(), window, cx);
        }
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<FontPicker>) {
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
        _cx: &mut Context<FontPicker>,
    ) -> Option<Self::ListItem> {
        let font_match = self.filtered_fonts.get(ix)?;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(Label::new(font_match.string.clone()))
                .into_any_element(),
        )
    }
}

pub fn font_picker(
    current_font: SharedString,
    on_font_changed: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    window: &mut Window,
    cx: &mut Context<FontPicker>,
) -> FontPicker {
    let delegate = FontPickerDelegate::new(current_font, on_font_changed, cx);

    Picker::uniform_list(delegate, window, cx)
        .show_scrollbar(true)
        .width(rems_from_px(210.))
        .max_height(Some(rems(18.).into()))
}
