use agent_skills::{
    AGENTS_DIR_NAME, MAX_SKILL_DESCRIPTION_LEN, MAX_SKILL_FILE_SIZE, SKILL_FILE_NAME,
    SKILLS_DIR_NAME, SkillMetadata, SkillsUpdatedHook, global_skills_dir, parse_skill_file_content,
    slugify_skill_name, validate_description, validate_name,
};
use anyhow::{Context as _, Result, anyhow};
use editor::{CurrentLineHighlight, Editor, EditorElement, EditorEvent, EditorStyle};
use fs::Fs;
use futures::AsyncReadExt;
use gpui::{
    App, Entity, EventEmitter, FocusHandle, Focusable, ScrollHandle, Subscription, Task, TextStyle,
    WeakEntity, WindowHandle, actions,
};
use http_client::{AsyncBody, HttpClient, HttpRequestExt, Request, StatusCode, Url};
use language::{Buffer, language_settings::SoftWrap};
use settings::{ActionSequence, Settings};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use theme_settings::ThemeSettings;
use ui::{Banner, Divider, SwitchField, WithScrollbar, prelude::*};
use ui_input::{ErasedEditorEvent, InputField};
use util::ResultExt;
use workspace::MultiWorkspace;

use crate::{SettingsUiFile, SettingsWindow, all_projects};

actions!(
    skill_creator,
    [SaveSkill, Cancel, FocusNextField, FocusPreviousField,]
);

const URL_FIELD_TAB_INDEX: isize = 1;
const NAME_FIELD_TAB_INDEX: isize = 2;
const DESCRIPTION_FIELD_TAB_INDEX: isize = 3;
const DISABLE_MODEL_INVOCATION_TAB_INDEX: isize = 4;
const BODY_FIELD_TAB_INDEX: isize = 5;
const SAVE_BUTTON_TAB_INDEX: isize = 6;
const URL_IMPORT_DEBOUNCE: Duration = Duration::from_millis(300);
const URL_IMPORT_ERROR_BODY_MAX_LEN: usize = 2048;

#[derive(Clone, Debug, Default)]
pub enum SkillCreatorOpenMode {
    #[default]
    Form,
    Url {
        initial_url: Option<String>,
    },
    Install {
        content: String,
    },
}

pub(crate) enum SkillCreatorEvent {
    Dismissed,
    Saved,
}

#[derive(Clone, Debug)]
enum UrlImportStatus {
    Idle,
    Fetching,
    Error(SharedString),
}

#[derive(Debug)]
struct ImportedSkill {
    name: String,
    description: String,
    body: String,
    disable_model_invocation: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum ScopeChoice {
    Global,
    Project {
        root_name: SharedString,
        abs_path: Arc<std::path::Path>,
    },
}

impl ScopeChoice {
    /// Absolute path of the `.agents/skills` directory this scope writes to.
    fn skills_dir(&self) -> PathBuf {
        match self {
            ScopeChoice::Global => global_skills_dir(),
            ScopeChoice::Project { abs_path, .. } => {
                abs_path.join(AGENTS_DIR_NAME).join(SKILLS_DIR_NAME)
            }
        }
    }
}

fn scope_for_settings_file(
    current_file: &SettingsUiFile,
    original_window: Option<&WindowHandle<MultiWorkspace>>,
    cx: &App,
) -> ScopeChoice {
    if let SettingsUiFile::Project((worktree_id, _)) = current_file {
        for project in all_projects(original_window, cx) {
            if let Some(worktree) = project.read(cx).worktree_for_id(*worktree_id, cx) {
                let worktree = worktree.read(cx);
                return ScopeChoice::Project {
                    root_name: SharedString::from(worktree.root_name_str().to_string()),
                    abs_path: worktree.abs_path(),
                };
            }
        }
    }
    ScopeChoice::Global
}

pub(crate) fn skill_url_from_clipboard(cx: &App) -> Option<String> {
    cx.read_from_clipboard()
        .and_then(|clipboard| clipboard.text())
        .map(|text| text.trim().to_string())
        .filter(|text| is_supported_skill_url(text))
}

/// Renders the skill creator sub-page pushed by
/// [`SettingsWindow::open_skill_creator_sub_page`].
pub(crate) fn render_skill_creator_page(
    settings_window: &SettingsWindow,
    _scroll_handle: &ScrollHandle,
    _window: &mut Window,
    _cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let Some(page) = settings_window.skill_creator_page() else {
        return gpui::Empty.into_any_element();
    };
    page.into_any_element()
}

pub struct SkillCreatorPage {
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    url_editor: Entity<InputField>,
    name_editor: Entity<InputField>,
    description_editor: Entity<InputField>,
    body_editor: Entity<Editor>,
    description_length: usize,
    settings_window: WeakEntity<SettingsWindow>,
    disable_model_invocation: bool,
    name_error: Option<&'static str>,
    description_error: Option<&'static str>,
    body_error: Option<&'static str>,
    save_error: Option<SharedString>,
    url_import_status: UrlImportStatus,
    saving: bool,
    save_task: Option<Task<()>>,
    url_import_debounce_task: Option<Task<()>>,
    url_import_task: Option<Task<()>>,
    scroll_handle: ScrollHandle,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<SkillCreatorEvent> for SkillCreatorPage {}

impl SkillCreatorPage {
    pub(crate) fn new(
        settings_window: WeakEntity<SettingsWindow>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let app_state = workspace::AppState::global(cx);
        let fs = app_state.fs.clone();
        let language_registry = app_state.languages.clone();
        let http_client = cx.http_client();

        let focus_handle = cx.focus_handle();

        let url_editor = cx.new(|cx| {
            InputField::new(
                window,
                cx,
                "https://github.com/owner/repo/blob/main/path/to/SKILL.md",
            )
            .tab_index(URL_FIELD_TAB_INDEX)
            .tab_stop(true)
        });

        let name_editor = cx.new(|cx| {
            InputField::new(window, cx, "my-new-skill")
                .label("Name")
                .tab_index(NAME_FIELD_TAB_INDEX)
                .tab_stop(true)
        });
        // Focus the name field on open.
        window.focus(&name_editor.focus_handle(cx), cx);

        let description_editor = cx.new(|cx| {
            InputField::new(
                window,
                cx,
                "e.g., Fill the PR description following this template.",
            )
            .label("Description")
            .tab_index(DESCRIPTION_FIELD_TAB_INDEX)
            .tab_stop(true)
        });

        let body_editor = cx.new(|cx| {
            let buffer = cx.new(|cx| {
                let buffer = Buffer::local(String::new(), cx);
                buffer.set_language_registry(language_registry.clone());
                buffer
            });
            let mut editor = Editor::for_buffer(buffer, None, window, cx);
            editor.set_placeholder_text("Add skill content…", window, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_use_modal_editing(true);
            editor.set_current_line_highlight(Some(CurrentLineHighlight::None));
            editor
        });

        cx.spawn_in(window, {
            let body_editor = body_editor.downgrade();
            let language_registry = language_registry.clone();
            async move |_this, cx| {
                let markdown = language_registry.language_for_name("Markdown").await.ok();
                if let Some(markdown) = markdown {
                    body_editor
                        .update(cx, |editor, cx| {
                            editor.buffer().update(cx, |multi_buffer, cx| {
                                if let Some(buffer) = multi_buffer.as_singleton() {
                                    buffer.update(cx, |buffer, cx| {
                                        buffer.set_language(Some(markdown), cx)
                                    });
                                }
                            });
                        })
                        .ok();
                }
            }
        })
        .detach();

        let url_input_editor = url_editor.read(cx).editor().clone();
        let name_input_editor = name_editor.read(cx).editor().clone();
        let description_input_editor = description_editor.read(cx).editor().clone();
        let weak = cx.weak_entity();
        let url_subscription = url_input_editor.subscribe(
            Box::new(move |event, window, cx| {
                weak.update(cx, |this, cx| {
                    this.handle_url_input_event(&event, window, cx);
                })
                .ok();
            }),
            window,
            cx,
        );
        let weak = cx.weak_entity();
        let name_subscription = name_input_editor.subscribe(
            Box::new(move |event, window, cx| {
                weak.update(cx, |this, cx| {
                    this.handle_name_input_event(&event, window, cx);
                })
                .ok();
            }),
            window,
            cx,
        );
        let weak = cx.weak_entity();
        let description_subscription = description_input_editor.subscribe(
            Box::new(move |event, window, cx| {
                weak.update(cx, |this, cx| {
                    this.handle_description_input_event(&event, window, cx);
                })
                .ok();
            }),
            window,
            cx,
        );

        let subscriptions = vec![
            url_subscription,
            name_subscription,
            description_subscription,
            cx.subscribe_in(&body_editor, window, Self::handle_body_editor_event),
        ];

        Self {
            focus_handle,
            fs,
            http_client,
            url_editor,
            name_editor,
            description_editor,
            body_editor,
            description_length: 0,
            settings_window,
            disable_model_invocation: false,
            name_error: None,
            description_error: None,
            body_error: None,
            save_error: None,
            url_import_status: UrlImportStatus::Idle,
            saving: false,
            save_task: None,
            url_import_debounce_task: None,
            url_import_task: None,
            scroll_handle: ScrollHandle::new(),
            _subscriptions: subscriptions,
        }
    }

    pub(crate) fn name_editor_focus_handle(&self, cx: &App) -> FocusHandle {
        self.name_editor.focus_handle(cx)
    }

    fn handle_url_input_event(
        &mut self,
        event: &ErasedEditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !matches!(event, ErasedEditorEvent::BufferEdited) {
            return;
        }

        // Convention from `thread_view::handle_title_editor_event` and
        // `agent_panel::handle_terminal_title_editor_event`: programmatic
        // `set_text` is performed while the editor is unfocused, so the
        // focus check filters synthesized `BufferEdited` events out of
        // the user-edit path without needing a one-shot suppression flag.
        if !self.url_editor.focus_handle(cx).is_focused(window) {
            return;
        }

        self.save_error = None;
        self.schedule_url_import(window, cx);
    }

    fn handle_name_input_event(
        &mut self,
        event: &ErasedEditorEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, ErasedEditorEvent::BufferEdited) {
            self.recompute_name_error(cx);
            self.save_error = None;
            cx.notify();
        }
    }

