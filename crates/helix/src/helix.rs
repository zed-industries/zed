#![allow(dead_code)]

use anyhow::Result;
use editor::{Editor, EditorEvent, EditorMode};
use gpui::{
    impl_actions, Action, AppContext, EventEmitter, KeyContext, Render, View, ViewContext, WeakView,
};
use language::CursorShape;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_derive::Serialize;
use settings::{Settings, SettingsSources, SettingsStore};
use ui::{IntoElement, VisualContext as _};
use workspace::{Pane, Workspace};

/// Whether or not to enable Helix editing.
///
/// Default: false
pub struct HelixModeSetting(pub bool);

#[derive(Clone, Copy, Deserialize, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Select,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct SwitchMode(pub Mode);

impl_actions!(helix, [SwitchMode]);

/// Initializes the `helix` module.
pub fn init(cx: &mut AppContext) {
    HelixModeSetting::register(cx);
    HelixSettings::register(cx);

    cx.observe_new_views(|editor: &mut Editor, cx| Helix::register(editor, cx))
        .detach();
}

#[derive(Clone)]
pub(crate) struct HelixAddon {
    pub(crate) view: View<Helix>,
}

impl editor::Addon for HelixAddon {
    fn extend_key_context(&self, key_context: &mut KeyContext, cx: &AppContext) {
        self.view.read(cx).extend_key_context(key_context)
    }

    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// The state pertaining to Helix editing.
pub(crate) struct Helix {
    pub(crate) mode: Mode,
    pub last_mode: Mode,
    editor: WeakView<Editor>,
}

impl Render for Helix {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

enum HelixEvent {}
impl EventEmitter<HelixEvent> for Helix {}

impl Helix {
    pub fn new(cx: &mut ViewContext<Editor>) -> View<Self> {
        let editor = cx.view().clone();

        cx.new_view(|cx: &mut ViewContext<Helix>| {
            cx.subscribe(&editor, |helix, _, event, cx| {
                helix.handle_editor_event(event, cx)
            })
            .detach();

            Helix {
                mode: Mode::Normal,
                last_mode: Mode::Normal,
                editor: editor.downgrade(),
            }
        })
    }

    fn register(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        if !editor.use_modal_editing() {
            return;
        }

        let mut was_enabled = Helix::enabled(cx);
        cx.observe_global::<SettingsStore>(move |editor, cx| {
            let enabled = Helix::enabled(cx);
            if was_enabled == enabled {
                return;
            }
            was_enabled = enabled;
            if enabled {
                Self::activate(editor, cx)
            } else {
                Self::deactivate(editor, cx)
            }
        })
        .detach();
        if was_enabled {
            Self::activate(editor, cx)
        }
    }

    fn activate(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        let helix = Helix::new(cx);

        editor.register_addon(HelixAddon {
            view: helix.clone(),
        });

        helix.update(cx, |_, cx| {
            Helix::action(editor, cx, |helix, action: &SwitchMode, cx| {
                helix.switch_mode(action.0, cx)
            });

            cx.defer(|helix, cx| {
                helix.focused(false, cx);
            })
        })
    }

    fn deactivate(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        editor.set_cursor_shape(CursorShape::Bar, cx);
        editor.set_collapse_matches(false);
        editor.set_input_enabled(true);
        editor.set_autoindent(true);
        editor.selections.line_mode = false;
        editor.unregister_addon::<HelixAddon>();
    }

    pub fn action<A: Action>(
        editor: &mut Editor,
        cx: &mut ViewContext<Helix>,
        f: impl Fn(&mut Helix, &A, &mut ViewContext<Helix>) + 'static,
    ) {
        let subscription = editor.register_action(cx.listener(f));
        cx.on_release(|_, _, _| drop(subscription)).detach();
    }

    pub fn editor(&self) -> Option<View<Editor>> {
        self.editor.upgrade()
    }

    pub fn workspace(&self, cx: &ViewContext<Self>) -> Option<View<Workspace>> {
        self.editor().and_then(|editor| editor.read(cx).workspace())
    }

