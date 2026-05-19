use agent_skills::{
    AGENTS_DIR_NAME, GLOBAL_SKILLS_DIR_DISPLAY, MAX_SKILL_DESCRIPTION_LEN, MAX_SKILL_NAME_LEN,
    SKILL_FILE_NAME, SKILLS_DIR_NAME, SkillMetadata, global_skills_dir, validate_description,
    validate_name,
};
use anyhow::{Context as _, Result};
use editor::{CurrentLineHighlight, Editor, EditorElement, EditorEvent, EditorStyle};
use fs::Fs;
use gpui::{
    App, Bounds, DEFAULT_ADDITIONAL_WINDOW_SIZE, Entity, FocusHandle, Focusable, Subscription,
    Task, TextStyle, Tiling, TitlebarOptions, WeakEntity, WindowBounds, WindowHandle,
    WindowOptions, actions, point, size,
};
use language::{Buffer, LanguageRegistry, language_settings::SoftWrap};
use platform_title_bar::PlatformTitleBar;
use release_channel::ReleaseChannel;
use settings::Settings;
use std::path::PathBuf;
use std::sync::Arc;
use theme_settings::ThemeSettings;
use ui::{
    ContextMenu, DropdownMenu, DropdownStyle, Headline, HeadlineSize, Switch, Tooltip, prelude::*,
};
use ui_input::{ErasedEditorEvent, InputField};
use util::ResultExt;
use workspace::{
    Toast, Workspace, WorkspaceSettings, client_side_decorations, notifications::NotificationId,
};
use worktree::WorktreeId;

actions!(
    skill_creator,
    [
        /// Saves the skill being edited to disk.
        SaveSkill,
        /// Closes the skill creator window without saving.
        Cancel,
        /// Moves focus to the next form field, wrapping around.
        FocusNextField,
        /// Moves focus to the previous form field, wrapping around.
        FocusPreviousField,
    ]
);

const NAME_FIELD_TAB_INDEX: isize = 1;
const DESCRIPTION_FIELD_TAB_INDEX: isize = 2;
const SCOPE_FIELD_TAB_INDEX: isize = 3;
const DISABLE_MODEL_INVOCATION_TAB_INDEX: isize = 4;
const BODY_FIELD_TAB_INDEX: isize = 5;

pub fn init(_cx: &mut App) {}

#[derive(Clone, Debug)]
enum ScopeChoice {
    Global,
    Project {
        worktree_id: WorktreeId,
        root_name: SharedString,
        abs_path: Arc<std::path::Path>,
    },
}

impl ScopeChoice {
    fn label(&self) -> SharedString {
        match self {
            ScopeChoice::Global => "Global".into(),
            ScopeChoice::Project { root_name, .. } => root_name.clone(),
        }
    }

