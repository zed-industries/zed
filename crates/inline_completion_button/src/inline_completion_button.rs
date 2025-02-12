use anyhow::Result;
use editor::{
    actions::{ShowEditPrediction, ToggleEditPrediction},
    scroll::Autoscroll,
    Editor,
};
use feature_flags::FeatureFlagAppExt;
use fs::Fs;
use gpui::{
    actions, div, App, AsyncWindowContext, Entity, FocusHandle, Focusable, IntoElement, Render,
    Subscription, WeakEntity,
};
use indoc::indoc;
use language::{
    language_settings::{self, all_language_settings, AllLanguageSettings, EditPredictionProvider},
    File, Language,
};
use regex::Regex;
use settings::{update_settings_file, Settings, SettingsStore};
use std::sync::{Arc, LazyLock};
use ui::{prelude::*, ContextMenu, ContextMenuEntry, PopoverMenuHandle};
use workspace::{create_and_open_local_file, item::ItemHandle, StatusItemView, Workspace};

actions!(edit_prediction, [ToggleMenu]);

pub struct InlineCompletionButton {
    editor_subscription: Option<(Subscription, usize)>,
    editor_enabled: Option<bool>,
    editor_show_predictions: bool,
    editor_focus_handle: Option<FocusHandle>,
    language: Option<Arc<Language>>,
    file: Option<Arc<dyn File>>,
    edit_prediction_provider: Option<Arc<dyn inline_completion::InlineCompletionProviderHandle>>,
    fs: Arc<dyn Fs>,
    popover_menu_handle: PopoverMenuHandle<ContextMenu>,
}

impl Render for InlineCompletionButton {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let all_language_settings = all_language_settings(None, cx);

        match all_language_settings.edit_predictions.provider {
            EditPredictionProvider::None => div(),
        }
    }
}

impl InlineCompletionButton {
    pub fn new(
        fs: Arc<dyn Fs>,
        popover_menu_handle: PopoverMenuHandle<ContextMenu>,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe_global::<SettingsStore>(move |_, cx| cx.notify())
            .detach();

        Self {
            editor_subscription: None,
            editor_enabled: None,
            editor_show_predictions: true,
            editor_focus_handle: None,
            language: None,
            file: None,
            edit_prediction_provider: None,
            popover_menu_handle,
            fs,
        }
    }