    pub fn pane(&self, cx: &ViewContext<Self>) -> Option<View<Pane>> {
        self.workspace(cx)
            .and_then(|workspace| workspace.read(cx).pane_for(&self.editor()?))
    }

    pub fn enabled(cx: &mut AppContext) -> bool {
        HelixModeSetting::get_global(cx).0
    }

    pub fn switch_mode(&mut self, mode: Mode, cx: &mut ViewContext<Self>) {
        let last_mode = self.mode;
        self.last_mode = last_mode;
        self.mode = mode;

        // Sync editor settings like clip mode
        self.sync_helix_settings(cx);
    }

    pub fn cursor_shape(&self) -> CursorShape {
        match self.mode {
            Mode::Normal => CursorShape::Block,
            Mode::Insert => CursorShape::Bar,
            Mode::Select => CursorShape::Block,
        }
    }

    pub fn editor_input_enabled(&self) -> bool {
        self.mode == Mode::Insert
    }

    pub fn extend_key_context(&self, context: &mut KeyContext) {
        let mode = match self.mode {
            Mode::Normal => "normal",
            Mode::Select => "select",
            Mode::Insert => "insert",
        };

        context.add("Helix");
        context.set("helix_mode", mode);
    }

    fn focused(&mut self, preserve_selection: bool, cx: &mut ViewContext<Self>) {
        let Some(editor) = self.editor() else {
            return;
        };
        let editor = editor.read(cx);
        let editor_mode = editor.mode();
        let newest_selection_empty = editor.selections.newest::<usize>(cx).is_empty();

        if editor_mode == EditorMode::Full
            && !newest_selection_empty
            && self.mode == Mode::Normal
            && editor.leader_peer_id().is_none()
        {
            if preserve_selection {
                self.switch_mode(Mode::Select, cx);
            } else {
                self.update_editor(cx, |_, editor, cx| {
                    editor.change_selections(None, cx, |s| {
                        s.move_with(|_, selection| {
                            selection.collapse_to(selection.start, selection.goal)
                        })
                    });
                });
            }
        }

        self.sync_helix_settings(cx);
    }

    fn handle_editor_event(&mut self, event: &EditorEvent, cx: &mut ViewContext<Self>) {
        match event {
            EditorEvent::Focused => self.focused(true, cx),
            EditorEvent::Blurred => self.blurred(cx),
            EditorEvent::FocusedIn => self.sync_helix_settings(cx),
            _ => {}
        }
    }

    fn update_editor<S>(
        &mut self,
        cx: &mut ViewContext<Self>,
        update: impl FnOnce(&mut Self, &mut Editor, &mut ViewContext<Editor>) -> S,
    ) -> Option<S> {
        let editor = self.editor.upgrade()?;
        Some(editor.update(cx, |editor, cx| update(self, editor, cx)))
    }

    fn sync_helix_settings(&mut self, cx: &mut ViewContext<Self>) {
        self.update_editor(cx, |helix, editor, cx| {
            editor.set_cursor_shape(helix.cursor_shape(), cx);
            editor.set_collapse_matches(true);
            editor.set_input_enabled(helix.editor_input_enabled());
            editor.set_autoindent(true);
            editor.selections.line_mode = false;
            editor.set_inline_completions_enabled(helix.mode == Mode::Insert);
        });
        cx.notify()
    }

    fn blurred(&mut self, cx: &mut ViewContext<Self>) {
        self.update_editor(cx, |_, editor, cx| {
            editor.set_cursor_shape(language::CursorShape::Hollow, cx);
        });
    }
}

impl Settings for HelixModeSetting {
    const KEY: Option<&'static str> = Some("helix_mode");

    type FileContent = Option<bool>;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        Ok(Self(sources.user.copied().flatten().unwrap_or(
            sources.default.ok_or_else(Self::missing_default)?,
        )))
    }
}

#[derive(Deserialize)]
struct HelixSettings {
    // Add any Helix specific settings here
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
struct HelixSettingsContent {
    // Add corresponding optional fields for settings here
}

impl Settings for HelixSettings {
    const KEY: Option<&'static str> = Some("helix");

    type FileContent = HelixSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}
