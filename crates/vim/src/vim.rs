//! Vim support for Zed.

#[cfg(test)]
mod test;

mod change_list;
mod command;
mod digraph;
mod helix;
mod indent;
mod insert;
mod mode_indicator;
mod motion;
mod normal;
mod object;
mod replace;
mod rewrap;
mod state;
mod surrounds;
mod visual;

use anyhow::Result;
use collections::HashMap;
use editor::{
    Anchor, Bias, Editor, EditorEvent, EditorSettings, HideMouseCursorOrigin, SelectionEffects,
    ToPoint,
    movement::{self, FindRange},
};
use gpui::{
    Action, App, AppContext, Axis, Context, Entity, EventEmitter, KeyContext, KeystrokeEvent,
    Render, Subscription, Task, WeakEntity, Window, actions,
};
use insert::{NormalBefore, TemporaryNormal};
use language::{CharKind, CursorShape, Point, Selection, SelectionGoal, TransactionId};
pub use mode_indicator::ModeIndicator;
use motion::Motion;
use normal::search::SearchSubmit;
use object::Object;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_derive::Serialize;
use settings::{Settings, SettingsSources, SettingsStore, update_settings_file};
use state::{Mode, Operator, RecordedSelection, SearchState, VimGlobals};
use std::{mem, ops::Range, sync::Arc};
use surrounds::SurroundsType;
use theme::ThemeSettings;
use ui::{IntoElement, SharedString, px};
use vim_mode_setting::HelixModeSetting;
use vim_mode_setting::VimModeSetting;
use workspace::{self, Pane, Workspace};

use crate::state::ReplayableAction;

/// Number is used to manage vim's count. Pushing a digit
/// multiplies the current value by 10 and adds the digit.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
struct Number(usize);

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
struct SelectRegister(String);

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct PushObject {
    around: bool,
}

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct PushFindForward {
    before: bool,
    multiline: bool,
}

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct PushFindBackward {
    after: bool,
    multiline: bool,
}

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct PushSneak {
    first_char: Option<char>,
}

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct PushSneakBackward {
    first_char: Option<char>,
}

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct PushAddSurrounds;

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct PushChangeSurrounds {
    target: Option<Object>,
}

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct PushJump {
    line: bool,
}

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct PushDigraph {
    first_char: Option<char>,
}

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct PushLiteral {
    prefix: Option<String>,
}

actions!(
    vim,
    [
        /// Switches to normal mode.
        SwitchToNormalMode,
        /// Switches to insert mode.
        SwitchToInsertMode,
        /// Switches to replace mode.
        SwitchToReplaceMode,
        /// Switches to visual mode.
        SwitchToVisualMode,
        /// Switches to visual line mode.
        SwitchToVisualLineMode,
        /// Switches to visual block mode.
        SwitchToVisualBlockMode,
        /// Switches to Helix-style normal mode.
        SwitchToHelixNormalMode,
        /// Clears any pending operators.
        ClearOperators,
        /// Clears the exchange register.
        ClearExchange,
        /// Inserts a tab character.
        Tab,
        /// Inserts a newline.
        Enter,
        /// Selects inner text object.
        InnerObject,
        /// Maximizes the current pane.
        MaximizePane,
        /// Opens the default keymap file.
        OpenDefaultKeymap,
        /// Resets all pane sizes to default.
        ResetPaneSizes,
        /// Resizes the pane to the right.
        ResizePaneRight,
        /// Resizes the pane to the left.
        ResizePaneLeft,
        /// Resizes the pane upward.
        ResizePaneUp,
        /// Resizes the pane downward.
        ResizePaneDown,
        /// Starts a change operation.
        PushChange,
        /// Starts a delete operation.
        PushDelete,
        /// Exchanges text regions.
        Exchange,
        /// Starts a yank operation.
        PushYank,
        /// Starts a replace operation.
        PushReplace,
        /// Deletes surrounding characters.
        PushDeleteSurrounds,
        /// Sets a mark at the current position.
        PushMark,
        /// Toggles the marks view.
        ToggleMarksView,
        /// Starts a forced motion.
        PushForcedMotion,
        /// Starts an indent operation.
        PushIndent,
        /// Starts an outdent operation.
        PushOutdent,
        /// Starts an auto-indent operation.
        PushAutoIndent,
        /// Starts a rewrap operation.
        PushRewrap,
        /// Starts a shell command operation.
        PushShellCommand,
        /// Converts to lowercase.
        PushLowercase,
        /// Converts to uppercase.
        PushUppercase,
        /// Toggles case.
        PushOppositeCase,
        /// Applies ROT13 encoding.
        PushRot13,
        /// Applies ROT47 encoding.
        PushRot47,
        /// Toggles the registers view.
        ToggleRegistersView,
        /// Selects a register.
        PushRegister,
        /// Starts recording to a register.
        PushRecordRegister,
        /// Replays a register.
        PushReplayRegister,
        /// Replaces with register contents.
        PushReplaceWithRegister,
        /// Toggles comments.
        PushToggleComments,
    ]
);

// in the workspace namespace so it's not filtered out when vim is disabled.
actions!(
    workspace,
    [
        /// Toggles Vim mode on or off.
        ToggleVimMode,
    ]
);