    pub fn build_language_settings_menu(&self, mut menu: ContextMenu, cx: &mut App) -> ContextMenu {
        let fs = self.fs.clone();

        menu = menu.header("Show Edit Predictions For");

        if let Some(editor_focus_handle) = self.editor_focus_handle.clone() {
            menu = menu.toggleable_entry(
                "This Buffer",
                self.editor_show_predictions,
                IconPosition::Start,
                Some(Box::new(ToggleEditPrediction)),
                {
                    let editor_focus_handle = editor_focus_handle.clone();
                    move |window, cx| {
                        editor_focus_handle.dispatch_action(&ToggleEditPrediction, window, cx);
                    }
                },
            );
        }

        if let Some(language) = self.language.clone() {
            let fs = fs.clone();
            let language_enabled =
                language_settings::language_settings(Some(language.name()), None, cx)
                    .show_edit_predictions;

            menu = menu.toggleable_entry(
                language.name(),
                language_enabled,
                IconPosition::Start,
                None,
                move |_, cx| {
                    toggle_show_inline_completions_for_language(language.clone(), fs.clone(), cx)
                },
            );
        }

        let settings = AllLanguageSettings::get_global(cx);
        let globally_enabled = settings.show_inline_completions(None, cx);
        menu = menu.toggleable_entry("All Files", globally_enabled, IconPosition::Start, None, {
            let fs = fs.clone();
            move |_, cx| toggle_inline_completions_globally(fs.clone(), cx)
        });
        menu = menu.separator().header("Privacy Settings");

        menu = menu.item(
            ContextMenuEntry::new("Configure Excluded Files")
                .icon(IconName::LockOutlined)
                .icon_color(Color::Muted)
                .documentation_aside(|_| {
                    Label::new(indoc!{"
                        Open your settings to add sensitive paths for which Zed will never predict edits."}).into_any_element()
                })
                .handler(move |window, cx| {
                    if let Some(workspace) = window.root().flatten() {
                        let workspace = workspace.downgrade();
                        window
                            .spawn(cx, |cx| {
                                open_disabled_globs_setting_in_editor(
                                    workspace,
                                    cx,
                                )
                            })
                            .detach_and_log_err(cx);
                    }
                }),
        );

        if !self.editor_enabled.unwrap_or(true) {
            menu = menu.item(
                ContextMenuEntry::new("This file is excluded.")
                    .disabled(true)
                    .icon(IconName::ZedPredictDisabled)
                    .icon_size(IconSize::Small),
            );
        }

        if cx.has_flag::<feature_flags::PredictEditsNonEagerModeFeatureFlag>() {
            let is_eager_preview_enabled = match settings.edit_predictions_mode() {
                language::EditPredictionsMode::Auto => false,
                language::EditPredictionsMode::EagerPreview => true,
            };
            menu = menu.separator().toggleable_entry(
                "Eager Preview Mode",
                is_eager_preview_enabled,
                IconPosition::Start,
                None,
                {
                    let fs = fs.clone();
                    move |_window, cx| {
                        update_settings_file::<AllLanguageSettings>(
                            fs.clone(),
                            cx,
                            move |settings, _cx| {
                                let new_mode = match is_eager_preview_enabled {
                                    true => language::EditPredictionsMode::Auto,
                                    false => language::EditPredictionsMode::EagerPreview,
                                };

                                if let Some(edit_predictions) = settings.edit_predictions.as_mut() {
                                    edit_predictions.mode = new_mode;
                                } else {
                                    settings.edit_predictions =
                                        Some(language_settings::EditPredictionSettingsContent {
                                            mode: new_mode,
                                            ..Default::default()
                                        });
                                }
                            },
                        );
                    }
                },
            );
        }

        if let Some(editor_focus_handle) = self.editor_focus_handle.clone() {
            menu = menu
                .separator()
                .entry(
                    "Predict Edit at Cursor",
                    Some(Box::new(ShowEditPrediction)),
                    {
                        let editor_focus_handle = editor_focus_handle.clone();
                        move |window, cx| {
                            editor_focus_handle.dispatch_action(&ShowEditPrediction, window, cx);
                        }
                    },
                )
                .context(editor_focus_handle);
        }

        menu
    }

    pub fn update_enabled(&mut self, editor: Entity<Editor>, cx: &mut Context<Self>) {
        let editor = editor.read(cx);
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let suggestion_anchor = editor.selections.newest_anchor().start;
        let language = snapshot.language_at(suggestion_anchor);
        let file = snapshot.file_at(suggestion_anchor).cloned();
        self.editor_enabled = {
            let file = file.as_ref();
            Some(
                file.map(|file| {
                    all_language_settings(Some(file), cx)
                        .inline_completions_enabled_for_path(file.path())
                })
                .unwrap_or(true),
            )
        };
        self.editor_show_predictions = editor.edit_predictions_enabled();
        self.edit_prediction_provider = editor.edit_prediction_provider();
        self.language = language.cloned();
        self.file = file;
        self.editor_focus_handle = Some(editor.focus_handle(cx));

        cx.notify();
    }

    pub fn toggle_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.popover_menu_handle.toggle(window, cx);
    }
}

impl StatusItemView for InlineCompletionButton {
    fn set_active_pane_item(
        &mut self,
        item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = item.and_then(|item| item.act_as::<Editor>(cx)) {
            self.editor_subscription = Some((
                cx.observe(&editor, Self::update_enabled),
                editor.entity_id().as_u64() as usize,
            ));
            self.update_enabled(editor, cx);
        } else {
            self.language = None;
            self.editor_subscription = None;
            self.editor_enabled = None;
        }
        cx.notify();
    }
}

async fn open_disabled_globs_setting_in_editor(
    workspace: WeakEntity<Workspace>,
    mut cx: AsyncWindowContext,
) -> Result<()> {
    let settings_editor = workspace
        .update_in(&mut cx, |_, window, cx| {
            create_and_open_local_file(paths::settings_file(), window, cx, || {
                settings::initial_user_settings_content().as_ref().into()
            })
        })?
        .await?
        .downcast::<Editor>()
        .unwrap();

    settings_editor
        .downgrade()
        .update_in(&mut cx, |item, window, cx| {
            let text = item.buffer().read(cx).snapshot(cx).text();

            let settings = cx.global::<SettingsStore>();

            // Ensure that we always have "inline_completions { "disabled_globs": [] }"
            let edits = settings.edits_for_update::<AllLanguageSettings>(&text, |file| {
                file.edit_predictions
                    .get_or_insert_with(Default::default)
                    .disabled_globs
                    .get_or_insert_with(Vec::new);
            });

            if !edits.is_empty() {
                item.edit(edits.iter().cloned(), cx);
            }

            let text = item.buffer().read(cx).snapshot(cx).text();

            static DISABLED_GLOBS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
                Regex::new(r#""disabled_globs":\s*\[\s*(?P<content>(?:.|\n)*?)\s*\]"#).unwrap()
            });
            // Only capture [...]
            let range = DISABLED_GLOBS_REGEX.captures(&text).and_then(|captures| {
                captures
                    .name("content")
                    .map(|inner_match| inner_match.start()..inner_match.end())
            });
            if let Some(range) = range {
                item.change_selections(Some(Autoscroll::newest()), window, cx, |selections| {
                    selections.select_ranges(vec![range]);
                });
            }
        })?;

    anyhow::Ok(())
}

fn toggle_inline_completions_globally(fs: Arc<dyn Fs>, cx: &mut App) {
    let show_edit_predictions = all_language_settings(None, cx).show_inline_completions(None, cx);
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file, _| {
        file.defaults.show_edit_predictions = Some(!show_edit_predictions)
    });
}

fn toggle_show_inline_completions_for_language(
    language: Arc<Language>,
    fs: Arc<dyn Fs>,
    cx: &mut App,
) {
    let show_edit_predictions =
        all_language_settings(None, cx).show_inline_completions(Some(&language), cx);
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file, _| {
        file.languages
            .entry(language.name())
            .or_default()
            .show_edit_predictions = Some(!show_edit_predictions);
    });
}