    fn key(&self) -> SharedString {
        match self {
            ScopeChoice::Global => "global".into(),
            ScopeChoice::Project { worktree_id, .. } => {
                SharedString::from(format!("project-{}", worktree_id.to_usize()))
            }
        }
    }

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

/// Collect the user-visible worktrees from the originating workspace and
/// turn them into project-scope choices. Returns an empty `Vec` if the
/// workspace can't be read (e.g. it was already dropped).
fn project_scopes_from_workspace(
    workspace: &Option<WeakEntity<Workspace>>,
    cx: &App,
) -> Vec<ScopeChoice> {
    let Some(workspace) = workspace.as_ref().and_then(|w| w.upgrade()) else {
        return Vec::new();
    };
    let workspace = workspace.read(cx);
    let project = workspace.project().read(cx);
    project
        .visible_worktrees(cx)
        .filter_map(|worktree| {
            let worktree = worktree.read(cx);
            if !worktree.is_local() {
                return None;
            }
            Some(ScopeChoice::Project {
                worktree_id: worktree.id(),
                root_name: SharedString::from(worktree.root_name_str().to_string()),
                abs_path: worktree.abs_path(),
            })
        })
        .collect()
}

/// Open the skills library window. If one is already open, brings it to the
/// foreground.
pub fn open_skill_creator(
    workspace: Option<WeakEntity<Workspace>>,
    language_registry: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    cx: &mut App,
) -> Task<Result<WindowHandle<SkillCreator>>> {
    cx.spawn(async move |cx| {
        let existing = cx.update(|cx| {
            let handle = cx
                .windows()
                .into_iter()
                .find_map(|window| window.downcast::<SkillCreator>());
            if let Some(handle) = handle {
                handle
                    .update(cx, |_, window, _| window.activate_window())
                    .ok();
                Some(handle)
            } else {
                None
            }
        });
        if let Some(window) = existing {
            return Ok(window);
        }

        cx.update(|cx| {
            let app_id = ReleaseChannel::global(cx).app_id();
            let bounds = Bounds::centered(None, size(px(720.0), px(720.0)), cx);
            let window_decorations = match std::env::var("ZED_WINDOW_DECORATIONS") {
                Ok(val) if val == "server" => gpui::WindowDecorations::Server,
                Ok(val) if val == "client" => gpui::WindowDecorations::Client,
                _ => match WorkspaceSettings::get_global(cx).window_decorations {
                    settings::WindowDecorations::Server => gpui::WindowDecorations::Server,
                    settings::WindowDecorations::Client => gpui::WindowDecorations::Client,
                },
            };
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("New Skill".into()),
                        appears_transparent: true,
                        traffic_light_position: Some(point(px(12.0), px(12.0))),
                    }),
                    app_id: Some(app_id.to_owned()),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    window_background: cx.theme().window_background_appearance(),
                    window_decorations: Some(window_decorations),
                    window_min_size: Some(DEFAULT_ADDITIONAL_WINDOW_SIZE),
                    kind: gpui::WindowKind::Floating,
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| SkillCreator::new(workspace, language_registry, fs, window, cx))
                },
            )
        })
    })
}