    fn handle_description_input_event(
        &mut self,
        event: &ErasedEditorEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, ErasedEditorEvent::BufferEdited) {
            self.recompute_description_error(cx);
            self.save_error = None;
            cx.notify();
        }
    }

    fn handle_body_editor_event(
        &mut self,
        _: &Entity<Editor>,
        event: &EditorEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, EditorEvent::BufferEdited) {
            self.recompute_body_error(cx);
            self.save_error = None;
            cx.notify();
        }
    }

    fn current_name(&self, cx: &App) -> String {
        self.name_editor.read(cx).text(cx)
    }

    fn current_description(&self, cx: &App) -> String {
        self.description_editor.read(cx).text(cx)
    }

    fn current_body(&self, cx: &App) -> String {
        self.body_editor.read(cx).text(cx)
    }

    fn current_url(&self, cx: &App) -> String {
        self.url_editor.read(cx).text(cx)
    }

    fn recompute_name_error(&mut self, cx: &mut Context<Self>) {
        let name = self.current_name(cx);
        let error = validate_name(&name).err();
        self.name_error = error;
        self.name_editor
            .update(cx, |field, cx| field.set_error(error, cx));
    }

    fn recompute_description_error(&mut self, cx: &mut Context<Self>) {
        let description = self.current_description(cx);
        self.description_length = description.len();
        let error = validate_description(&description).err();
        self.description_error = error;
        self.description_editor
            .update(cx, |field, cx| field.set_error(error, cx));
    }

    fn recompute_body_error(&mut self, cx: &App) {
        let body = self.current_body(cx);
        self.body_error = if body.trim().is_empty() {
            Some("Body is required.")
        } else {
            None
        };
    }

    fn is_valid(&self, cx: &App) -> bool {
        validate_name(&self.current_name(cx)).is_ok()
            && validate_description(&self.current_description(cx)).is_ok()
            && !self.current_body(cx).trim().is_empty()
    }

    pub(crate) fn apply_open_mode(
        &mut self,
        open_mode: SkillCreatorOpenMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match open_mode {
            SkillCreatorOpenMode::Form => {}
            SkillCreatorOpenMode::Url { initial_url } => {
                self.open_url_import(initial_url, window, cx);
            }
            SkillCreatorOpenMode::Install { content } => {
                self.open_install_review(content, window, cx);
            }
        }
    }

    /// Pre-fill the form with a skill supplied inline (from a share link) so
    /// the recipient can review it before saving. Unlike URL import, this
    /// doesn't touch the URL editor or perform any network request.
    fn open_install_review(
        &mut self,
        content: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.url_import_debounce_task = None;
        self.url_import_task = None;
        self.url_import_status = UrlImportStatus::Idle;

        match parse_imported_skill(&content, "") {
            Ok(imported) => self.apply_imported_skill(imported, window, cx),
            Err(err) => {
                self.save_error = Some(SharedString::from(format!(
                    "Couldn't read shared skill: {err}"
                )));
                cx.notify();
            }
        }
    }

