use std::sync::Arc;

use fuzzy::StringMatch;
use gpui::{AnyElement, App, Context, DismissEvent, SharedString, Task, Window};
use i18n::t;
use picker::{Picker, PickerDelegate};
use ui::{ListItem, ListItemSpacing, prelude::*};

type LocalePicker = Picker<LocalePickerDelegate>;

/// One language on offer: the tag that names it in settings, and the label it is
/// listed under.
struct LocaleRow {
    tag: SharedString,
    label: SharedString,
}

pub struct LocalePickerDelegate {
    locales: Vec<LocaleRow>,
    filtered: Vec<StringMatch>,
    selected_index: usize,
    current_tag: SharedString,
    on_locale_changed: Arc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>,
}

impl LocalePickerDelegate {
    fn new(
        current_tag: SharedString,
        on_locale_changed: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        let locales = i18n::available_locales()
            .into_iter()
            .map(|locale| LocaleRow {
                tag: locale.to_string().into(),
                label: i18n::display_label(&locale).into(),
            })
            .collect::<Vec<_>>();
        let selected_index = locales
            .iter()
            .position(|row| row.tag == current_tag)
            .unwrap_or(0);
        let filtered = rows_as_matches(&locales, |_| true);

        Self {
            locales,
            filtered,
            selected_index,
            current_tag,
            on_locale_changed: Arc::new(on_locale_changed),
        }
    }
}

fn rows_as_matches(locales: &[LocaleRow], keep: impl Fn(&LocaleRow) -> bool) -> Vec<StringMatch> {
    locales
        .iter()
        .enumerate()
        .filter(|(_, row)| keep(row))
        .map(|(index, row)| StringMatch {
            candidate_id: index,
            string: row.label.to_string(),
            positions: Vec::new(),
            score: 0.0,
        })
        .collect()
}

impl PickerDelegate for LocalePickerDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "locale picker"
    }

    fn match_count(&self) -> usize {
        self.filtered.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<LocalePicker>) {
        self.selected_index = ix.min(self.filtered.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        String::from(t!("Search language…")).into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<LocalePicker>,
    ) -> Task<()> {
        // The label carries both the language's own name and its tag, so one
        // comparison answers a query for either.
        let query = query.to_lowercase();
        self.filtered = rows_as_matches(&self.locales, |row| {
            query.is_empty() || row.label.to_lowercase().contains(&query)
        });

        self.selected_index = self
            .filtered
            .iter()
            .position(|matched| self.locales[matched.candidate_id].tag == self.current_tag)
            .unwrap_or(0);
        cx.notify();

        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<LocalePicker>) {
        let Some(matched) = self.filtered.get(self.selected_index) else {
            return;
        };
        let Some(row) = self.locales.get(matched.candidate_id) else {
            return;
        };
        (self.on_locale_changed)(row.tag.clone(), window, cx);
        // Choosing is the end of the interaction, so the popover closes rather
        // than waiting for a click elsewhere.
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<LocalePicker>) {
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
        _cx: &mut Context<LocalePicker>,
    ) -> Option<Self::ListItem> {
        let matched = self.filtered.get(ix)?;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(Label::new(matched.string.clone()))
                .into_any_element(),
        )
    }
}

pub fn locale_picker(
    current_tag: SharedString,
    on_locale_changed: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    window: &mut Window,
    cx: &mut Context<LocalePicker>,
) -> LocalePicker {
    let delegate = LocalePickerDelegate::new(current_tag, on_locale_changed);

    Picker::uniform_list(delegate, window, cx)
        .show_scrollbar(true)
        .initial_width(rems_from_px(210.))
        .max_height(rems(18.))
        .popover()
}