/// Initializes the `vim` crate.
pub fn init(cx: &mut App) {
    vim_mode_setting::init(cx);
    VimSettings::register(cx);
    VimGlobals::register(cx);

    cx.observe_new(Vim::register).detach();

    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleVimMode, _, cx| {
            let fs = workspace.app_state().fs.clone();
            let currently_enabled = Vim::enabled(cx);
            update_settings_file::<VimModeSetting>(fs, cx, move |setting, _| {
                *setting = Some(!currently_enabled)
            })
        });

        workspace.register_action(|_, _: &OpenDefaultKeymap, _, cx| {
            cx.emit(workspace::Event::OpenBundledFile {
                text: settings::vim_keymap(),
                title: "Default Vim Bindings",
                language: "JSON",
            });
        });

        workspace.register_action(|workspace, _: &ResetPaneSizes, _, cx| {
            workspace.reset_pane_sizes(cx);
        });

        workspace.register_action(|workspace, _: &MaximizePane, window, cx| {
            let pane = workspace.active_pane();
            let Some(size) = workspace.bounding_box_for_pane(&pane) else {
                return;
            };

            let theme = ThemeSettings::get_global(cx);
            let height = theme.buffer_font_size(cx) * theme.buffer_line_height.value();

            let desired_size = if let Some(count) = Vim::take_count(cx) {
                height * count
            } else {
                px(10000.)
            };
            workspace.resize_pane(Axis::Vertical, desired_size - size.size.height, window, cx)
        });

        workspace.register_action(|workspace, _: &ResizePaneRight, window, cx| {
            let count = Vim::take_count(cx).unwrap_or(1) as f32;
            Vim::take_forced_motion(cx);
            let theme = ThemeSettings::get_global(cx);
            let Ok(font_id) = window.text_system().font_id(&theme.buffer_font) else {
                return;
            };
            let Ok(width) = window
                .text_system()
                .advance(font_id, theme.buffer_font_size(cx), 'm')
            else {
                return;
            };
            workspace.resize_pane(Axis::Horizontal, width.width * count, window, cx);
        });

        workspace.register_action(|workspace, _: &ResizePaneLeft, window, cx| {
            let count = Vim::take_count(cx).unwrap_or(1) as f32;
            Vim::take_forced_motion(cx);
            let theme = ThemeSettings::get_global(cx);
            let Ok(font_id) = window.text_system().font_id(&theme.buffer_font) else {
                return;
            };
            let Ok(width) = window
                .text_system()
                .advance(font_id, theme.buffer_font_size(cx), 'm')
            else {
                return;
            };
            workspace.resize_pane(Axis::Horizontal, -width.width * count, window, cx);
        });

        workspace.register_action(|workspace, _: &ResizePaneUp, window, cx| {
            let count = Vim::take_count(cx).unwrap_or(1) as f32;
            Vim::take_forced_motion(cx);
            let theme = ThemeSettings::get_global(cx);
            let height = theme.buffer_font_size(cx) * theme.buffer_line_height.value();
            workspace.resize_pane(Axis::Vertical, height * count, window, cx);
        });

        workspace.register_action(|workspace, _: &ResizePaneDown, window, cx| {
            let count = Vim::take_count(cx).unwrap_or(1) as f32;
            Vim::take_forced_motion(cx);
            let theme = ThemeSettings::get_global(cx);
            let height = theme.buffer_font_size(cx) * theme.buffer_line_height.value();
            workspace.resize_pane(Axis::Vertical, -height * count, window, cx);
        });

        workspace.register_action(|workspace, _: &SearchSubmit, window, cx| {
            let vim = workspace
                .focused_pane(window, cx)
                .read(cx)
                .active_item()
                .and_then(|item| item.act_as::<Editor>(cx))
                .and_then(|editor| editor.read(cx).addon::<VimAddon>().cloned());
            let Some(vim) = vim else { return };
            vim.entity.update(cx, |_, cx| {
                cx.defer_in(window, |vim, window, cx| vim.search_submit(window, cx))
            })
        });
    })
    .detach();
}

#[derive(Clone)]
pub(crate) struct VimAddon {
    pub(crate) entity: Entity<Vim>,
}

impl editor::Addon for VimAddon {
    fn extend_key_context(&self, key_context: &mut KeyContext, cx: &App) {
        self.entity.read(cx).extend_key_context(key_context, cx)
    }

    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// The state pertaining to Vim mode.
pub(crate) struct Vim {
    pub(crate) mode: Mode,
    pub last_mode: Mode,
    pub temp_mode: bool,
    pub status_label: Option<SharedString>,
    pub exit_temporary_mode: bool,

    operator_stack: Vec<Operator>,
    pub(crate) replacements: Vec<(Range<editor::Anchor>, String)>,

    pub(crate) stored_visual_mode: Option<(Mode, Vec<bool>)>,

    pub(crate) current_tx: Option<TransactionId>,
    pub(crate) current_anchor: Option<Selection<Anchor>>,
    pub(crate) undo_modes: HashMap<TransactionId, Mode>,
    pub(crate) undo_last_line_tx: Option<TransactionId>,

    selected_register: Option<char>,
    pub search: SearchState,

    editor: WeakEntity<Editor>,

    last_command: Option<String>,
    running_command: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

// Hack: Vim intercepts events dispatched to a window and updates the view in response.
// This means it needs a VisualContext. The easiest way to satisfy that constraint is
// to make Vim a "View" that is just never actually rendered.
impl Render for Vim {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

enum VimEvent {
    Focused,
}
impl EventEmitter<VimEvent> for Vim {}

impl Vim {
    /// The namespace for Vim actions.
    const NAMESPACE: &'static str = "vim";

    pub fn new(window: &mut Window, cx: &mut Context<Editor>) -> Entity<Self> {
        let editor = cx.entity().clone();

        let mut initial_mode = VimSettings::get_global(cx).default_mode;
        if initial_mode == Mode::Normal && HelixModeSetting::get_global(cx).0 {
            initial_mode = Mode::HelixNormal;
        }

        cx.new(|cx| Vim {
            mode: initial_mode,
            last_mode: Mode::Normal,
            temp_mode: false,
            exit_temporary_mode: false,
            operator_stack: Vec::new(),
            replacements: Vec::new(),

            stored_visual_mode: None,
            current_tx: None,
            undo_last_line_tx: None,
            current_anchor: None,
            undo_modes: HashMap::default(),

            status_label: None,
            selected_register: None,
            search: SearchState::default(),

            last_command: None,
            running_command: None,

            editor: editor.downgrade(),
            _subscriptions: vec![
                cx.observe_keystrokes(Self::observe_keystrokes),
                cx.subscribe_in(&editor, window, |this, _, event, window, cx| {
                    this.handle_editor_event(event, window, cx)
                }),
            ],
        })
    }

    fn register(editor: &mut Editor, window: Option<&mut Window>, cx: &mut Context<Editor>) {
        let Some(window) = window else {
            return;
        };

        if !editor.use_modal_editing() {
            return;
        }

        let mut was_enabled = Vim::enabled(cx);
        let mut was_toggle = VimSettings::get_global(cx).toggle_relative_line_numbers;
        cx.observe_global_in::<SettingsStore>(window, move |editor, window, cx| {
            let enabled = Vim::enabled(cx);
            let toggle = VimSettings::get_global(cx).toggle_relative_line_numbers;
            if enabled && was_enabled && (toggle != was_toggle) {
                if toggle {
                    let is_relative = editor
                        .addon::<VimAddon>()
                        .map(|vim| vim.entity.read(cx).mode != Mode::Insert);
                    editor.set_relative_line_number(is_relative, cx)
                } else {
                    editor.set_relative_line_number(None, cx)
                }
            }
            was_toggle = VimSettings::get_global(cx).toggle_relative_line_numbers;
            if was_enabled == enabled {
                return;
            }
            was_enabled = enabled;
            if enabled {
                Self::activate(editor, window, cx)
            } else {
                Self::deactivate(editor, cx)
            }
        })
        .detach();
        if was_enabled {
            Self::activate(editor, window, cx)
        }
    }