    fn open_url_import(
        &mut self,
        initial_url: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.save_error = None;
        self.url_import_debounce_task = None;
        self.url_import_task = None;
        self.url_import_status = UrlImportStatus::Idle;

        let text = initial_url.unwrap_or_default();
        let should_fetch = !text.is_empty();
        let needs_set_text = should_fetch || !self.current_url(cx).is_empty();
        if !needs_set_text {
            // No text to write and nothing to clear: just move focus.
            window.focus(&self.url_editor.focus_handle(cx), cx);
            cx.notify();
            return;
        }

        // Defer so the programmatic `set_text` runs before we move focus
        // to the URL editor. `handle_url_input_event` uses
        // `url_editor.is_focused(window)` to distinguish user edits from
        // programmatic ones, so writing while unfocused is what keeps the
        // synthesized `BufferEdited` from being treated as a user edit.
        let skill_creator = cx.weak_entity();
        let url_editor = self.url_editor.clone();
        window.defer(cx, move |window, cx| {
            url_editor.update(cx, |input, cx| {
                input.set_text(&text, window, cx);
            });
            window.focus(&url_editor.focus_handle(cx), cx);
            if should_fetch {
                skill_creator
                    .update(cx, |this, cx| {
                        this.start_url_import(window, cx);
                    })
                    .log_err();
            }
        });
        cx.notify();
    }

