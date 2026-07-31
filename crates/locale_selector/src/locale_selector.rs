//! Chooses the language Zed renders its interface in.
//!
//! The choice is stored as the `ui_language` setting; this is the discoverable
//! way to reach it, so that picking a language does not require editing settings
//! by hand.

use std::sync::Arc;

use fs::Fs;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, Render, WeakEntity, Window,
};
use i18n::{LanguageIdentifier, t};
use picker::{Picker, PickerDelegate};
use settings::update_settings_file;
use ui::{ListItem, ListItemSpacing, prelude::*, v_flex};
use util::ResultExt;
use workspace::{ModalView, Workspace, ui::HighlightedLabel, with_active_or_new_workspace};

pub fn init(cx: &mut App) {
    cx.on_action(|_: &zed_actions::locale_selector::Toggle, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            toggle(workspace, window, cx);
        });
    });
}

fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
    let fs = workspace.app_state().fs.clone();
    workspace.toggle_modal(window, cx, |window, cx| {
        let delegate = LocaleSelectorDelegate::new(cx.entity().downgrade(), fs);
        LocaleSelector::new(delegate, window, cx)
    });
}

struct LocaleSelector {
    picker: Entity<Picker<LocaleSelectorDelegate>>,
}

impl LocaleSelector {
    fn new(delegate: LocaleSelectorDelegate, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl EventEmitter<DismissEvent> for LocaleSelector {}

impl Focusable for LocaleSelector {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl ModalView for LocaleSelector {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.revert(cx);
        });
        workspace::DismissDecision::Dismiss(true)
    }
}

impl Render for LocaleSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("LocaleSelector")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

struct LocaleChoice {
    locale: LanguageIdentifier,
    /// What the row shows, and what a query is matched against. It carries both
    /// the language's own name and its tag, so that either "中文" or "zh" finds
    /// Simplified Chinese.
    label: SharedString,
}

struct LocaleSelectorDelegate {
    fs: Arc<dyn Fs>,
    choices: Vec<LocaleChoice>,
    matches: Vec<StringMatch>,
    /// The locale that was active when the picker opened, restored when the user
    /// dismisses without choosing.
    original: LanguageIdentifier,
    selected_index: usize,
    selection_completed: bool,
    selector: WeakEntity<LocaleSelector>,
}

impl LocaleSelectorDelegate {
    fn new(selector: WeakEntity<LocaleSelector>, fs: Arc<dyn Fs>) -> Self {
        let original = i18n::current_locale();
        let choices = i18n::available_locales()
            .into_iter()
            .map(|locale| LocaleChoice {
                label: i18n::display_label(&locale).into(),
                locale,
            })
            .collect::<Vec<_>>();
        let matches = choices
            .iter()
            .enumerate()
            .map(|(index, choice)| StringMatch {
                candidate_id: index,
                score: 0.0,
                positions: Vec::new(),
                string: choice.label.to_string(),
            })
            .collect();
        let selected_index = choices
            .iter()
            .position(|choice| choice.locale == original)
            .unwrap_or_default();

        Self {
            fs,
            choices,
            matches,
            original,
            selected_index,
            selection_completed: false,
            selector,
        }
    }

    fn selected_locale(&self) -> Option<&LanguageIdentifier> {
        let matched = self.matches.get(self.selected_index)?;
        Some(&self.choices.get(matched.candidate_id)?.locale)
    }

    /// Applies the highlighted locale so the interface reads in it while the
    /// picker is open.
    ///
    /// This moves the catalog rather than the setting, so the application menus,
    /// which are rebuilt from a settings observer, keep their previous language
    /// until a choice is confirmed.
    fn preview(&self, cx: &mut App) {
        let Some(locale) = self.selected_locale().cloned() else {
            return;
        };
        if i18n::set_locale(locale) {
            cx.refresh_windows();
        }
    }

    fn revert(&mut self, cx: &mut App) {
        if self.selection_completed {
            return;
        }
        self.selection_completed = true;
        if i18n::set_locale(self.original.clone()) {
            cx.refresh_windows();
        }
    }
}

impl PickerDelegate for LocaleSelectorDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "locale selector"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        t!("Select Interface Language…").resolve().as_ref().into()
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
        self.preview(cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self
            .choices
            .iter()
            .enumerate()
            .map(|(index, choice)| StringMatchCandidate::new(index, &choice.label))
            .collect::<Vec<_>>();

        cx.spawn_in(window, async move |this, cx| {
            let matches = if query.is_empty() {
                candidates
                    .into_iter()
                    .enumerate()
                    .map(|(index, candidate)| StringMatch {
                        candidate_id: index,
                        string: candidate.string,
                        positions: Vec::new(),
                        score: 0.0,
                    })
                    .collect()
            } else {
                match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update(cx, |this, cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
                this.delegate.preview(cx);
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(locale) = self.selected_locale().cloned() else {
            return;
        };
        self.selection_completed = true;
        let tag = locale.to_string();

        telemetry::event!("Settings Changed", setting = "ui_language", value = tag);

        update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings.ui_language = Some(settings::UiLanguage(tag.into()));
        });

        self.selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.revert(cx);
        self.selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let matched = self.matches.get(ix)?;
        let is_original = self.choices.get(matched.candidate_id)?.locale == self.original;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(
                    matched.string.clone(),
                    matched.positions.clone(),
                ))
                .when(is_original, |this| {
                    this.end_slot(Icon::new(IconName::Check).color(Color::Muted))
                }),
        )
    }
}