    fn activate(editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>) {
        let vim = Vim::new(window, cx);

        if !editor.mode().is_full() {
            vim.update(cx, |vim, _| {
                vim.mode = Mode::Insert;
            });
        }

        editor.register_addon(VimAddon {
            entity: vim.clone(),
        });

        vim.update(cx, |_, cx| {
            Vim::action(editor, cx, |vim, _: &SwitchToNormalMode, window, cx| {
                if HelixModeSetting::get_global(cx).0 {
                    vim.switch_mode(Mode::HelixNormal, false, window, cx)
                } else {
                    vim.switch_mode(Mode::Normal, false, window, cx)
                }
            });

            Vim::action(editor, cx, |vim, _: &SwitchToInsertMode, window, cx| {
                vim.switch_mode(Mode::Insert, false, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &SwitchToReplaceMode, window, cx| {
                vim.switch_mode(Mode::Replace, false, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &SwitchToVisualMode, window, cx| {
                vim.switch_mode(Mode::Visual, false, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &SwitchToVisualLineMode, window, cx| {
                vim.switch_mode(Mode::VisualLine, false, window, cx)
            });

            Vim::action(
                editor,
                cx,
                |vim, _: &SwitchToVisualBlockMode, window, cx| {
                    vim.switch_mode(Mode::VisualBlock, false, window, cx)
                },
            );

            Vim::action(
                editor,
                cx,
                |vim, _: &SwitchToHelixNormalMode, window, cx| {
                    vim.switch_mode(Mode::HelixNormal, false, window, cx)
                },
            );
            Vim::action(editor, cx, |_, _: &PushForcedMotion, _, cx| {
                Vim::globals(cx).forced_motion = true;
            });
            Vim::action(editor, cx, |vim, action: &PushObject, window, cx| {
                vim.push_operator(
                    Operator::Object {
                        around: action.around,
                    },
                    window,
                    cx,
                )
            });

            Vim::action(editor, cx, |vim, action: &PushFindForward, window, cx| {
                vim.push_operator(
                    Operator::FindForward {
                        before: action.before,
                        multiline: action.multiline,
                    },
                    window,
                    cx,
                )
            });

            Vim::action(editor, cx, |vim, action: &PushFindBackward, window, cx| {
                vim.push_operator(
                    Operator::FindBackward {
                        after: action.after,
                        multiline: action.multiline,
                    },
                    window,
                    cx,
                )
            });

            Vim::action(editor, cx, |vim, action: &PushSneak, window, cx| {
                vim.push_operator(
                    Operator::Sneak {
                        first_char: action.first_char,
                    },
                    window,
                    cx,
                )
            });

            Vim::action(editor, cx, |vim, action: &PushSneakBackward, window, cx| {
                vim.push_operator(
                    Operator::SneakBackward {
                        first_char: action.first_char,
                    },
                    window,
                    cx,
                )
            });

            Vim::action(editor, cx, |vim, _: &PushAddSurrounds, window, cx| {
                vim.push_operator(Operator::AddSurrounds { target: None }, window, cx)
            });

            Vim::action(
                editor,
                cx,
                |vim, action: &PushChangeSurrounds, window, cx| {
                    vim.push_operator(
                        Operator::ChangeSurrounds {
                            target: action.target,
                        },
                        window,
                        cx,
                    )
                },
            );

            Vim::action(editor, cx, |vim, action: &PushJump, window, cx| {
                vim.push_operator(Operator::Jump { line: action.line }, window, cx)
            });

            Vim::action(editor, cx, |vim, action: &PushDigraph, window, cx| {
                vim.push_operator(
                    Operator::Digraph {
                        first_char: action.first_char,
                    },
                    window,
                    cx,
                )
            });

            Vim::action(editor, cx, |vim, action: &PushLiteral, window, cx| {
                vim.push_operator(
                    Operator::Literal {
                        prefix: action.prefix.clone(),
                    },
                    window,
                    cx,
                )
            });

            Vim::action(editor, cx, |vim, _: &PushChange, window, cx| {
                vim.push_operator(Operator::Change, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushDelete, window, cx| {
                vim.push_operator(Operator::Delete, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushYank, window, cx| {
                vim.push_operator(Operator::Yank, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushReplace, window, cx| {
                vim.push_operator(Operator::Replace, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushDeleteSurrounds, window, cx| {
                vim.push_operator(Operator::DeleteSurrounds, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushMark, window, cx| {
                vim.push_operator(Operator::Mark, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushIndent, window, cx| {
                vim.push_operator(Operator::Indent, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushOutdent, window, cx| {
                vim.push_operator(Operator::Outdent, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushAutoIndent, window, cx| {
                vim.push_operator(Operator::AutoIndent, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushRewrap, window, cx| {
                vim.push_operator(Operator::Rewrap, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushShellCommand, window, cx| {
                vim.push_operator(Operator::ShellCommand, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushLowercase, window, cx| {
                vim.push_operator(Operator::Lowercase, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushUppercase, window, cx| {
                vim.push_operator(Operator::Uppercase, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushOppositeCase, window, cx| {
                vim.push_operator(Operator::OppositeCase, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushRot13, window, cx| {
                vim.push_operator(Operator::Rot13, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushRot47, window, cx| {
                vim.push_operator(Operator::Rot47, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushRegister, window, cx| {
                vim.push_operator(Operator::Register, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushRecordRegister, window, cx| {
                vim.push_operator(Operator::RecordRegister, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushReplayRegister, window, cx| {
                vim.push_operator(Operator::ReplayRegister, window, cx)
            });

            Vim::action(
                editor,
                cx,
                |vim, _: &PushReplaceWithRegister, window, cx| {
                    vim.push_operator(Operator::ReplaceWithRegister, window, cx)
                },
            );

            Vim::action(editor, cx, |vim, _: &Exchange, window, cx| {
                if vim.mode.is_visual() {
                    vim.exchange_visual(window, cx)
                } else {
                    vim.push_operator(Operator::Exchange, window, cx)
                }
            });

            Vim::action(editor, cx, |vim, _: &ClearExchange, window, cx| {
                vim.clear_exchange(window, cx)
            });

            Vim::action(editor, cx, |vim, _: &PushToggleComments, window, cx| {
                vim.push_operator(Operator::ToggleComments, window, cx)
            });

            Vim::action(editor, cx, |vim, _: &ClearOperators, window, cx| {
                vim.clear_operator(window, cx)
            });
            Vim::action(editor, cx, |vim, n: &Number, window, cx| {
                vim.push_count_digit(n.0, window, cx);
            });
            Vim::action(editor, cx, |vim, _: &Tab, window, cx| {
                vim.input_ignored(" ".into(), window, cx)
            });
            Vim::action(
                editor,
                cx,
                |vim, action: &editor::actions::AcceptEditPrediction, window, cx| {
                    vim.update_editor(window, cx, |_, editor, window, cx| {
                        editor.accept_edit_prediction(action, window, cx);
                    });
                    // In non-insertion modes, predictions will be hidden and instead a jump will be
                    // displayed (and performed by `accept_edit_prediction`). This switches to
                    // insert mode so that the prediction is displayed after the jump.
                    match vim.mode {
                        Mode::Replace => {}
                        _ => vim.switch_mode(Mode::Insert, true, window, cx),
                    };
                },
            );
            Vim::action(editor, cx, |vim, _: &Enter, window, cx| {
                vim.input_ignored("\n".into(), window, cx)
            });

            normal::register(editor, cx);
            insert::register(editor, cx);
            helix::register(editor, cx);
            motion::register(editor, cx);
            command::register(editor, cx);
            replace::register(editor, cx);
            indent::register(editor, cx);
            rewrap::register(editor, cx);
            object::register(editor, cx);
            visual::register(editor, cx);
            change_list::register(editor, cx);
            digraph::register(editor, cx);

            cx.defer_in(window, |vim, window, cx| {
                vim.focused(false, window, cx);
            })
        })
    }

    fn deactivate(editor: &mut Editor, cx: &mut Context<Editor>) {
        editor.set_cursor_shape(CursorShape::Bar, cx);
        editor.set_clip_at_line_ends(false, cx);
        editor.set_collapse_matches(false);
        editor.set_input_enabled(true);
        editor.set_autoindent(true);
        editor.selections.line_mode = false;
        editor.unregister_addon::<VimAddon>();
        editor.set_relative_line_number(None, cx);
        if let Some(vim) = Vim::globals(cx).focused_vim() {
            if vim.entity_id() == cx.entity().entity_id() {
                Vim::globals(cx).focused_vim = None;
            }
        }
    }

    /// Register an action on the editor.
    pub fn action<A: Action>(
        editor: &mut Editor,
        cx: &mut Context<Vim>,
        f: impl Fn(&mut Vim, &A, &mut Window, &mut Context<Vim>) + 'static,
    ) {
        let subscription = editor.register_action(cx.listener(f));
        cx.on_release(|_, _| drop(subscription)).detach();
    }

    pub fn editor(&self) -> Option<Entity<Editor>> {
        self.editor.upgrade()
    }

    pub fn workspace(&self, window: &mut Window) -> Option<Entity<Workspace>> {
        window.root::<Workspace>().flatten()
    }

    pub fn pane(&self, window: &mut Window, cx: &mut Context<Self>) -> Option<Entity<Pane>> {
        self.workspace(window)
            .map(|workspace| workspace.read(cx).focused_pane(window, cx))
    }

    pub fn enabled(cx: &mut App) -> bool {
        VimModeSetting::get_global(cx).0 || HelixModeSetting::get_global(cx).0
    }

    /// Called whenever an keystroke is typed so vim can observe all actions
    /// and keystrokes accordingly.
    fn observe_keystrokes(
        &mut self,
        keystroke_event: &KeystrokeEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.exit_temporary_mode {
            self.exit_temporary_mode = false;
            // Don't switch to insert mode if the action is temporary_normal.
            if let Some(action) = keystroke_event.action.as_ref() {
                if action.as_any().downcast_ref::<TemporaryNormal>().is_some() {
                    return;
                }
            }
            self.switch_mode(Mode::Insert, false, window, cx)
        }
        if let Some(action) = keystroke_event.action.as_ref() {
            // Keystroke is handled by the vim system, so continue forward
            if action.name().starts_with("vim::") {
                self.update_editor(window, cx, |_, editor, _, cx| {
                    editor.hide_mouse_cursor(HideMouseCursorOrigin::MovementAction, cx)
                });
                return;
            }
        } else if window.has_pending_keystrokes() || keystroke_event.keystroke.is_ime_in_progress()
        {
            return;
        }

        if let Some(operator) = self.active_operator() {
            match operator {
                Operator::Literal { prefix } => {
                    self.handle_literal_keystroke(
                        keystroke_event,
                        prefix.unwrap_or_default(),
                        window,
                        cx,
                    );
                }
                _ if !operator.is_waiting(self.mode) => {
                    self.clear_operator(window, cx);
                    self.stop_recording_immediately(Box::new(ClearOperators), cx)
                }
                _ => {}
            }
        }
    }

    fn handle_editor_event(
        &mut self,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::Focused => self.focused(true, window, cx),
            EditorEvent::Blurred => self.blurred(window, cx),
            EditorEvent::SelectionsChanged { local: true } => {
                self.local_selections_changed(window, cx);
            }
            EditorEvent::InputIgnored { text } => {
                self.input_ignored(text.clone(), window, cx);
                Vim::globals(cx).observe_insertion(text, None)
            }
            EditorEvent::InputHandled {
                text,
                utf16_range_to_replace: range_to_replace,
            } => Vim::globals(cx).observe_insertion(text, range_to_replace.clone()),
            EditorEvent::TransactionBegun { transaction_id } => {
                self.transaction_begun(*transaction_id, window, cx)
            }
            EditorEvent::TransactionUndone { transaction_id } => {
                self.transaction_undone(transaction_id, window, cx)
            }
            EditorEvent::Edited { .. } => self.push_to_change_list(window, cx),
            EditorEvent::FocusedIn => self.sync_vim_settings(window, cx),
            EditorEvent::CursorShapeChanged => self.cursor_shape_changed(window, cx),
            EditorEvent::PushedToNavHistory {
                anchor,
                is_deactivate,
            } => {
                self.update_editor(window, cx, |vim, editor, window, cx| {
                    let mark = if *is_deactivate {
                        "\"".to_string()
                    } else {
                        "'".to_string()
                    };
                    vim.set_mark(mark, vec![*anchor], editor.buffer(), window, cx);
                });
            }
            _ => {}
        }
    }

    fn push_operator(&mut self, operator: Operator, window: &mut Window, cx: &mut Context<Self>) {
        if operator.starts_dot_recording() {
            self.start_recording(cx);
        }
        // Since these operations can only be entered with pre-operators,
        // we need to clear the previous operators when pushing,
        // so that the current stack is the most correct
        if matches!(
            operator,
            Operator::AddSurrounds { .. }
                | Operator::ChangeSurrounds { .. }
                | Operator::DeleteSurrounds
                | Operator::Exchange
        ) {
            self.operator_stack.clear();
        };
        self.operator_stack.push(operator);
        self.sync_vim_settings(window, cx);
    }

    pub fn switch_mode(
        &mut self,
        mode: Mode,
        leave_selections: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.temp_mode && mode == Mode::Normal {
            self.temp_mode = false;
            self.switch_mode(Mode::Normal, leave_selections, window, cx);
            self.switch_mode(Mode::Insert, false, window, cx);
            return;
        } else if self.temp_mode
            && !matches!(mode, Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
        {
            self.temp_mode = false;
        }

        let last_mode = self.mode;
        let prior_mode = self.last_mode;
        let prior_tx = self.current_tx;
        self.status_label.take();
        self.last_mode = last_mode;
        self.mode = mode;
        self.operator_stack.clear();
        self.selected_register.take();
        self.cancel_running_command(window, cx);
        if mode == Mode::Normal || mode != last_mode {
            self.current_tx.take();
            self.current_anchor.take();
            self.update_editor(window, cx, |_, editor, _, _| {
                editor.clear_selection_drag_state();
            });
        }
        Vim::take_forced_motion(cx);
        if mode != Mode::Insert && mode != Mode::Replace {
            Vim::take_count(cx);
        }

        // Sync editor settings like clip mode
        self.sync_vim_settings(window, cx);

        if VimSettings::get_global(cx).toggle_relative_line_numbers
            && self.mode != self.last_mode
            && (self.mode == Mode::Insert || self.last_mode == Mode::Insert)
        {
            self.update_editor(window, cx, |vim, editor, _, cx| {
                let is_relative = vim.mode != Mode::Insert;
                editor.set_relative_line_number(Some(is_relative), cx)
            });
        }

        if leave_selections {
            return;
        }

        if !mode.is_visual() && last_mode.is_visual() {
            self.create_visual_marks(last_mode, window, cx);
        }

        // Adjust selections
        self.update_editor(window, cx, |vim, editor, window, cx| {
            if last_mode != Mode::VisualBlock && last_mode.is_visual() && mode == Mode::VisualBlock
            {
                vim.visual_block_motion(true, editor, window, cx, |_, point, goal| {
                    Some((point, goal))
                })
            }
            if last_mode == Mode::Insert || last_mode == Mode::Replace {
                if let Some(prior_tx) = prior_tx {
                    editor.group_until_transaction(prior_tx, cx)
                }
            }

            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                // we cheat with visual block mode and use multiple cursors.
                // the cost of this cheat is we need to convert back to a single
                // cursor whenever vim would.
                if last_mode == Mode::VisualBlock
                    && (mode != Mode::VisualBlock && mode != Mode::Insert)
                {
                    let tail = s.oldest_anchor().tail();
                    let head = s.newest_anchor().head();
                    s.select_anchor_ranges(vec![tail..head]);
                } else if last_mode == Mode::Insert
                    && prior_mode == Mode::VisualBlock
                    && mode != Mode::VisualBlock
                {
                    let pos = s.first_anchor().head();
                    s.select_anchor_ranges(vec![pos..pos])
                }

                let snapshot = s.display_map();
                if let Some(pending) = s.pending.as_mut() {
                    if pending.selection.reversed && mode.is_visual() && !last_mode.is_visual() {
                        let mut end = pending.selection.end.to_point(&snapshot.buffer_snapshot);
                        end = snapshot
                            .buffer_snapshot
                            .clip_point(end + Point::new(0, 1), Bias::Right);
                        pending.selection.end = snapshot.buffer_snapshot.anchor_before(end);
                    }
                }

                s.move_with(|map, selection| {
                    if last_mode.is_visual() && !mode.is_visual() {
                        let mut point = selection.head();
                        if !selection.reversed && !selection.is_empty() {
                            point = movement::left(map, selection.head());
                        }
                        selection.collapse_to(point, selection.goal)
                    } else if !last_mode.is_visual() && mode.is_visual() && selection.is_empty() {
                        selection.end = movement::right(map, selection.start);
                    }
                });
            })
        });
    }

    pub fn take_count(cx: &mut App) -> Option<usize> {
        let global_state = cx.global_mut::<VimGlobals>();
        if global_state.dot_replaying {
            return global_state.recorded_count;
        }

        let count = if global_state.post_count.is_none() && global_state.pre_count.is_none() {
            return None;
        } else {
            Some(
                global_state.post_count.take().unwrap_or(1)
                    * global_state.pre_count.take().unwrap_or(1),
            )
        };

        if global_state.dot_recording {
            global_state.recorded_count = count;
        }
        count
    }

    pub fn take_forced_motion(cx: &mut App) -> bool {
        let global_state = cx.global_mut::<VimGlobals>();
        let forced_motion = global_state.forced_motion;
        global_state.forced_motion = false;
        forced_motion
    }

    pub fn cursor_shape(&self, cx: &mut App) -> CursorShape {
        let cursor_shape = VimSettings::get_global(cx).cursor_shape;
        match self.mode {
            Mode::Normal => {
                if let Some(operator) = self.operator_stack.last() {
                    match operator {
                        // Navigation operators -> Block cursor
                        Operator::FindForward { .. }
                        | Operator::FindBackward { .. }
                        | Operator::Mark
                        | Operator::Jump { .. }
                        | Operator::Register
                        | Operator::RecordRegister
                        | Operator::ReplayRegister => CursorShape::Block,

                        // All other operators -> Underline cursor
                        _ => CursorShape::Underline,
                    }
                } else {
                    cursor_shape.normal.unwrap_or(CursorShape::Block)
                }
            }
            Mode::HelixNormal => cursor_shape.normal.unwrap_or(CursorShape::Block),
            Mode::Replace => cursor_shape.replace.unwrap_or(CursorShape::Underline),
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                cursor_shape.visual.unwrap_or(CursorShape::Block)
            }
            Mode::Insert => cursor_shape.insert.unwrap_or({
                let editor_settings = EditorSettings::get_global(cx);
                editor_settings.cursor_shape.unwrap_or_default()
            }),
        }
    }

    pub fn editor_input_enabled(&self) -> bool {
        match self.mode {
            Mode::Insert => {
                if let Some(operator) = self.operator_stack.last() {
                    !operator.is_waiting(self.mode)
                } else {
                    true
                }
            }
            Mode::Normal
            | Mode::HelixNormal
            | Mode::Replace
            | Mode::Visual
            | Mode::VisualLine
            | Mode::VisualBlock => false,
        }
    }

    pub fn should_autoindent(&self) -> bool {
        !(self.mode == Mode::Insert && self.last_mode == Mode::VisualBlock)
    }

    pub fn clip_at_line_ends(&self) -> bool {
        match self.mode {
            Mode::Insert
            | Mode::Visual
            | Mode::VisualLine
            | Mode::VisualBlock
            | Mode::Replace
            | Mode::HelixNormal => false,
            Mode::Normal => true,
        }
    }

    pub fn extend_key_context(&self, context: &mut KeyContext, cx: &App) {
        let mut mode = match self.mode {
            Mode::Normal => "normal",
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => "visual",
            Mode::Insert => "insert",
            Mode::Replace => "replace",
            Mode::HelixNormal => "helix_normal",
        }
        .to_string();

        let mut operator_id = "none";

        let active_operator = self.active_operator();
        if active_operator.is_none() && cx.global::<VimGlobals>().pre_count.is_some()
            || active_operator.is_some() && cx.global::<VimGlobals>().post_count.is_some()
        {
            context.add("VimCount");
        }

        if let Some(active_operator) = active_operator {
            if active_operator.is_waiting(self.mode) {
                if matches!(active_operator, Operator::Literal { .. }) {
                    mode = "literal".to_string();
                } else {
                    mode = "waiting".to_string();
                }
            } else {
                operator_id = active_operator.id();
                mode = "operator".to_string();
            }
        }

        if mode == "normal" || mode == "visual" || mode == "operator" || mode == "helix_normal" {
            context.add("VimControl");
        }
        context.set("vim_mode", mode);
        context.set("vim_operator", operator_id);
    }

    fn focused(&mut self, preserve_selection: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(editor) = self.editor() else {
            return;
        };
        let newest_selection_empty = editor.update(cx, |editor, cx| {
            editor.selections.newest::<usize>(cx).is_empty()
        });
        let editor = editor.read(cx);
        let editor_mode = editor.mode();

        if editor_mode.is_full()
            && !newest_selection_empty
            && self.mode == Mode::Normal
            // When following someone, don't switch vim mode.
            && editor.leader_id().is_none()
        {
            if preserve_selection {
                self.switch_mode(Mode::Visual, true, window, cx);
            } else {
                self.update_editor(window, cx, |_, editor, window, cx| {
                    editor.set_clip_at_line_ends(false, cx);
                    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                        s.move_with(|_, selection| {
                            selection.collapse_to(selection.start, selection.goal)
                        })
                    });
                });
            }
        }

        cx.emit(VimEvent::Focused);
        self.sync_vim_settings(window, cx);

        if VimSettings::get_global(cx).toggle_relative_line_numbers {
            if let Some(old_vim) = Vim::globals(cx).focused_vim() {
                if old_vim.entity_id() != cx.entity().entity_id() {
                    old_vim.update(cx, |vim, cx| {
                        vim.update_editor(window, cx, |_, editor, _, cx| {
                            editor.set_relative_line_number(None, cx)
                        });
                    });

                    self.update_editor(window, cx, |vim, editor, _, cx| {
                        let is_relative = vim.mode != Mode::Insert;
                        editor.set_relative_line_number(Some(is_relative), cx)
                    });
                }
            } else {
                self.update_editor(window, cx, |vim, editor, _, cx| {
                    let is_relative = vim.mode != Mode::Insert;
                    editor.set_relative_line_number(Some(is_relative), cx)
                });
            }
        }
        Vim::globals(cx).focused_vim = Some(cx.entity().downgrade());
    }

    fn blurred(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.stop_recording_immediately(NormalBefore.boxed_clone(), cx);
        self.store_visual_marks(window, cx);
        self.clear_operator(window, cx);
        self.update_editor(window, cx, |vim, editor, _, cx| {
            if vim.cursor_shape(cx) == CursorShape::Block {
                editor.set_cursor_shape(CursorShape::Hollow, cx);
            }
        });
    }

    fn cursor_shape_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(window, cx, |vim, editor, _, cx| {
            editor.set_cursor_shape(vim.cursor_shape(cx), cx);
        });
    }

    fn update_editor<S>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        update: impl FnOnce(&mut Self, &mut Editor, &mut Window, &mut Context<Editor>) -> S,
    ) -> Option<S> {
        let editor = self.editor.upgrade()?;
        Some(editor.update(cx, |editor, cx| update(self, editor, window, cx)))
    }

    fn editor_selections(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Range<Anchor>> {
        self.update_editor(window, cx, |_, editor, _, _| {
            editor
                .selections
                .disjoint_anchors()
                .iter()
                .map(|selection| selection.tail()..selection.head())
                .collect()
        })
        .unwrap_or_default()
    }

    fn editor_cursor_word(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<String> {
        self.update_editor(window, cx, |_, editor, window, cx| {
            let selection = editor.selections.newest::<usize>(cx);

            let snapshot = &editor.snapshot(window, cx).buffer_snapshot;
            let (range, kind) = snapshot.surrounding_word(selection.start, true);
            if kind == Some(CharKind::Word) {
                let text: String = snapshot.text_for_range(range).collect();
                if !text.trim().is_empty() {
                    return Some(text);
                }
            }

            None
        })
        .unwrap_or_default()
    }

    /// When doing an action that modifies the buffer, we start recording so that `.`
    /// will replay the action.
    pub fn start_recording(&mut self, cx: &mut Context<Self>) {
        Vim::update_globals(cx, |globals, cx| {
            if !globals.dot_replaying {
                globals.dot_recording = true;
                globals.recording_actions = Default::default();
                globals.recorded_count = None;

                let selections = self.editor().map(|editor| {
                    editor.update(cx, |editor, cx| {
                        (
                            editor.selections.oldest::<Point>(cx),
                            editor.selections.newest::<Point>(cx),
                        )
                    })
                });

                if let Some((oldest, newest)) = selections {
                    globals.recorded_selection = match self.mode {
                        Mode::Visual if newest.end.row == newest.start.row => {
                            RecordedSelection::SingleLine {
                                cols: newest.end.column - newest.start.column,
                            }
                        }
                        Mode::Visual => RecordedSelection::Visual {
                            rows: newest.end.row - newest.start.row,
                            cols: newest.end.column,
                        },
                        Mode::VisualLine => RecordedSelection::VisualLine {
                            rows: newest.end.row - newest.start.row,
                        },
                        Mode::VisualBlock => RecordedSelection::VisualBlock {
                            rows: newest.end.row.abs_diff(oldest.start.row),
                            cols: newest.end.column.abs_diff(oldest.start.column),
                        },
                        _ => RecordedSelection::None,
                    }
                } else {
                    globals.recorded_selection = RecordedSelection::None;
                }
            }
        })
    }

    pub fn stop_replaying(&mut self, cx: &mut Context<Self>) {
        let globals = Vim::globals(cx);
        globals.dot_replaying = false;
        if let Some(replayer) = globals.replayer.take() {
            replayer.stop();
        }
    }

    /// When finishing an action that modifies the buffer, stop recording.
    /// as you usually call this within a keystroke handler we also ensure that
    /// the current action is recorded.
    pub fn stop_recording(&mut self, cx: &mut Context<Self>) {
        let globals = Vim::globals(cx);
        if globals.dot_recording {
            globals.stop_recording_after_next_action = true;
        }
        self.exit_temporary_mode = self.temp_mode;
    }

    /// Stops recording actions immediately rather than waiting until after the
    /// next action to stop recording.
    ///
    /// This doesn't include the current action.
    pub fn stop_recording_immediately(&mut self, action: Box<dyn Action>, cx: &mut Context<Self>) {
        let globals = Vim::globals(cx);
        if globals.dot_recording {
            globals
                .recording_actions
                .push(ReplayableAction::Action(action.boxed_clone()));
            globals.recorded_actions = mem::take(&mut globals.recording_actions);
            globals.dot_recording = false;
            globals.stop_recording_after_next_action = false;
        }
        self.exit_temporary_mode = self.temp_mode;
    }

    /// Explicitly record one action (equivalents to start_recording and stop_recording)
    pub fn record_current_action(&mut self, cx: &mut Context<Self>) {
        self.start_recording(cx);
        self.stop_recording(cx);
    }

    fn push_count_digit(&mut self, number: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.active_operator().is_some() {
            let post_count = Vim::globals(cx).post_count.unwrap_or(0);

            Vim::globals(cx).post_count = Some(
                post_count
                    .checked_mul(10)
                    .and_then(|post_count| post_count.checked_add(number))
                    .unwrap_or(post_count),
            )
        } else {
            let pre_count = Vim::globals(cx).pre_count.unwrap_or(0);

            Vim::globals(cx).pre_count = Some(
                pre_count
                    .checked_mul(10)
                    .and_then(|pre_count| pre_count.checked_add(number))
                    .unwrap_or(pre_count),
            )
        }
        // update the keymap so that 0 works
        self.sync_vim_settings(window, cx)
    }

    fn select_register(&mut self, register: Arc<str>, window: &mut Window, cx: &mut Context<Self>) {
        if register.chars().count() == 1 {
            self.selected_register
                .replace(register.chars().next().unwrap());
        }
        self.operator_stack.clear();
        self.sync_vim_settings(window, cx);
    }

    fn maybe_pop_operator(&mut self) -> Option<Operator> {
        self.operator_stack.pop()
    }

    fn pop_operator(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Operator {
        let popped_operator = self.operator_stack.pop()
            .expect("Operator popped when no operator was on the stack. This likely means there is an invalid keymap config");
        self.sync_vim_settings(window, cx);
        popped_operator
    }

    fn clear_operator(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        Vim::take_count(cx);
        Vim::take_forced_motion(cx);
        self.selected_register.take();
        self.operator_stack.clear();
        self.sync_vim_settings(window, cx);
    }

    fn active_operator(&self) -> Option<Operator> {
        self.operator_stack.last().cloned()
    }

    fn transaction_begun(
        &mut self,
        transaction_id: TransactionId,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
        let mode = if (self.mode == Mode::Insert
            || self.mode == Mode::Replace
            || self.mode == Mode::Normal)
            && self.current_tx.is_none()
        {
            self.current_tx = Some(transaction_id);
            self.last_mode
        } else {
            self.mode
        };
        if mode == Mode::VisualLine || mode == Mode::VisualBlock {
            self.undo_modes.insert(transaction_id, mode);
        }
    }

    fn transaction_undone(
        &mut self,
        transaction_id: &TransactionId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.mode {
            Mode::VisualLine | Mode::VisualBlock | Mode::Visual => {
                self.update_editor(window, cx, |vim, editor, window, cx| {
                    let original_mode = vim.undo_modes.get(transaction_id);
                    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                        match original_mode {
                            Some(Mode::VisualLine) => {
                                s.move_with(|map, selection| {
                                    selection.collapse_to(
                                        map.prev_line_boundary(selection.start.to_point(map)).1,
                                        SelectionGoal::None,
                                    )
                                });
                            }
                            Some(Mode::VisualBlock) => {
                                let mut first = s.first_anchor();
                                first.collapse_to(first.start, first.goal);
                                s.select_anchors(vec![first]);
                            }
                            _ => {
                                s.move_with(|map, selection| {
                                    selection.collapse_to(
                                        map.clip_at_line_end(selection.start),
                                        selection.goal,
                                    );
                                });
                            }
                        }
                    });
                });
                self.switch_mode(Mode::Normal, true, window, cx)
            }
            Mode::Normal => {
                self.update_editor(window, cx, |_, editor, window, cx| {
                    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                        s.move_with(|map, selection| {
                            selection
                                .collapse_to(map.clip_at_line_end(selection.end), selection.goal)
                        })
                    })
                });
            }
            Mode::Insert | Mode::Replace | Mode::HelixNormal => {}
        }
    }

    fn local_selections_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(editor) = self.editor() else { return };

        if editor.read(cx).leader_id().is_some() {
            return;
        }

        let newest = editor.read(cx).selections.newest_anchor().clone();
        let is_multicursor = editor.read(cx).selections.count() > 1;
        if self.mode == Mode::Insert && self.current_tx.is_some() {
            if self.current_anchor.is_none() {
                self.current_anchor = Some(newest);
            } else if self.current_anchor.as_ref().unwrap() != &newest {
                if let Some(tx_id) = self.current_tx.take() {
                    self.update_editor(window, cx, |_, editor, _, cx| {
                        editor.group_until_transaction(tx_id, cx)
                    });
                }
            }
        } else if self.mode == Mode::Normal && newest.start != newest.end {
            if matches!(newest.goal, SelectionGoal::HorizontalRange { .. }) {
                self.switch_mode(Mode::VisualBlock, false, window, cx);
            } else {
                self.switch_mode(Mode::Visual, false, window, cx)
            }
        } else if newest.start == newest.end
            && !is_multicursor
            && [Mode::Visual, Mode::VisualLine, Mode::VisualBlock].contains(&self.mode)
        {
            self.switch_mode(Mode::Normal, true, window, cx);
        }
    }

    fn input_ignored(&mut self, text: Arc<str>, window: &mut Window, cx: &mut Context<Self>) {
        if text.is_empty() {
            return;
        }

        match self.active_operator() {
            Some(Operator::FindForward { before, multiline }) => {
                let find = Motion::FindForward {
                    before,
                    char: text.chars().next().unwrap(),
                    mode: if multiline {
                        FindRange::MultiLine
                    } else {
                        FindRange::SingleLine
                    },
                    smartcase: VimSettings::get_global(cx).use_smartcase_find,
                };
                Vim::globals(cx).last_find = Some(find.clone());
                self.motion(find, window, cx)
            }
            Some(Operator::FindBackward { after, multiline }) => {
                let find = Motion::FindBackward {
                    after,
                    char: text.chars().next().unwrap(),
                    mode: if multiline {
                        FindRange::MultiLine
                    } else {
                        FindRange::SingleLine
                    },
                    smartcase: VimSettings::get_global(cx).use_smartcase_find,
                };
                Vim::globals(cx).last_find = Some(find.clone());
                self.motion(find, window, cx)
            }
            Some(Operator::Sneak { first_char }) => {
                if let Some(first_char) = first_char {
                    if let Some(second_char) = text.chars().next() {
                        let sneak = Motion::Sneak {
                            first_char,
                            second_char,
                            smartcase: VimSettings::get_global(cx).use_smartcase_find,
                        };
                        Vim::globals(cx).last_find = Some((&sneak).clone());
                        self.motion(sneak, window, cx)
                    }
                } else {
                    let first_char = text.chars().next();
                    self.pop_operator(window, cx);
                    self.push_operator(Operator::Sneak { first_char }, window, cx);
                }
            }
            Some(Operator::SneakBackward { first_char }) => {
                if let Some(first_char) = first_char {
                    if let Some(second_char) = text.chars().next() {
                        let sneak = Motion::SneakBackward {
                            first_char,
                            second_char,
                            smartcase: VimSettings::get_global(cx).use_smartcase_find,
                        };
                        Vim::globals(cx).last_find = Some((&sneak).clone());
                        self.motion(sneak, window, cx)
                    }
                } else {
                    let first_char = text.chars().next();
                    self.pop_operator(window, cx);
                    self.push_operator(Operator::SneakBackward { first_char }, window, cx);
                }
            }
            Some(Operator::Replace) => match self.mode {
                Mode::Normal => self.normal_replace(text, window, cx),
                Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                    self.visual_replace(text, window, cx)
                }
                Mode::HelixNormal => self.helix_replace(&text, window, cx),
                _ => self.clear_operator(window, cx),
            },
            Some(Operator::Digraph { first_char }) => {
                if let Some(first_char) = first_char {
                    if let Some(second_char) = text.chars().next() {
                        self.insert_digraph(first_char, second_char, window, cx);
                    }
                } else {
                    let first_char = text.chars().next();
                    self.pop_operator(window, cx);
                    self.push_operator(Operator::Digraph { first_char }, window, cx);
                }
            }
            Some(Operator::Literal { prefix }) => {
                self.handle_literal_input(prefix.unwrap_or_default(), &text, window, cx)
            }
            Some(Operator::AddSurrounds { target }) => match self.mode {
                Mode::Normal => {
                    if let Some(target) = target {
                        self.add_surrounds(text, target, window, cx);
                        self.clear_operator(window, cx);
                    }
                }
                Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                    self.add_surrounds(text, SurroundsType::Selection, window, cx);
                    self.clear_operator(window, cx);
                }
                _ => self.clear_operator(window, cx),
            },
            Some(Operator::ChangeSurrounds { target }) => match self.mode {
                Mode::Normal => {
                    if let Some(target) = target {
                        self.change_surrounds(text, target, window, cx);
                        self.clear_operator(window, cx);
                    }
                }
                _ => self.clear_operator(window, cx),
            },
            Some(Operator::DeleteSurrounds) => match self.mode {
                Mode::Normal => {
                    self.delete_surrounds(text, window, cx);
                    self.clear_operator(window, cx);
                }
                _ => self.clear_operator(window, cx),
            },
            Some(Operator::Mark) => self.create_mark(text, window, cx),
            Some(Operator::RecordRegister) => {
                self.record_register(text.chars().next().unwrap(), window, cx)
            }
            Some(Operator::ReplayRegister) => {
                self.replay_register(text.chars().next().unwrap(), window, cx)
            }
            Some(Operator::Register) => match self.mode {
                Mode::Insert => {
                    self.update_editor(window, cx, |_, editor, window, cx| {
                        if let Some(register) = Vim::update_globals(cx, |globals, cx| {
                            globals.read_register(text.chars().next(), Some(editor), cx)
                        }) {
                            editor.do_paste(
                                &register.text.to_string(),
                                register.clipboard_selections.clone(),
                                false,
                                window,
                                cx,
                            )
                        }
                    });
                    self.clear_operator(window, cx);
                }
                _ => {
                    self.select_register(text, window, cx);
                }
            },
            Some(Operator::Jump { line }) => self.jump(text, line, true, window, cx),
            _ => {
                if self.mode == Mode::Replace {
                    self.multi_replace(text, window, cx)
                }

                if self.mode == Mode::Normal {
                    self.update_editor(window, cx, |_, editor, window, cx| {
                        editor.accept_edit_prediction(
                            &editor::actions::AcceptEditPrediction {},
                            window,
                            cx,
                        );
                    });
                }
            }
        }
    }

    fn sync_vim_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(window, cx, |vim, editor, window, cx| {
            editor.set_cursor_shape(vim.cursor_shape(cx), cx);
            editor.set_clip_at_line_ends(vim.clip_at_line_ends(), cx);
            editor.set_collapse_matches(true);
            editor.set_input_enabled(vim.editor_input_enabled());
            editor.set_autoindent(vim.should_autoindent());
            editor.selections.line_mode = matches!(vim.mode, Mode::VisualLine);

            let hide_edit_predictions = match vim.mode {
                Mode::Insert | Mode::Replace => false,
                _ => true,
            };
            editor.set_edit_predictions_hidden_for_vim_mode(hide_edit_predictions, window, cx);
        });
        cx.notify()
    }
}

/// Controls when to use system clipboard.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UseSystemClipboard {
    /// Don't use system clipboard.
    Never,
    /// Use system clipboard.
    Always,
    /// Use system clipboard for yank operations.
    OnYank,
}

/// The settings for cursor shape.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
struct CursorShapeSettings {
    /// Cursor shape for the normal mode.
    ///
    /// Default: block
    pub normal: Option<CursorShape>,
    /// Cursor shape for the replace mode.
    ///
    /// Default: underline
    pub replace: Option<CursorShape>,
    /// Cursor shape for the visual mode.
    ///
    /// Default: block
    pub visual: Option<CursorShape>,
    /// Cursor shape for the insert mode.
    ///
    /// The default value follows the primary cursor_shape.
    pub insert: Option<CursorShape>,
}

#[derive(Deserialize)]
struct VimSettings {
    pub default_mode: Mode,
    pub toggle_relative_line_numbers: bool,
    pub use_system_clipboard: UseSystemClipboard,
    pub use_smartcase_find: bool,
    pub custom_digraphs: HashMap<String, Arc<str>>,
    pub highlight_on_yank_duration: u64,
    pub cursor_shape: CursorShapeSettings,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
struct VimSettingsContent {
    pub default_mode: Option<ModeContent>,
    pub toggle_relative_line_numbers: Option<bool>,
    pub use_system_clipboard: Option<UseSystemClipboard>,
    pub use_smartcase_find: Option<bool>,
    pub custom_digraphs: Option<HashMap<String, Arc<str>>>,
    pub highlight_on_yank_duration: Option<u64>,
    pub cursor_shape: Option<CursorShapeSettings>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModeContent {
    #[default]
    Normal,
    Insert,
    Replace,
    Visual,
    VisualLine,
    VisualBlock,
    HelixNormal,
}

impl From<ModeContent> for Mode {
    fn from(mode: ModeContent) -> Self {
        match mode {
            ModeContent::Normal => Self::Normal,
            ModeContent::Insert => Self::Insert,
            ModeContent::Replace => Self::Replace,
            ModeContent::Visual => Self::Visual,
            ModeContent::VisualLine => Self::VisualLine,
            ModeContent::VisualBlock => Self::VisualBlock,
            ModeContent::HelixNormal => Self::HelixNormal,
        }
    }
}

impl Settings for VimSettings {
    const KEY: Option<&'static str> = Some("vim");

    type FileContent = VimSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        let settings: VimSettingsContent = sources.json_merge()?;

        Ok(Self {
            default_mode: settings
                .default_mode
                .ok_or_else(Self::missing_default)?
                .into(),
            toggle_relative_line_numbers: settings
                .toggle_relative_line_numbers
                .ok_or_else(Self::missing_default)?,
            use_system_clipboard: settings
                .use_system_clipboard
                .ok_or_else(Self::missing_default)?,
            use_smartcase_find: settings
                .use_smartcase_find
                .ok_or_else(Self::missing_default)?,
            custom_digraphs: settings.custom_digraphs.ok_or_else(Self::missing_default)?,
            highlight_on_yank_duration: settings
                .highlight_on_yank_duration
                .ok_or_else(Self::missing_default)?,
            cursor_shape: settings.cursor_shape.ok_or_else(Self::missing_default)?,
        })
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {
        // TODO: translate vim extension settings
    }
}