    fn schedule_url_import(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.url_import_debounce_task = None;
        self.url_import_task = None;

        let url = self.current_url(cx).trim().to_string();
        if url.is_empty() {
            self.url_import_status = UrlImportStatus::Idle;
            cx.notify();
            return;
        }

        self.url_import_status = UrlImportStatus::Idle;
        let task = cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(URL_IMPORT_DEBOUNCE).await;
            this.update_in(cx, |this, window, cx| {
                this.start_url_import(window, cx);
            })
            .log_err();
        });
        self.url_import_debounce_task = Some(task);
        cx.notify();
    }

    fn start_url_import(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Cancel any pending debounce so the explicit start supersedes it,
        // instead of racing with a timer that's about to fire.
        self.url_import_debounce_task = None;
        self.url_import_task = None;

        let url = self.current_url(cx).trim().to_string();
        if url.is_empty() {
            self.url_import_status = UrlImportStatus::Idle;
            cx.notify();
            return;
        }

        if let Err(err) = github_raw_url(&url) {
            self.url_import_status = UrlImportStatus::Error(SharedString::from(err.to_string()));
            cx.notify();
            return;
        }

        self.url_import_status = UrlImportStatus::Fetching;
        let http_client = self.http_client.clone();
        let fetch_task = cx.background_spawn(fetch_imported_skill_from_url(http_client, url));
        let task = cx.spawn_in(window, async move |this, cx| {
            let result = fetch_task.await;
            this.update_in(cx, |this, window, cx| {
                this.url_import_debounce_task = None;
                this.url_import_task = None;
                match result {
                    Ok(imported) => {
                        this.apply_imported_skill(imported, window, cx);
                    }
                    Err(err) => {
                        this.url_import_status =
                            UrlImportStatus::Error(SharedString::from(err.to_string()));
                        cx.notify();
                    }
                }
            })
            .log_err();
        });
        self.url_import_task = Some(task);
        cx.notify();
    }

    fn apply_imported_skill(
        &mut self,
        imported: ImportedSkill,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.url_import_status = UrlImportStatus::Idle;
        self.save_error = None;

        let name_editor = self.name_editor.clone();
        let description_editor = self.description_editor.clone();
        let body_editor = self.body_editor.clone();
        let skill_creator = cx.weak_entity();
        window.defer(cx, move |window, cx| {
            name_editor.update(cx, |input, cx| {
                input.set_text(&imported.name, window, cx);
            });
            description_editor.update(cx, |input, cx| {
                input.set_text(&imported.description, window, cx);
            });
            body_editor.update(cx, |editor, cx| {
                editor.set_text(imported.body.clone(), window, cx);
            });
            skill_creator
                .update(cx, |this, cx| {
                    this.disable_model_invocation = imported.disable_model_invocation;
                    this.url_import_status = UrlImportStatus::Idle;
                    this.url_import_debounce_task = None;
                    this.url_import_task = None;
                    this.save_error = None;
                    this.recompute_name_error(cx);
                    this.recompute_description_error(cx);
                    this.recompute_body_error(cx);
                    cx.notify();
                })
                .log_err();
            window.focus(&name_editor.focus_handle(cx), cx);
        });
    }

    fn save_skill(&mut self, _: &SaveSkill, window: &mut Window, cx: &mut Context<Self>) {
        self.recompute_name_error(cx);
        self.recompute_description_error(cx);
        self.recompute_body_error(cx);

        if !self.is_valid(cx) || self.saving {
            cx.notify();
            return;
        }

        // Resolve the scope at save time so the skill is written to whichever
        // settings file is selected at the moment the user clicks Save.
        let scope = self
            .settings_window
            .read_with(cx, |settings_window, cx| {
                scope_for_settings_file(
                    &settings_window.current_file,
                    settings_window.original_window.as_ref(),
                    cx,
                )
            })
            .unwrap_or(ScopeChoice::Global);
        let name = self.current_name(cx);
        let description = self.current_description(cx);
        let body = self.current_body(cx);
        let disable_model_invocation = self.disable_model_invocation;
        let fs = self.fs.clone();

        self.saving = true;
        self.save_error = None;
        cx.notify();

        let task = cx.spawn_in(window, async move |this, cx| {
            let result = write_skill_to_disk(
                fs.as_ref(),
                &scope.skills_dir(),
                &name,
                &description,
                &body,
                disable_model_invocation,
            )
            .await;

            this.update_in(cx, |this, _window, cx| {
                this.saving = false;
                this.save_task = None;
                match result {
                    Ok(_) => {
                        // Rescan skill directories so new skills show up in Settings page right away
                        if let Some(hook) = cx.try_global::<SkillsUpdatedHook>() {
                            let hook = hook.0.clone();
                            hook(cx);
                        }

                        cx.emit(SkillCreatorEvent::Saved);
                    }
                    Err(err) => {
                        this.save_error = Some(SharedString::from(err.to_string()));
                        cx.notify();
                    }
                }
            })
            .log_err();
        });
        self.save_task = Some(task);
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        // Block dismissal while a save is in flight
        if self.saving {
            return;
        }
        cx.emit(SkillCreatorEvent::Dismissed);
    }

    fn toggle_disable_model_invocation(&mut self, cx: &mut Context<Self>) {
        self.disable_model_invocation = !self.disable_model_invocation;
        cx.notify();
    }

    fn render_url_import(&self) -> impl IntoElement {
        v_flex()
            .flex_shrink_0()
            .gap_2()
            .child(
                h_flex()
                    .gap_1()
                    .child(Label::new("Import from URL"))
                    .child(Label::new("(optional)").color(Color::Muted)),
            )
            .child(self.url_editor.clone())
            .child(match &self.url_import_status {
                UrlImportStatus::Idle => Label::new(
                    "Paste a GitHub .md URL to fetch it and fill out the form. \
                     For private files, Zed retries using GITHUB_TOKEN, if set.",
                )
                .size(LabelSize::Small)
                .color(Color::Muted)
                .into_any_element(),
                UrlImportStatus::Fetching => {
                    LoadingLabel::new("Fetching and parsing…").into_any_element()
                }
                UrlImportStatus::Error(error) => h_flex()
                    .gap_1()
                    .child(
                        Icon::new(IconName::XCircle)
                            .size(IconSize::Small)
                            .color(Color::Error),
                    )
                    .child(
                        Label::new(error.clone())
                            .size(LabelSize::Small)
                            .color(Color::Error),
                    )
                    .into_any_element(),
            })
    }

    fn render_form_fields(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("skill-creator-form-fields")
            .flex_grow_1()
            .flex_shrink_0()
            .gap_4()
            .child(
                v_flex()
                    .gap_2()
                    .child(Label::new("Front-matter"))
                    .child(self.name_editor.clone())
                    .child(self.description_editor.clone()),
            )
            .child(self.render_optional_params(cx))
            .child(Divider::horizontal())
            .child(
                v_flex()
                    .flex_grow_1()
                    .flex_shrink_0()
                    .gap_2()
                    .child(Label::new("Skill Content"))
                    .child(self.render_body_field(window, cx))
                    .when_some(self.body_error, |this, error| {
                        this.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
                    }),
            )
    }

    fn render_optional_params(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let toggle_state: ToggleState = self.disable_model_invocation.into();

        SwitchField::new(
            "disable-model-invocation",
            Some("Disable model invocation"),
            Some(
                "Hide this skill from the model's catalog. It can still be invoked via slash command."
                    .into(),
            ),
            toggle_state,
            cx.listener(|this, _state: &ToggleState, _window, cx| {
                this.toggle_disable_model_invocation(cx);
            }),
        )
        .tab_index(DISABLE_MODEL_INVOCATION_TAB_INDEX)
        .into_any_element()
    }

    fn render_body_field(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let theme = cx.theme().clone();

        let has_error = self.body_error.is_some();

        let focus_handle = self
            .body_editor
            .focus_handle(cx)
            .tab_index(BODY_FIELD_TAB_INDEX)
            .tab_stop(true);

        let border_color = if has_error {
            theme.status().error_border
        } else if focus_handle.contains_focused(window, cx) {
            theme.colors().border_focused
        } else {
            theme.colors().border
        };

        div()
            .w_full()
            .flex_1()
            .min_h(px(160.))
            .p_2p5()
            .rounded_md()
            .border_1()
            .border_color(border_color)
            .bg(theme.colors().editor_background)
            .track_focus(&focus_handle)
            .overflow_hidden()
            .child(EditorElement::new(
                &self.body_editor,
                EditorStyle {
                    local_player: theme.players().local(),
                    text: TextStyle {
                        color: theme.colors().text,
                        font_family: settings.buffer_font.family.clone(),
                        font_features: settings.buffer_font.features.clone(),
                        font_size: rems(0.875).into(),
                        font_weight: settings.buffer_font.weight,
                        line_height: relative(settings.buffer_line_height.value()),
                        ..Default::default()
                    },
                    syntax: theme.syntax().clone(),
                    inlay_hints_style: editor::make_inlay_hints_style(cx),
                    edit_prediction_styles: editor::make_suggestion_styles(cx),
                    ..EditorStyle::default()
                },
            ))
    }

    fn render_footer(&self, _window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        let saving = self.saving;
        let main_action = if saving { "Saving…" } else { "Save Skill" };

        v_flex()
            .w_full()
            .py_2p5()
            .px_8()
            .border_t_1()
            .border_color(cx.theme().colors().border_variant.opacity(0.4))
            .when(self.save_error.is_some(), |this| {
                this.gap_2().child(
                    Banner::new()
                        .severity(Severity::Error)
                        .children(self.save_error.clone().map(|err| Label::new(err))),
                )
            })
            .child(
                h_flex().w_full().gap_1().justify_end().child(
                    Button::new("save-skill", main_action)
                        .size(ButtonSize::Medium)
                        .style(ButtonStyle::Outlined)
                        .loading(saving)
                        .tab_index(SAVE_BUTTON_TAB_INDEX)
                        // Call `save_skill` directly instead of dispatching the
                        // `SaveSkill` action: action dispatch follows the focused
                        // element's path, so a dispatched action is silently
                        // dropped whenever focus is outside the creator (e.g.
                        // right after switching the settings file/scope).
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.save_skill(&SaveSkill, window, cx);
                        })),
                ),
            )
    }

    fn focus_next_field(
        &mut self,
        _: &FocusNextField,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus_next(cx);
    }

    fn focus_previous_field(
        &mut self,
        _: &FocusPreviousField,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus_prev(cx);
    }

    fn on_menu_next(&mut self, _: &menu::SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
    }

    fn on_menu_prev(
        &mut self,
        _: &menu::SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus_prev(cx);
    }
}

impl Focusable for SkillCreatorPage {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SkillCreatorPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("skill-creator")
            .key_context("SkillCreator")
            .track_focus(&self.focus_handle)
            .on_action(
                |action_sequence: &ActionSequence, window: &mut Window, cx: &mut App| {
                    for action in &action_sequence.0 {
                        window.dispatch_action(action.boxed_clone(), cx);
                    }
                },
            )
            .on_action(cx.listener(Self::save_skill))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::focus_next_field))
            .on_action(cx.listener(Self::focus_previous_field))
            .on_action(cx.listener(Self::on_menu_next))
            .on_action(cx.listener(Self::on_menu_prev))
            .size_full()
            .overflow_hidden()
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .vertical_scrollbar_for(&self.scroll_handle, window, cx)
                    .child(
                        v_flex()
                            .id("skill-creator-form")
                            .tab_index(0)
                            .tab_group()
                            .tab_stop(false)
                            .size_full()
                            .overflow_y_scroll()
                            .track_scroll(&self.scroll_handle)
                            .gap_4()
                            .px_8()
                            .py_4()
                            .child(self.render_url_import())
                            .child(Divider::horizontal().flex_shrink_0().flex_grow_1())
                            .child(self.render_form_fields(window, cx)),
                    ),
            )
            .child(self.render_footer(window, cx))
    }
}