pub struct SkillCreator {
    focus_handle: FocusHandle,
    title_bar: Option<Entity<PlatformTitleBar>>,
    workspace: Option<WeakEntity<Workspace>>,
    fs: Arc<dyn Fs>,
    name_editor: Entity<InputField>,
    description_editor: Entity<InputField>,
    body_editor: Entity<Editor>,
    description_length: usize,
    scopes: Vec<ScopeChoice>,
    selected_scope_key: SharedString,
    disable_model_invocation: bool,
    name_error: Option<&'static str>,
    description_error: Option<&'static str>,
    body_error: Option<&'static str>,
    save_error: Option<SharedString>,
    saving: bool,
    // Held so that dropping the entity (e.g. the window closing) cancels
    // an in-flight save. Detaching the task instead would let
    // `write_skill_to_disk` complete after the UI is gone, silently
    // creating a SKILL.md on disk with no toast and no error feedback.
    save_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl SkillCreator {
    fn new(
        workspace: Option<WeakEntity<Workspace>>,
        language_registry: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let project_scopes = project_scopes_from_workspace(&workspace, cx);

        // Default to first project scope (project-level) when available;
        // otherwise fall back to Global.
        let mut scopes: Vec<ScopeChoice> = Vec::with_capacity(project_scopes.len() + 1);
        scopes.push(ScopeChoice::Global);
        scopes.extend(project_scopes);
        let selected_scope_key = scopes
            .iter()
            .find(|scope| matches!(scope, ScopeChoice::Project { .. }))
            .map(|scope| scope.key())
            .unwrap_or_else(|| ScopeChoice::Global.key());

        let name_editor = cx.new(|cx| {
            InputField::new(window, cx, "Name")
                .tab_index(NAME_FIELD_TAB_INDEX)
                .tab_stop(true)
        });
        // Focus the name field on open. Without this, no element inside
        // the window has focus, so dispatching the `Cancel` action from
        // the Cancel button (which walks the focused element's dispatch
        // path looking for `on_action` handlers) silently does nothing
        // until the user manually clicks into one of the editors. The
        // name editor is also the natural first field to type into.
        window.focus(&name_editor.focus_handle(cx), cx);

        let description_editor = cx.new(|cx| {
            InputField::new(window, cx, "Description")
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
            editor.set_placeholder_text("Body", window, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_use_modal_editing(true);
            // Without this, the editor highlights the cursor's line with
            // a slightly different shade than the surrounding
            // `editor_background`, which on an empty body looks like a
            // stray band behind the "Body" placeholder text.
            editor.set_current_line_highlight(Some(CurrentLineHighlight::None));
            editor
        });

        // Attach Markdown language to the body editor asynchronously, since
        // `language_for_name` returns a Task.
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

        let name_input_editor = name_editor.read(cx).editor().clone();
        let description_input_editor = description_editor.read(cx).editor().clone();
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
            name_subscription,
            description_subscription,
            cx.subscribe_in(&body_editor, window, Self::handle_body_editor_event),
        ];

        Self {
            focus_handle,
            title_bar: if !cfg!(target_os = "macos") {
                Some(cx.new(|cx| PlatformTitleBar::new("skill-creator-title-bar", cx)))
            } else {
                None
            },
            workspace,
            fs,
            name_editor,
            description_editor,
            body_editor,
            description_length: 0,
            scopes,
            selected_scope_key,
            disable_model_invocation: false,
            name_error: None,
            description_error: None,
            body_error: None,
            save_error: None,
            saving: false,
            save_task: None,
            _subscriptions: subscriptions,
        }
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

    fn recompute_name_error(&mut self, cx: &App) {
        let name = self.current_name(cx);
        self.name_error = validate_name(&name).err();
    }

    fn recompute_description_error(&mut self, cx: &App) {
        let description = self.current_description(cx);
        self.description_length = description.len();
        self.description_error = validate_description(&description).err();
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
            && self.selected_scope().is_some()
    }

    fn selected_scope(&self) -> Option<&ScopeChoice> {
        self.scopes
            .iter()
            .find(|scope| scope.key() == self.selected_scope_key)
    }

    fn save_skill(&mut self, _: &SaveSkill, window: &mut Window, cx: &mut Context<Self>) {
        // Surface any field-level errors before attempting to save.
        self.recompute_name_error(cx);
        self.recompute_description_error(cx);
        self.recompute_body_error(cx);
        if !self.is_valid(cx) || self.saving {
            cx.notify();
            return;
        }

        let Some(scope) = self.selected_scope().cloned() else {
            self.save_error = Some("Select a scope to save this skill to.".into());
            cx.notify();
            return;
        };

        let name = self.current_name(cx);
        let description = self.current_description(cx);
        let body = self.current_body(cx);
        let disable_model_invocation = self.disable_model_invocation;
        let fs = self.fs.clone();
        let workspace = self.workspace.clone();
        let scope_label = scope.label();

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

            this.update_in(cx, |this, window, cx| {
                this.saving = false;
                this.save_task = None;
                match result {
                    Ok(path) => {
                        if let Some(workspace) = workspace.as_ref().and_then(|w| w.upgrade()) {
                            workspace.update(cx, |workspace, cx| {
                                workspace.show_toast(
                                    Toast::new(
                                        NotificationId::unique::<SaveSkill>(),
                                        format!(
                                            "Saved skill \"{name}\" to {scope_label} ({})",
                                            path.display()
                                        ),
                                    ),
                                    cx,
                                );
                            });
                        }
                        window.remove_window();
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

    fn cancel(&mut self, _: &Cancel, window: &mut Window, _cx: &mut Context<Self>) {
        // Block dismissal while a save is in flight. Otherwise the
        // detached I/O could complete after the window is gone, leaving
        // a SKILL.md on disk with no success or error feedback. The
        // user can still force-close the window via the platform
        // chrome, in which case dropping `self.save_task` cancels the
        // pending write.
        if self.saving {
            return;
        }
        window.remove_window();
    }

    fn select_scope(&mut self, key: SharedString, cx: &mut Context<Self>) {
        if self.scopes.iter().any(|scope| scope.key() == key) {
            self.selected_scope_key = key;
            self.save_error = None;
            cx.notify();
        }
    }

    fn toggle_disable_model_invocation(&mut self, cx: &mut Context<Self>) {
        self.disable_model_invocation = !self.disable_model_invocation;
        cx.notify();
    }

    fn render_hint(text: SharedString, is_error: bool) -> impl IntoElement {
        Label::new(text).size(LabelSize::Small).color(if is_error {
            Color::Error
        } else {
            Color::Muted
        })
    }

    fn render_name_field(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let hint: SharedString = if let Some(err) = self.name_error {
            err.into()
        } else {
            // The numeric bound is sourced from the validator's constant
            // so that the hint, the validator's error message, and the
            // loader's check all move together when the limit changes.
            format!(
                "Lowercase letters, numbers, and hyphens (1\u{2013}{MAX_SKILL_NAME_LEN} chars). \
                 For example: draft-pr."
            )
            .into()
        };
        let has_error = self.name_error.is_some();
        v_flex()
            .gap_1p5()
            .child(self.name_editor.clone())
            .child(Self::render_hint(hint, has_error))
    }

    fn render_description_field(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let counter_color = if self.description_length > MAX_SKILL_DESCRIPTION_LEN {
            Color::Error
        } else {
            Color::Muted
        };
        let hint: SharedString = if let Some(err) = self.description_error {
            err.into()
        } else {
            "Shown to the agent so it can decide when to use this skill.".into()
        };
        let has_error = self.description_error.is_some();
        v_flex()
            .gap_1p5()
            .child(self.description_editor.clone())
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(Self::render_hint(hint, has_error))
                    .child(
                        Label::new(format!(
                            "{} / {MAX_SKILL_DESCRIPTION_LEN} bytes",
                            self.description_length
                        ))
                        .size(LabelSize::Small)
                        .color(counter_color),
                    ),
            )
    }

    fn render_scope_field(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let scopes = self.scopes.clone();
        let selected = self.selected_scope().cloned();
        let selected_label: SharedString = match selected.as_ref() {
            Some(ScopeChoice::Global) => "Global".into(),
            Some(ScopeChoice::Project { root_name, .. }) => {
                SharedString::from(format!("{root_name} (project)"))
            }
            None => "Select a scope\u{2026}".into(),
        };
        let sep = std::path::MAIN_SEPARATOR;
        let scope_hint: SharedString = match selected.as_ref() {
            Some(ScopeChoice::Global) => SharedString::from(format!(
                "Saved to {GLOBAL_SKILLS_DIR_DISPLAY}{sep}\u{2039}name\u{203A}{sep}{SKILL_FILE_NAME}. \
                 Available across every Zed project."
            )),
            Some(ScopeChoice::Project { root_name, .. }) => SharedString::from(format!(
                "Saved to {root_name}{sep}{AGENTS_DIR_NAME}{sep}{SKILLS_DIR_NAME}{sep}\u{2039}name\u{203A}{sep}{SKILL_FILE_NAME}. \
                 Only available when this project is open."
            )),
            None => "Choose where this skill should live.".into(),
        };
        let weak = cx.weak_entity();

        let menu = ContextMenu::build(window, cx, move |mut menu, _window, _cx| {
            for scope in &scopes {
                let key = scope.key();
                let weak = weak.clone();
                let entry_label: SharedString = match scope {
                    ScopeChoice::Global => "Global".into(),
                    ScopeChoice::Project { root_name, .. } => {
                        SharedString::from(format!("{root_name} (project)"))
                    }
                };
                menu = menu.entry(entry_label, None, move |_window, cx| {
                    weak.update(cx, |this, cx| {
                        this.select_scope(key.clone(), cx);
                    })
                    .log_err();
                });
            }
            menu
        });

        v_flex()
            .gap_1p5()
            .child(
                Label::new("Scope")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                DropdownMenu::new("skill-scope-dropdown", selected_label, menu)
                    .tab_index(SCOPE_FIELD_TAB_INDEX)
                    .style(DropdownStyle::Outlined)
                    .full_width(true),
            )
            .child(Self::render_hint(scope_hint, false))
    }

    fn render_optional_params(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let toggle_state: ToggleState = self.disable_model_invocation.into();
        let theme = cx.theme().clone();
        v_flex()
            .gap_2()
            .child(
                Label::new("Optional Parameters")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                div()
                    .rounded_md()
                    .border_1()
                    .border_color(theme.colors().border)
                    .bg(theme.colors().elevated_surface_background)
                    .px_3()
                    .py_2p5()
                    .child(
                        h_flex()
                            .gap_2p5()
                            .justify_between()
                            .child(
                                v_flex()
                                    .gap_0p5()
                                    .child(Label::new("Disable model invocation"))
                                    .child(
                                        Label::new(
                                            "Hide this skill from the model's catalog. \
                                         It can still be invoked via slash command.",
                                        )
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                    ),
                            )
                            .child(
                                Switch::new("disable-model-invocation", toggle_state)
                                    .tab_index(DISABLE_MODEL_INVOCATION_TAB_INDEX)
                                    .on_click(cx.listener(
                                        |this, _state: &ToggleState, _window, cx| {
                                            this.toggle_disable_model_invocation(cx);
                                        },
                                    )),
                            ),
                    ),
            )
    }

    fn render_body_field(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let theme = cx.theme().clone();
        let hint: SharedString = if let Some(err) = self.body_error {
            err.into()
        } else {
            "The full instructions the agent will follow when this skill is used. Markdown is supported.".into()
        };
        let has_error = self.body_error.is_some();
        let focus_handle = self
            .body_editor
            .focus_handle(cx)
            .tab_index(BODY_FIELD_TAB_INDEX)
            .tab_stop(true);
        let border_color = if has_error {
            theme.status().error_border
        } else {
            theme.colors().border
        };
        // The outer flex column takes the remaining vertical space in
        // the form (siblings above are natural-sized). `min_h_0` lets
        // us shrink below the intrinsic editor size on small windows
        // instead of pushing the action bar out the bottom. The inner
        // editor container then `flex_grow`s into whatever space is
        // left, and `Editor` handles scrolling internally if the body
        // text overflows.
        v_flex()
            .flex_1()
            .min_h_0()
            .gap_1p5()
            .child(
                div()
                    .w_full()
                    .flex_1()
                    .min_h(px(160.))
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .border_1()
                    .border_color(border_color)
                    .bg(theme.colors().editor_background)
                    .track_focus(&focus_handle)
                    .overflow_hidden()
                    .child(EditorElement::new(
                        &self.body_editor,
                        EditorStyle {
                            background: theme.system().transparent,
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
                            status: theme.status().clone(),
                            inlay_hints_style: editor::make_inlay_hints_style(cx),
                            edit_prediction_styles: editor::make_suggestion_styles(cx),
                            ..EditorStyle::default()
                        },
                    )),
            )
            .child(Self::render_hint(hint, has_error))
    }

    fn render_action_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let valid = self.is_valid(cx);
        let saving = self.saving;
        h_flex()
            .w_full()
            .justify_between()
            .gap_3()
            .child(
                div().flex_grow().children(
                    self.save_error
                        .clone()
                        .map(|err| Label::new(err).size(LabelSize::Small).color(Color::Error)),
                ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Button::new("cancel-skill", "Cancel")
                            .style(ButtonStyle::Subtle)
                            .disabled(saving)
                            .on_click(|_, window, cx| {
                                window.dispatch_action(Box::new(Cancel), cx);
                            }),
                    )
                    .child(
                        Button::new(
                            "save-skill",
                            if saving {
                                "Saving\u{2026}"
                            } else {
                                "Save Skill"
                            },
                        )
                        .style(ButtonStyle::Filled)
                        .disabled(!valid || saving)
                        .tooltip(move |_window, cx| {
                            Tooltip::for_action("Save Skill", &SaveSkill, cx)
                        })
                        .on_click(|_, window, cx| {
                            window.dispatch_action(Box::new(SaveSkill), cx);
                        }),
                    ),
            )
    }

    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        // On macOS we use a transparent system title bar; the traffic
        // lights sit at (12, 12) over our content, so the header needs
        // extra leading space to avoid being overlapped by them. On
        // other platforms `client_side_decorations` already provides a
        // chrome row with its own close button above this view, so we
        // use a uniform horizontal padding instead. Either way the
        // platform always provides exactly one way to close the
        // window, so we don't render a redundant close button here.
        let needs_traffic_light_clearance = cfg!(target_os = "macos");
        h_flex()
            .w_full()
            .h(px(56.))
            .px_5()
            .when(needs_traffic_light_clearance, |this| this.pl(px(84.)))
            .border_b_1()
            .border_color(theme.colors().border)
            .bg(theme.colors().title_bar_background)
            .items_center()
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Headline::new("New Skill").size(HeadlineSize::Small))
                    .child(
                        Label::new("Define a reusable instruction the agent can follow on demand.")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
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

    // When focus is on a non-editor tab stop (dropdown button, switch),
    // Tab dispatches the global `menu::SelectNext` rather than our
    // custom `FocusNextField`. Catching it here keeps the cycle moving.
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

impl Focusable for SkillCreator {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SkillCreator {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme_settings::setup_ui_font(window, cx);
        let theme = cx.theme().clone();

        client_side_decorations(
            v_flex()
                .id("skill-creator")
                .key_context("SkillCreator")
                .track_focus(&self.focus_handle)
                .on_action(cx.listener(Self::save_skill))
                .on_action(cx.listener(Self::cancel))
                .on_action(cx.listener(Self::focus_next_field))
                .on_action(cx.listener(Self::focus_previous_field))
                .on_action(cx.listener(Self::on_menu_next))
                .on_action(cx.listener(Self::on_menu_prev))
                .size_full()
                .overflow_hidden()
                .font(ui_font)
                .text_color(theme.colors().text)
                .children(self.title_bar.clone())
                .bg(theme.colors().background)
                .child(self.render_header(cx))
                .child(
                    // The form container itself does NOT scroll. The top
                    // fields render at their natural height; the body
                    // field below has `flex_1 + min_h_0` so it expands
                    // into whatever vertical space is left, and the
                    // markdown editor inside handles its own scrolling.
                    v_flex()
                        .id("skill-creator-form")
                        .tab_index(0)
                        .tab_group()
                        .tab_stop(false)
                        .flex_1()
                        .min_h_0()
                        .gap_5()
                        .px_6()
                        .py_5()
                        .child(self.render_name_field(cx))
                        .child(self.render_description_field(cx))
                        .child(self.render_scope_field(window, cx))
                        .child(self.render_optional_params(cx))
                        .child(self.render_body_field(cx)),
                )
                .child(
                    h_flex()
                        .w_full()
                        .px_6()
                        .py_3()
                        .border_t_1()
                        .border_color(theme.colors().border)
                        .bg(theme.colors().panel_background)
                        .child(self.render_action_bar(cx)),
                ),
            window,
            cx,
            Tiling::default(),
        )
    }
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
    use std::path::Path;

    // Name and description validation rules are unit-tested in
    // `agent_skills`, which owns `validate_name` / `validate_description`
    // / `MAX_SKILL_DESCRIPTION_LEN`. The tests below cover this crate's
    // own surface area: SKILL.md formatting and disk-writing.

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
        assert!(
            message.contains("/skills/draft-pr"),
            "error should include the conflicting path, got: {message}"
        );
    }
}
