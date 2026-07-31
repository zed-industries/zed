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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Entity, TestAppContext, VisualTestContext};
    use project::Project;
    use serde_json::json;
    use util::path;
    use workspace::MultiWorkspace;

    const ZH_CN: &str = "zh-CN";
    const EN_US: &str = "en-US";

    /// Loads a catalog that names itself and nothing else, so that switching to it
    /// leaves every other key falling back to English.
    fn load_chinese_catalog() {
        i18n::reset();
        i18n::add_ftl(
            &ZH_CN.parse().expect("locale"),
            "locale-display-name = 简体中文\n".to_owned(),
        )
        .expect("catalog parses");
        i18n::set_locale(EN_US.parse().expect("locale"));
    }

    /// Everything that has to await, kept apart from the locale so that a test can
    /// take [`i18n::lock_for_test`] once the awaiting is over and hold it across
    /// synchronous code only.
    async fn setup_workspace(
        cx: &mut TestAppContext,
    ) -> (Entity<workspace::Workspace>, &mut VisualTestContext) {
        let app_state = cx.update(|cx| {
            let app_state = workspace::AppState::test(cx);
            settings::init(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            super::init(cx);
            app_state
        });
        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/test"), json!({}))
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/test").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        (workspace, cx)
    }

    fn open_selector(
        workspace: &Entity<workspace::Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<Picker<LocaleSelectorDelegate>> {
        cx.dispatch_action(zed_actions::locale_selector::Toggle);
        cx.run_until_parked();

        workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<LocaleSelector>(cx)
                .expect("locale selector should be open")
                .read(cx)
                .picker
                .clone()
        })
    }

    fn labels(
        picker: &Entity<Picker<LocaleSelectorDelegate>>,
        cx: &mut VisualTestContext,
    ) -> Vec<String> {
        picker.read_with(cx, |picker, _| {
            picker
                .delegate
                .matches
                .iter()
                .map(|matched| matched.string.clone())
                .collect()
        })
    }

    fn row_for(
        picker: &Entity<Picker<LocaleSelectorDelegate>>,
        tag: &str,
        cx: &mut VisualTestContext,
    ) -> usize {
        picker.read_with(cx, |picker, _| {
            picker
                .delegate
                .matches
                .iter()
                .position(|matched| matched.string.contains(tag))
                .unwrap_or_else(|| panic!("no row for {tag}"))
        })
    }

    #[gpui::test]
    async fn lists_each_language_under_its_own_name(cx: &mut TestAppContext) {
        let (workspace, cx) = setup_workspace(cx).await;

        // Taken after the awaiting is over: the lock serializes tests that change
        // the process-global locale, and everything below runs synchronously.
        let _guard = i18n::lock_for_test();
        load_chinese_catalog();
        let picker = open_selector(&workspace, cx);

        assert_eq!(
            labels(&picker, cx),
            vec!["English (en-US)".to_owned(), "简体中文 (zh-CN)".to_owned()],
            "each language should be listed under the name it gives itself, with its tag"
        );
        assert_eq!(
            picker.read_with(cx, |picker, _| picker.delegate.selected_index),
            row_for(&picker, EN_US, cx),
            "the active language should start out selected"
        );

        i18n::reset();
    }

    #[gpui::test]
    async fn previews_the_highlighted_language_and_reverts_on_dismiss(cx: &mut TestAppContext) {
        let (workspace, cx) = setup_workspace(cx).await;

        // Taken after the awaiting is over: the lock serializes tests that change
        // the process-global locale, and everything below runs synchronously.
        let _guard = i18n::lock_for_test();
        load_chinese_catalog();
        let picker = open_selector(&workspace, cx);
        let chinese = row_for(&picker, ZH_CN, cx);

        picker.update_in(cx, |picker, window, cx| {
            picker.delegate.set_selected_index(chinese, window, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            i18n::current_locale().to_string(),
            ZH_CN,
            "highlighting a language should apply it so the interface can be read"
        );

        picker.update_in(cx, |picker, window, cx| {
            picker.delegate.dismissed(window, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            i18n::current_locale().to_string(),
            EN_US,
            "dismissing without choosing should restore the language that was active"
        );

        i18n::reset();
    }

    #[gpui::test]
    async fn keeps_the_chosen_language_on_confirm(cx: &mut TestAppContext) {
        let (workspace, cx) = setup_workspace(cx).await;

        // Taken after the awaiting is over: the lock serializes tests that change
        // the process-global locale, and everything below runs synchronously.
        let _guard = i18n::lock_for_test();
        load_chinese_catalog();
        let picker = open_selector(&workspace, cx);
        let chinese = row_for(&picker, ZH_CN, cx);

        picker.update_in(cx, |picker, window, cx| {
            picker.delegate.set_selected_index(chinese, window, cx);
            picker.delegate.confirm(false, window, cx);
        });
        cx.run_until_parked();

        // The picker dismisses itself after confirming, and that must not undo the
        // choice the way dismissing without choosing does.
        picker.update_in(cx, |picker, window, cx| {
            picker.delegate.dismissed(window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            i18n::current_locale().to_string(),
            ZH_CN,
            "confirming should keep the chosen language"
        );

        i18n::reset();
    }
}