async fn fetch_imported_skill_from_url(
    http_client: Arc<dyn HttpClient>,
    url: String,
) -> Result<ImportedSkill> {
    let github_token = std::env::var("GITHUB_TOKEN").ok().and_then(|token| {
        let token = token.trim().to_string();
        (!token.is_empty()).then_some(token)
    });
    fetch_imported_skill_from_url_with_github_token(http_client, url, github_token).await
}

async fn fetch_imported_skill_from_url_with_github_token(
    http_client: Arc<dyn HttpClient>,
    url: String,
    github_token: Option<String>,
) -> Result<ImportedSkill> {
    let raw_url = github_raw_url(&url)?;
    let (mut status, mut body) =
        fetch_skill_url(http_client.clone(), raw_url.as_str(), None).await?;

    if status == StatusCode::NOT_FOUND
        && let Some(github_token) = github_token.as_deref()
    {
        (status, body) = fetch_skill_url(http_client, raw_url.as_str(), Some(github_token)).await?;
    }

    if !status.is_success() {
        return Err(github_fetch_error(status, &body));
    }

    if body.len() > MAX_SKILL_FILE_SIZE {
        anyhow::bail!(
            "SKILL.md file exceeds maximum size of {}KB",
            MAX_SKILL_FILE_SIZE / 1024
        );
    }

    let content = String::from_utf8(body).context("GitHub response was not valid UTF-8")?;
    parse_imported_skill(&content, raw_url.as_str())
}

async fn fetch_skill_url(
    http_client: Arc<dyn HttpClient>,
    raw_url: &str,
    github_token: Option<&str>,
) -> Result<(StatusCode, Vec<u8>)> {
    // When sending the GitHub token, don't follow redirects: whether an
    // `Authorization` header survives a cross-origin redirect depends on the
    // underlying `HttpClient` implementation, and a redirect away from
    // raw.githubusercontent.com must never carry the user's token with it.
    // Authenticated raw.githubusercontent.com responses are served directly,
    // so a redirect on that path is unexpected anyway.
    let redirect_policy = if github_token.is_some() {
        http_client::RedirectPolicy::NoFollow
    } else {
        http_client::RedirectPolicy::FollowAll
    };
    let request = Request::get(raw_url)
        .follow_redirects(redirect_policy)
        .when_some(github_token, |builder, token| {
            builder.header("Authorization", format!("Bearer {token}"))
        })
        .body(AsyncBody::default())?;

    let mut response = http_client
        .send(request)
        .await
        .with_context(|| format!("failed to fetch {raw_url}"))?;

    let status = response.status();
    if github_token.is_some() && status.is_redirection() {
        anyhow::bail!(
            "GitHub returned an unexpected redirect ({}) for the authenticated request to {raw_url}",
            status.as_u16()
        );
    }
    let mut body = Vec::new();
    response
        .body_mut()
        .take(MAX_SKILL_FILE_SIZE as u64 + 1)
        .read_to_end(&mut body)
        .await
        .context("failed to read response body")?;

    Ok((status, body))
}

fn github_fetch_error(status: StatusCode, body: &[u8]) -> anyhow::Error {
    let mut message = if status == StatusCode::NOT_FOUND {
        "GitHub returned 404 while fetching the skill; no repository exists at this URL, or it is private"
            .to_string()
    } else {
        format!(
            "GitHub returned {} while fetching the skill",
            status.as_u16()
        )
    };

    let response_text = truncated_response_body_for_error(body);
    if !response_text.is_empty() {
        message.push_str(": ");
        message.push_str(&response_text);
    }

    anyhow!(message)
}

pub(crate) fn is_supported_skill_url(input: &str) -> bool {
    github_raw_url(input).is_ok()
}

fn github_raw_url(input: &str) -> Result<String> {
    let url = Url::parse(input.trim()).context("Enter a valid GitHub URL")?;
    if url.scheme() != "https" {
        anyhow::bail!("GitHub skill URLs must use https://");
    }

    let host = url
        .host_str()
        .ok_or_else(|| anyhow!("Enter a valid GitHub URL"))?;
    let path_segments = url
        .path_segments()
        .ok_or_else(|| anyhow!("Enter a valid GitHub URL"))?
        .collect::<Vec<_>>();

    match host {
        "github.com" => github_blob_raw_url(&path_segments),
        "raw.githubusercontent.com" => {
            ensure_markdown_path(&path_segments)?;
            Ok(url.into())
        }
        _ => anyhow::bail!("Paste a GitHub .md URL"),
    }
}

fn github_blob_raw_url(path_segments: &[&str]) -> Result<String> {
    let [owner, repo, kind, reference, file_path @ ..] = path_segments else {
        anyhow::bail!("Paste a GitHub blob URL that points to a .md file");
    };

    if *kind != "blob" {
        anyhow::bail!("Paste a GitHub blob URL that points to a .md file");
    }

    ensure_markdown_path(file_path)?;
    Ok(format!(
        "https://raw.githubusercontent.com/{owner}/{repo}/{reference}/{}",
        file_path.join("/")
    ))
}

fn ensure_markdown_path(path_segments: &[&str]) -> Result<()> {
    let Some(file_name) = path_segments.last() else {
        anyhow::bail!("Paste a GitHub .md URL");
    };

    if !file_name.to_ascii_lowercase().ends_with(".md") {
        anyhow::bail!("Paste a GitHub URL that points to a .md file");
    }

    Ok(())
}

fn parse_imported_skill(content: &str, source_url: &str) -> Result<ImportedSkill> {
    if content.trim_start().starts_with("---") {
        let (metadata, body) = parse_skill_file_content(content)?;
        return Ok(ImportedSkill {
            name: metadata.name,
            description: metadata.description,
            body: body.trim().to_string(),
            disable_model_invocation: metadata.disable_model_invocation,
        });
    }

    Ok(ImportedSkill {
        name: derived_skill_name_from_url(source_url).unwrap_or_else(|| "imported-skill".into()),
        description: derived_description_from_markdown(content).unwrap_or_default(),
        body: content.trim().to_string(),
        disable_model_invocation: false,
    })
}

fn derived_skill_name_from_url(source_url: &str) -> Option<String> {
    let url = Url::parse(source_url).ok()?;
    let file_name = url.path_segments()?.next_back()?;
    let stem = file_name
        .rsplit_once('.')
        .and_then(|(stem, extension)| extension.eq_ignore_ascii_case("md").then_some(stem))
        .unwrap_or(file_name);
    slugify_skill_name(stem)
}

fn truncated_response_body_for_error(body: &[u8]) -> String {
    let text = String::from_utf8_lossy(body);
    let text = text.trim();
    if text.len() <= URL_IMPORT_ERROR_BODY_MAX_LEN {
        return text.to_string();
    }

    let mut end = URL_IMPORT_ERROR_BODY_MAX_LEN;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}…", text[..end].trim_end())
}

fn derived_description_from_markdown(content: &str) -> Option<String> {
    content.lines().find_map(|line| {
        let line = line.trim();
        if line.is_empty() || line == "---" {
            return None;
        }

        let text = line.trim_start_matches('#').trim();
        if text.is_empty() {
            None
        } else {
            Some(truncate_description(text))
        }
    })
}

fn truncate_description(description: &str) -> String {
    if description.len() <= MAX_SKILL_DESCRIPTION_LEN {
        return description.to_string();
    }

    let mut end = MAX_SKILL_DESCRIPTION_LEN;
    while !description.is_char_boundary(end) {
        end -= 1;
    }
    description[..end].trim().to_string()
}

/// Serialize the SKILL.md file to disk at `<skills_dir>/<name>/SKILL.md`.
///
/// Refuses to overwrite an existing directory at `<skills_dir>/<name>`. The
/// caller surfaces the resulting error to the user, who picks a different
/// name.
async fn write_skill_to_disk(
    fs: &dyn Fs,
    skills_dir: &std::path::Path,
    name: &str,
    description: &str,
    body: &str,
    disable_model_invocation: bool,
) -> Result<PathBuf> {
    let skill_dir = skills_dir.join(name);
    match fs.metadata(&skill_dir).await {
        Ok(Some(metadata)) if metadata.is_dir => {
            anyhow::bail!(
                "A skill named \"{name}\" already exists at {}. Pick a different name.",
                skill_dir.display()
            );
        }
        Ok(Some(_)) => {
            // Something exists at this path, but it isn't a directory — e.g.
            // a stray file the user (or another tool) left there. Without
            // this branch we'd fall through to `create_dir`, which on the
            // real fs returns a generic "File exists" IO error that gives
            // the user no idea what's wrong or how to recover.
            anyhow::bail!(
                "A file (not a skill directory) already exists at {}. \
                 Delete it or pick a different skill name.",
                skill_dir.display()
            );
        }
        Ok(None) => {}
        Err(err) => {
            return Err(err).with_context(|| {
                format!(
                    "failed to check whether {} already exists",
                    skill_dir.display()
                )
            });
        }
    }

    let content = format_skill_file(name, description, body, disable_model_invocation)?;

    fs.create_dir(&skill_dir)
        .await
        .with_context(|| format!("failed to create skill directory {}", skill_dir.display()))?;
    let skill_file_path = skill_dir.join(SKILL_FILE_NAME);
    fs.write(&skill_file_path, content.as_bytes())
        .await
        .with_context(|| format!("failed to write {}", skill_file_path.display()))?;

    Ok(skill_file_path)
}

fn format_skill_file(
    name: &str,
    description: &str,
    body: &str,
    disable_model_invocation: bool,
) -> Result<String> {
    let metadata = SkillMetadata {
        name: name.to_string(),
        description: description.to_string(),
        disable_model_invocation,
    };
    let frontmatter = serde_yaml_ng::to_string(&metadata)
        .context("failed to serialize skill frontmatter as YAML")?;

    let mut content = String::with_capacity(frontmatter.len() + body.len() + 16);
    content.push_str("---\n");
    content.push_str(&frontmatter);
    content.push_str("---\n");
    let trimmed_body = body.trim();
    if !trimmed_body.is_empty() {
        content.push('\n');
        content.push_str(trimmed_body);
        content.push('\n');
    }
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_skills::{SkillSource, parse_skill_frontmatter};
    use fs::FakeFs;
    use std::{
        collections::VecDeque,
        io,
        path::Path,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{self, Poll},
    };

    struct TestHttpClient {
        responses: Mutex<VecDeque<(StatusCode, AsyncBody)>>,
        authorization_headers: Mutex<Vec<Option<String>>>,
        redirect_policies: Mutex<Vec<http_client::RedirectPolicy>>,
    }

    impl TestHttpClient {
        fn new(status: u16, body: AsyncBody) -> Arc<dyn HttpClient> {
            Self::new_sequence(vec![(status, body)])
        }

        fn new_sequence(responses: Vec<(u16, AsyncBody)>) -> Arc<Self> {
            Arc::new(Self {
                responses: Mutex::new(
                    responses
                        .into_iter()
                        .map(|(status, body)| {
                            (
                                StatusCode::from_u16(status)
                                    .expect("test status code should be valid"),
                                body,
                            )
                        })
                        .collect(),
                ),
                authorization_headers: Mutex::new(Vec::new()),
                redirect_policies: Mutex::new(Vec::new()),
            })
        }

        fn authorization_headers(&self) -> Vec<Option<String>> {
            self.authorization_headers
                .lock()
                .expect("authorization header mutex should not be poisoned")
                .clone()
        }

        fn redirect_policies(&self) -> Vec<http_client::RedirectPolicy> {
            self.redirect_policies
                .lock()
                .expect("redirect policy mutex should not be poisoned")
                .clone()
        }
    }

    impl HttpClient for TestHttpClient {
        fn user_agent(&self) -> Option<&http_client::http::HeaderValue> {
            None
        }

        fn proxy(&self) -> Option<&Url> {
            None
        }

        fn send(
            &self,
            req: http_client::Request<AsyncBody>,
        ) -> futures::future::BoxFuture<'static, Result<http_client::Response<AsyncBody>>> {
            let authorization_header = req
                .headers()
                .get("Authorization")
                .and_then(|header| header.to_str().ok())
                .map(ToString::to_string);

            match self.authorization_headers.lock() {
                Ok(mut authorization_headers) => authorization_headers.push(authorization_header),
                Err(_) => {
                    return Box::pin(async {
                        Err(anyhow::anyhow!(
                            "test authorization header mutex was poisoned"
                        ))
                    });
                }
            }

            let redirect_policy = req
                .extensions()
                .get::<http_client::RedirectPolicy>()
                .cloned()
                .unwrap_or_default();
            match self.redirect_policies.lock() {
                Ok(mut redirect_policies) => redirect_policies.push(redirect_policy),
                Err(_) => {
                    return Box::pin(async {
                        Err(anyhow::anyhow!("test redirect policy mutex was poisoned"))
                    });
                }
            }

            let response = match self.responses.lock() {
                Ok(mut responses) => responses.pop_front(),
                Err(_) => {
                    return Box::pin(async {
                        Err(anyhow::anyhow!("test response body mutex was poisoned"))
                    });
                }
            };
            let Some((status, body)) = response else {
                return Box::pin(async {
                    Err(anyhow::anyhow!("test response body was already consumed"))
                });
            };

            Box::pin(async move {
                http_client::Response::builder()
                    .status(status)
                    .body(body)
                    .map_err(anyhow::Error::new)
            })
        }
    }

    struct FailsAfterLimitReader {
        bytes_read: usize,
        limit: usize,
    }

    impl futures::AsyncRead for FailsAfterLimitReader {
        fn poll_read(
            mut self: Pin<&mut Self>,
            _cx: &mut task::Context<'_>,
            buffer: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            if self.bytes_read >= self.limit {
                return Poll::Ready(Err(io::Error::other("read past limit")));
            }

            let byte_count = buffer.len().min(self.limit - self.bytes_read);
            buffer[..byte_count].fill(b'a');
            self.bytes_read += byte_count;
            Poll::Ready(Ok(byte_count))
        }
    }

    // Name and description validation rules are unit-tested in
    // `agent_skills`, which owns `validate_name` / `validate_description`
    // / `MAX_SKILL_DESCRIPTION_LEN`. The tests below cover the skill
    // creator's own surface area: SKILL.md formatting and disk-writing.

    #[test]
    fn format_skill_file_round_trips_through_parser() {
        let content =
            format_skill_file("draft-pr", "Push a draft PR", "Do the thing.", false).unwrap();
        let skill = parse_skill_frontmatter(
            Path::new("/skills/draft-pr/SKILL.md"),
            &content,
            SkillSource::Global,
        )
        .expect("generated frontmatter must round-trip through parse_skill_frontmatter");
        assert_eq!(skill.name, "draft-pr");
        assert_eq!(skill.description, "Push a draft PR");
        assert!(!skill.disable_model_invocation);
    }

    #[test]
    fn format_skill_file_writes_disable_model_invocation_true() {
        let content = format_skill_file("my-skill", "description", "body", true).unwrap();
        assert!(content.contains("disable-model-invocation: true"));
    }

    #[test]
    fn format_skill_file_omits_body_when_empty() {
        let content = format_skill_file("my-skill", "description", "   ", false).unwrap();
        // The trailing closing-delimiter newline is the last byte.
        assert!(content.ends_with("---\n"));
    }

    #[test]
    fn format_skill_file_escapes_yaml_specials_in_description() {
        // serde_yaml_ng must quote/escape descriptions that contain YAML
        // specials so the file round-trips. If we ever swap formatters,
        // this test will catch a regression.
        let tricky = "contains: a colon, # a hash, and a \"quote\"";
        let content = format_skill_file("weird-skill", tricky, "body", false).unwrap();
        let skill = parse_skill_frontmatter(
            Path::new("/skills/weird-skill/SKILL.md"),
            &content,
            SkillSource::Global,
        )
        .expect("YAML-special characters must round-trip");
        assert_eq!(skill.description, tricky);
    }

    #[test]
    fn github_blob_url_converts_to_raw_url() {
        let source_url = "https://github.com/cursor/plugins/blob/3347cbab5b54136f6fba0994c3a01a56f7fb7fca/cursor-team-kit/agents/thermo-nuclear-code-quality-review.md";
        let raw_url = github_raw_url(source_url).expect("GitHub blob URLs should be importable");

        assert_eq!(
            raw_url,
            "https://raw.githubusercontent.com/cursor/plugins/3347cbab5b54136f6fba0994c3a01a56f7fb7fca/cursor-team-kit/agents/thermo-nuclear-code-quality-review.md"
        );
        assert!(is_supported_skill_url(source_url));
        assert!(!is_supported_skill_url(
            "https://example.com/not-a-skill.md"
        ));
    }

    #[test]
    fn derived_skill_name_strips_markdown_extension_case_insensitively() {
        let name = derived_skill_name_from_url(
            "https://raw.githubusercontent.com/owner/repo/main/README.MD",
        )
        .expect("name should be derived from Markdown URL");

        assert_eq!(name, "readme");
    }

    #[test]
    fn parse_imported_skill_reads_frontmatter_and_body() {
        let imported = parse_imported_skill(
            "---\nname: imported-skill\ndescription: Imported from GitHub.\ndisable-model-invocation: true\n---\n\n# Instructions\n\nDo the thing.\n",
            "https://raw.githubusercontent.com/owner/repo/main/imported-skill.md",
        )
        .expect("valid skill frontmatter should parse");

        assert_eq!(imported.name, "imported-skill");
        assert_eq!(imported.description, "Imported from GitHub.");
        assert_eq!(imported.body, "# Instructions\n\nDo the thing.");
        assert!(imported.disable_model_invocation);
    }

    #[test]
    fn parse_imported_skill_falls_back_to_markdown_when_frontmatter_is_missing() {
        let imported = parse_imported_skill(
            "# Code Review\n\nReview code for maintainability.",
            "https://raw.githubusercontent.com/owner/repo/main/code-review.md",
        )
        .expect("plain markdown should still import");

        assert_eq!(imported.name, "code-review");
        assert_eq!(imported.description, "Code Review");
        assert_eq!(
            imported.body,
            "# Code Review\n\nReview code for maintainability."
        );
        assert!(!imported.disable_model_invocation);
    }

    #[test]
    fn parse_imported_skill_reuses_skill_metadata_validation() {
        let error = parse_imported_skill(
            "---\nname: Imported Skill\ndescription: Imported from GitHub.\n---\n\n# Instructions\n",
            "https://raw.githubusercontent.com/owner/repo/main/imported-skill.md",
        )
        .expect_err("invalid skill metadata should be rejected instead of imported");
        let message = error.to_string();

        assert!(
            message.contains("Skill name must contain only lowercase letters"),
            "error should come from shared skill metadata validation, got: {message}"
        );
    }

    #[gpui::test]
    async fn fetch_imported_skill_retries_404_with_github_token(_cx: &mut gpui::TestAppContext) {
        let client = TestHttpClient::new_sequence(vec![
            (404, AsyncBody::from("Not Found")),
            (200, AsyncBody::from("# Imported Skill\n\nDo the thing.")),
        ]);

        let imported = fetch_imported_skill_from_url_with_github_token(
            client.clone(),
            "https://github.com/owner/repo/blob/main/skill.md".to_string(),
            Some("secret-token".to_string()),
        )
        .await
        .expect("private repo fallback should retry with the GitHub token");

        assert_eq!(imported.name, "skill");
        assert_eq!(imported.description, "Imported Skill");
        assert_eq!(
            client.authorization_headers(),
            vec![None, Some("Bearer secret-token".to_string())]
        );
        assert_eq!(
            client.redirect_policies(),
            vec![
                http_client::RedirectPolicy::FollowAll,
                http_client::RedirectPolicy::NoFollow,
            ],
            "the authenticated retry must not follow redirects, so the token \
             can never be forwarded to another host"
        );
    }

    #[gpui::test]
    async fn fetch_imported_skill_rejects_redirect_on_authenticated_request(
        _cx: &mut gpui::TestAppContext,
    ) {
        let client = TestHttpClient::new_sequence(vec![
            (404, AsyncBody::from("Not Found")),
            (302, AsyncBody::from("")),
        ]);

        let error = fetch_imported_skill_from_url_with_github_token(
            client.clone(),
            "https://github.com/owner/repo/blob/main/skill.md".to_string(),
            Some("secret-token".to_string()),
        )
        .await
        .expect_err("a redirect on the authenticated request should be an error");
        let message = error.to_string();

        assert!(
            message.contains("unexpected redirect (302)"),
            "error should report the redirect, got: {message}"
        );
    }

    #[gpui::test]
    async fn fetch_imported_skill_reports_private_or_missing_for_404(
        _cx: &mut gpui::TestAppContext,
    ) {
        let client = TestHttpClient::new_sequence(vec![(404, AsyncBody::from("Not Found"))]);

        let error = fetch_imported_skill_from_url_with_github_token(
            client.clone(),
            "https://github.com/owner/repo/blob/main/skill.md".to_string(),
            None,
        )
        .await
        .expect_err("404 without a GitHub token should fail");
        let message = error.to_string();

        assert!(
            message.contains("no repository exists at this URL, or it is private"),
            "404 error should mention private repositories, got: {message}"
        );
        assert_eq!(client.authorization_headers(), vec![None]);
    }

    #[gpui::test]
    async fn fetch_imported_skill_stops_reading_after_size_limit(_cx: &mut gpui::TestAppContext) {
        let client = TestHttpClient::new(
            200,
            AsyncBody::from_reader(FailsAfterLimitReader {
                bytes_read: 0,
                limit: MAX_SKILL_FILE_SIZE + 1,
            }),
        );

        let error = fetch_imported_skill_from_url(
            client,
            "https://github.com/owner/repo/blob/main/skill.md".to_string(),
        )
        .await
        .expect_err("oversized responses should be rejected");
        let message = error.to_string();

        assert!(
            message.contains("exceeds maximum size"),
            "error should report the skill size limit, got: {message}"
        );
        assert!(
            !message.contains("failed to read response body"),
            "reader should not be polled past the limit, got: {message}"
        );
    }

    #[gpui::test]
    async fn fetch_imported_skill_truncates_error_response_body(_cx: &mut gpui::TestAppContext) {
        let body = format!(
            "{}tail-that-should-not-appear",
            "x".repeat(URL_IMPORT_ERROR_BODY_MAX_LEN + 20)
        );
        let client = TestHttpClient::new(500, AsyncBody::from(body));

        let error = fetch_imported_skill_from_url(
            client,
            "https://github.com/owner/repo/blob/main/skill.md".to_string(),
        )
        .await
        .expect_err("non-success responses should be rejected");
        let message = error.to_string();

        assert!(message.contains("GitHub returned 500"));
        assert!(
            message.ends_with('…'),
            "error body should be visibly truncated, got: {message}"
        );
        assert!(
            !message.contains("tail-that-should-not-appear"),
            "error body should not include the unbounded tail, got: {message}"
        );
    }

    #[gpui::test]
    async fn write_skill_to_disk_creates_directory_and_file(cx: &mut gpui::TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/skills", serde_json::json!({})).await;

        let path = write_skill_to_disk(
            fs.as_ref(),
            Path::new("/skills"),
            "draft-pr",
            "Push a draft PR",
            "Body of the skill.",
            false,
        )
        .await
        .expect("write should succeed");

        assert_eq!(path, Path::new("/skills/draft-pr/SKILL.md"));
        let content = fs.load(&path).await.expect("file should exist");
        let skill = parse_skill_frontmatter(&path, &content, SkillSource::Global)
            .expect("written file should be parseable");
        assert_eq!(skill.name, "draft-pr");
        assert_eq!(skill.description, "Push a draft PR");
    }

    #[gpui::test]
    async fn write_skill_to_disk_refuses_to_overwrite(cx: &mut gpui::TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/skills",
            serde_json::json!({
                "draft-pr": {
                    "SKILL.md": "---\nname: draft-pr\ndescription: existing\n---\nbody\n"
                }
            }),
        )
        .await;

        let err = write_skill_to_disk(
            fs.as_ref(),
            Path::new("/skills"),
            "draft-pr",
            "Push a draft PR",
            "Body of the skill.",
            false,
        )
        .await
        .expect_err("writing over an existing skill must fail");
        assert!(
            err.to_string().contains("already exists"),
            "error message should mention the conflict, got: {err}"
        );
    }

    #[gpui::test]
    async fn write_skill_to_disk_rejects_non_directory_at_skill_path(
        cx: &mut gpui::TestAppContext,
    ) {
        let fs = FakeFs::new(cx.executor());
        // A *file* (not a directory) sitting at `/skills/draft-pr`. With the
        // old `is_dir` check this slipped through and we ended up surfacing
        // the underlying "File exists" OS error.
        fs.insert_tree(
            "/skills",
            serde_json::json!({ "draft-pr": "i am a stray file" }),
        )
        .await;

        let err = write_skill_to_disk(
            fs.as_ref(),
            Path::new("/skills"),
            "draft-pr",
            "Push a draft PR",
            "Body of the skill.",
            false,
        )
        .await
        .expect_err("writing where a file already lives must fail");
        let message = err.to_string();
        assert!(
            message.contains("not a skill directory"),
            "error should explain the conflict is a non-directory, got: {message}"
        );
        // Path separator differs between platforms
        let expected_path = Path::new("/skills").join("draft-pr");
        let expected_path = expected_path.display().to_string();
        assert!(
            message.contains(&expected_path),
            "error should include the conflicting path {expected_path:?}, got: {message}"
        );
    }
}
