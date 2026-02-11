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

use crate::normal::paste::Paste as VimPaste;
use collections::HashMap;
use editor::{
    Anchor, Bias, Editor, EditorEvent, EditorSettings, HideMouseCursorOrigin, MultiBufferOffset,
    SelectionEffects,
    actions::Paste,
    display_map::ToDisplayPoint,
    movement::{self, FindRange},
};
use gpui::{
    Action, App, AppContext, Axis, Context, Entity, EventEmitter, KeyContext, KeystrokeEvent,
    Render, Subscription, Task, WeakEntity, Window, actions,
};
use insert::{NormalBefore, TemporaryNormal};
use language::{
    CharKind, CharScopeContext, CursorShape, Point, Selection, SelectionGoal, TransactionId,
};
pub use mode_indicator::ModeIndicator;
use motion::Motion;
use multi_buffer::ToPoint as _;
use normal::search::SearchSubmit;
use object::Object;
use schemars::JsonSchema;
use search::BufferSearchBar;
use serde::Deserialize;
use settings::RegisterSetting;
pub use settings::{
    ModeContent, Settings, SettingsStore, UseSystemClipboard, update_settings_file,
};
use state::{Mode, Operator, RecordedSelection, SearchState, VimGlobals};
use std::{mem, ops::Range, sync::Arc};
use surrounds::SurroundsType;
use theme::ThemeSettings;
use ui::{IntoElement, SharedString, px};
use vim_mode_setting::HelixModeSetting;
use vim_mode_setting::VimModeSetting;
use workspace::{self, Pane, Workspace};

use crate::{
    normal::{GoToPreviousTab, GoToTab},
    state::ReplayableAction,
};

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
/// Selects the next object.
struct PushHelixNext {
    around: bool,
}

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
/// Selects the previous object.
struct PushHelixPrevious {
    around: bool,
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
        /// Selects (count) next menu item
        MenuSelectNext,
        /// Selects (count) previous menu item
        MenuSelectPrevious,
        /// Clears count or toggles project panel focus
        ToggleProjectPanelFocus,
        /// Starts a match operation.
        PushHelixMatch,
        /// Adds surrounding characters in Helix mode.
        PushHelixSurroundAdd,
        /// Replaces surrounding characters in Helix mode.
        PushHelixSurroundReplace,
        /// Deletes surrounding characters in Helix mode.
        PushHelixSurroundDelete,
    ]
);

// in the workspace namespace so it's not filtered out when vim is disabled.
actions!(
    workspace,
    [
        /// Toggles Vim mode on or off.
        ToggleVimMode,
        /// Toggles Helix mode on or off.
        ToggleHelixMode,
    ]
);

/// Initializes the `vim` crate.
pub fn init(cx: &mut App) {
    VimGlobals::register(cx);

    cx.observe_new(Vim::register).detach();

    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleVimMode, _, cx| {
            let fs = workspace.app_state().fs.clone();
            let currently_enabled = VimModeSetting::get_global(cx).0;
            update_settings_file(fs, cx, move |setting, _| {
                setting.vim_mode = Some(!currently_enabled);
                if let Some(helix_mode) = &mut setting.helix_mode {
                    *helix_mode = false;
                }
            })
        });

        workspace.register_action(|workspace, _: &ToggleHelixMode, _, cx| {
            let fs = workspace.app_state().fs.clone();
            let currently_enabled = HelixModeSetting::get_global(cx).0;
            update_settings_file(fs, cx, move |setting, _| {
                setting.helix_mode = Some(!currently_enabled);
                if let Some(vim_mode) = &mut setting.vim_mode {
                    *vim_mode = false;
                }
            })
        });

        workspace.register_action(|_, _: &MenuSelectNext, window, cx| {
            let count = Vim::take_count(cx).unwrap_or(1);

            for _ in 0..count {
                window.dispatch_action(menu::SelectNext.boxed_clone(), cx);
            }
        });

        workspace.register_action(|_, _: &MenuSelectPrevious, window, cx| {
            let count = Vim::take_count(cx).unwrap_or(1);

            for _ in 0..count {
                window.dispatch_action(menu::SelectPrevious.boxed_clone(), cx);
            }
        });

        workspace.register_action(|_, _: &ToggleProjectPanelFocus, window, cx| {
            if Vim::take_count(cx).is_none() {
                window.dispatch_action(zed_actions::project_panel::ToggleFocus.boxed_clone(), cx);
            }
        });

        workspace.register_action(|workspace, n: &Number, window, cx| {
            let vim = workspace
                .focused_pane(window, cx)
                .read(cx)
                .active_item()
                .and_then(|item| item.act_as::<Editor>(cx))
                .and_then(|editor| editor.read(cx).addon::<VimAddon>().cloned());
            if let Some(vim) = vim {
                let digit = n.0;
                vim.entity.update(cx, |_, cx| {
                    cx.defer_in(window, move |vim, window, cx| {
                        vim.push_count_digit(digit, window, cx)
                    })
                });
            } else {
                let count = Vim::globals(cx).pre_count.unwrap_or(0);
                Vim::globals(cx).pre_count = Some(
                    count
                        .checked_mul(10)
                        .and_then(|c| c.checked_add(n.0))
                        .unwrap_or(count),
                );
            };
        });

        workspace.register_action(|_, _: &zed_actions::vim::OpenDefaultKeymap, _, cx| {
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
            let Some(size) = workspace.bounding_box_for_pane(pane) else {
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
            let font_id = window.text_system().resolve_font(&theme.buffer_font);
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
            let font_id = window.text_system().resolve_font(&theme.buffer_font);
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
        workspace.register_action(|_, _: &GoToTab, window, cx| {
            let count = Vim::take_count(cx);
            Vim::take_forced_motion(cx);

            if let Some(tab_index) = count {
                // <count>gt goes to tab <count> (1-based).
                let zero_based_index = tab_index.saturating_sub(1);
                window.dispatch_action(
                    workspace::pane::ActivateItem(zero_based_index).boxed_clone(),
                    cx,
                );
            } else {
                // If no count is provided, go to the next tab.
                window.dispatch_action(workspace::pane::ActivateNextItem.boxed_clone(), cx);
            }
        });

        workspace.register_action(|workspace, _: &GoToPreviousTab, window, cx| {
            let count = Vim::take_count(cx);
            Vim::take_forced_motion(cx);

            if let Some(count) = count {
                // gT with count goes back that many tabs with wraparound (not the same as gt!).
                let pane = workspace.active_pane().read(cx);
                let item_count = pane.items().count();
                if item_count > 0 {
                    let current_index = pane.active_item_index();
                    let target_index = (current_index as isize - count as isize)
                        .rem_euclid(item_count as isize)
                        as usize;
                    window.dispatch_action(
                        workspace::pane::ActivateItem(target_index).boxed_clone(),
                        cx,
                    );
                }
            } else {
                // No count provided, go to the previous tab.
                window.dispatch_action(workspace::pane::ActivatePreviousItem.boxed_clone(), cx);
            }
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
    extended_pending_selection_id: Option<usize>,

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
        let editor = cx.entity();

        let initial_vim_mode = VimSettings::get_global(cx).default_mode;
        let (mode, last_mode) = if HelixModeSetting::get_global(cx).0 {
            let initial_helix_mode = match initial_vim_mode {
                Mode::Normal => Mode::HelixNormal,
                Mode::Insert => Mode::Insert,
                // Otherwise, we panic with a note that we should never get there due to the
                // possible values of VimSettings::get_global(cx).default_mode being either Mode::Normal or Mode::Insert.
                _ => unreachable!("Invalid default mode"),
            };
            (initial_helix_mode, Mode::HelixNormal)
        } else {
            (initial_vim_mode, Mode::Normal)
        };

        cx.new(|cx| Vim {
            mode,
            last_mode,
            temp_mode: false,
            exit_temporary_mode: false,
            operator_stack: Vec::new(),
            replacements: Vec::new(),

            stored_visual_mode: None,
            current_tx: None,
            undo_last_line_tx: None,
            current_anchor: None,
            extended_pending_selection_id: None,
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
        let state = vim.update(cx, |vim, cx| {
            if !editor.mode().is_full() {
                vim.mode = Mode::Insert;
            }

            vim.state_for_editor_settings(cx)
        });

        Vim::sync_vim_settings_to_editor(&state, editor, window, cx);

        editor.register_addon(VimAddon {
            entity: vim.clone(),
        });

        vim.update(cx, |_, cx| {
            Vim::action(editor, cx, |vim, _: &SwitchToNormalMode, window, cx| {
                vim.switch_mode(Mode::Normal, false, window, cx)
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
                    vim.switch_mode(Mode::HelixNormal, true, window, cx)
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
                            opening: false,
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
                    vim.update_editor(cx, |_, editor, cx| {
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
            Vim::action(editor, cx, |vim, _: &PushHelixMatch, window, cx| {
                vim.push_operator(Operator::HelixMatch, window, cx)
            });
            Vim::action(editor, cx, |vim, action: &PushHelixNext, window, cx| {
                vim.push_operator(
                    Operator::HelixNext {
                        around: action.around,
                    },
                    window,
                    cx,
                );
            });
            Vim::action(editor, cx, |vim, action: &PushHelixPrevious, window, cx| {
                vim.push_operator(
                    Operator::HelixPrevious {
                        around: action.around,
                    },
                    window,
                    cx,
                );
            });

            Vim::action(
                editor,
                cx,
                |vim, _: &editor::actions::Paste, window, cx| match vim.mode {
                    Mode::Replace => vim.paste_replace(window, cx),
                    Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                        vim.selected_register.replace('+');
                        vim.paste(&VimPaste::default(), window, cx);
                    }
                    _ => {
                        vim.update_editor(cx, |_, editor, cx| editor.paste(&Paste, window, cx));
                    }
                },
            );

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

            if editor.is_focused(window) {
                cx.defer_in(window, |vim, window, cx| {
                    vim.focused(false, window, cx);
                })
            }
        })
    }

    fn deactivate(editor: &mut Editor, cx: &mut Context<Editor>) {
        editor.set_cursor_shape(
            EditorSettings::get_global(cx)
                .cursor_shape
                .unwrap_or_default(),
            cx,
        );
        editor.set_clip_at_line_ends(false, cx);
        editor.set_collapse_matches(false);
        editor.set_input_enabled(true);
        editor.set_autoindent(true);
        editor.selections.set_line_mode(false);
        editor.unregister_addon::<VimAddon>();
        editor.set_relative_line_number(None, cx);
        if let Some(vim) = Vim::globals(cx).focused_vim()
            && vim.entity_id() == cx.entity().entity_id()
        {
            Vim::globals(cx).focused_vim = None;
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
            if let Some(action) = keystroke_event.action.as_ref()
                && action.as_any().downcast_ref::<TemporaryNormal>().is_some()
            {
                return;
            }
            self.switch_mode(Mode::Insert, false, window, cx)
        }
        if let Some(action) = keystroke_event.action.as_ref() {
            // Keystroke is handled by the vim system, so continue forward
            if action.name().starts_with("vim::") {
                self.update_editor(cx, |_, editor, cx| {
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
                self.update_editor(cx, |vim, editor, cx| {
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
            self.update_editor(cx, |_, editor, _| {
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
            self.update_editor(cx, |vim, editor, cx| {
                let is_relative = vim.mode != Mode::Insert;
                editor.set_relative_line_number(Some(is_relative), cx)
            });
        }
        if HelixModeSetting::get_global(cx).0 {
            if self.mode == Mode::Normal {
                self.mode = Mode::HelixNormal
            } else if self.mode == Mode::Visual {
                self.mode = Mode::HelixSelect
            }
        }

        if leave_selections {
            return;
        }

        if !mode.is_visual() && last_mode.is_visual() {
            self.create_visual_marks(last_mode, window, cx);
        }

        // Adjust selections
        self.update_editor(cx, |vim, editor, cx| {
            if last_mode != Mode::VisualBlock && last_mode.is_visual() && mode == Mode::VisualBlock
            {
                vim.visual_block_motion(true, editor, window, cx, |_, point, goal| {
                    Some((point, goal))
                })
            }
            if (last_mode == Mode::Insert || last_mode == Mode::Replace)
                && let Some(prior_tx) = prior_tx
            {
                editor.group_until_transaction(prior_tx, cx)
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

                let mut should_extend_pending = false;
                if !last_mode.is_visual()
                    && mode.is_visual()
                    && let Some(pending) = s.pending_anchor()
                {
                    let snapshot = s.display_snapshot();
                    let is_empty = pending
                        .start
                        .cmp(&pending.end, &snapshot.buffer_snapshot())
                        .is_eq();
                    should_extend_pending = pending.reversed
                        && !is_empty
                        && vim.extended_pending_selection_id != Some(pending.id);
                };

                if should_extend_pending {
                    let snapshot = s.display_snapshot();
                    s.change_with(&snapshot, |map| {
                        if let Some(pending) = map.pending_anchor_mut() {
                            let end = pending.end.to_point(&snapshot.buffer_snapshot());
                            let end = end.to_display_point(&snapshot);
                            let new_end = movement::right(&snapshot, end);
                            pending.end = snapshot
                                .buffer_snapshot()
                                .anchor_before(new_end.to_point(&snapshot));
                        }
                    });
                    vim.extended_pending_selection_id = s.pending_anchor().map(|p| p.id)
                }

                s.move_with(|map, selection| {
                    if last_mode.is_visual() && !mode.is_visual() {
                        let mut point = selection.head();
                        if !selection.reversed && !selection.is_empty() {
                            point = movement::left(map, selection.head());
                        } else if selection.is_empty() {
                            point = map.clip_point(point, Bias::Left);
                        }
                        selection.collapse_to(point, selection.goal)
                    } else if !last_mode.is_visual() && mode.is_visual() {
                        if selection.is_empty() {
                            selection.end = movement::right(map, selection.start);
                        }
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
            global_state.recording_count = count;
        }
        count
    }

    pub fn take_forced_motion(cx: &mut App) -> bool {
        let global_state = cx.global_mut::<VimGlobals>();
        let forced_motion = global_state.forced_motion;
        global_state.forced_motion = false;
        forced_motion
    }

    pub fn cursor_shape(&self, cx: &App) -> CursorShape {
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
                    cursor_shape.normal
                }
            }
            Mode::HelixNormal => cursor_shape.normal,
            Mode::Replace => cursor_shape.replace,
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock | Mode::HelixSelect => {
                cursor_shape.visual
            }
            Mode::Insert => match cursor_shape.insert {
                InsertModeCursorShape::Explicit(shape) => shape,
                InsertModeCursorShape::Inherit => {
                    let editor_settings = EditorSettings::get_global(cx);
                    editor_settings.cursor_shape.unwrap_or_default()
                }
            },
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
            | Mode::VisualBlock
            | Mode::HelixSelect => false,
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
            | Mode::HelixNormal
            | Mode::HelixSelect => false,
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
            Mode::HelixSelect => "helix_select",
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

        if mode == "normal"
            || mode == "visual"
            || mode == "operator"
            || mode == "helix_normal"
            || mode == "helix_select"
        {
            context.add("VimControl");
        }
        context.set("vim_mode", mode);
        context.set("vim_operator", operator_id);
    }

    fn focused(&mut self, preserve_selection: bool, window: &mut Window, cx: &mut Context<Self>) {
        // If editor gains focus while search bar is still open (not dismissed),
        // the user has explicitly navigated away - clear prior_selections so we
        // don't restore to the old position if they later dismiss the search.
        if !self.search.prior_selections.is_empty() {
            if let Some(pane) = self.pane(window, cx) {
                let search_still_open = pane
                    .read(cx)
                    .toolbar()
                    .read(cx)
                    .item_of_type::<BufferSearchBar>()
                    .is_some_and(|bar| !bar.read(cx).is_dismissed());
                if search_still_open {
                    self.search.prior_selections.clear();
                }
            }
        }

        let Some(editor) = self.editor() else {
            return;
        };
        let newest_selection_empty = editor.update(cx, |editor, cx| {
            editor
                .selections
                .newest::<MultiBufferOffset>(&editor.display_snapshot(cx))
                .is_empty()
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
                self.update_editor(cx, |_, editor, cx| {
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
                        vim.update_editor(cx, |_, editor, cx| {
                            editor.set_relative_line_number(None, cx)
                        });
                    });

                    self.update_editor(cx, |vim, editor, cx| {
                        let is_relative = vim.mode != Mode::Insert;
                        editor.set_relative_line_number(Some(is_relative), cx)
                    });
                }
            } else {
                self.update_editor(cx, |vim, editor, cx| {
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
        self.update_editor(cx, |vim, editor, cx| {
            if vim.cursor_shape(cx) == CursorShape::Block {
                editor.set_cursor_shape(CursorShape::Hollow, cx);
            }
        });
    }

    fn cursor_shape_changed(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(cx, |vim, editor, cx| {
            editor.set_cursor_shape(vim.cursor_shape(cx), cx);
        });
    }

    fn update_editor<S>(
        &mut self,
        cx: &mut Context<Self>,
        update: impl FnOnce(&mut Self, &mut Editor, &mut Context<Editor>) -> S,
    ) -> Option<S> {
        let editor = self.editor.upgrade()?;
        Some(editor.update(cx, |editor, cx| update(self, editor, cx)))
    }

    fn editor_selections(&mut self, _: &mut Window, cx: &mut Context<Self>) -> Vec<Range<Anchor>> {
        self.update_editor(cx, |_, editor, _| {
            editor
                .selections
                .disjoint_anchors_arc()
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
        self.update_editor(cx, |_, editor, cx| {
            let snapshot = &editor.snapshot(window, cx);
            let selection = editor
                .selections
                .newest::<MultiBufferOffset>(&snapshot.display_snapshot);

            let snapshot = snapshot.buffer_snapshot();
            let (range, kind) =
                snapshot.surrounding_word(selection.start, Some(CharScopeContext::Completion));
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
                globals.recording_count = None;

                let selections = self.editor().map(|editor| {
                    editor.update(cx, |editor, cx| {
                        let snapshot = editor.display_snapshot(cx);

                        (
                            editor.selections.oldest::<Point>(&snapshot),
                            editor.selections.newest::<Point>(&snapshot),
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
            globals.recorded_count = globals.recording_count.take();
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
                    .filter(|post_count| *post_count < isize::MAX as usize)
                    .unwrap_or(post_count),
            )
        } else {
            let pre_count = Vim::globals(cx).pre_count.unwrap_or(0);

            Vim::globals(cx).pre_count = Some(
                pre_count
                    .checked_mul(10)
                    .and_then(|pre_count| pre_count.checked_add(number))
                    .filter(|pre_count| *pre_count < isize::MAX as usize)
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
            Mode::VisualLine | Mode::VisualBlock | Mode::Visual | Mode::HelixSelect => {
                self.update_editor(cx, |vim, editor, cx| {
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
                self.update_editor(cx, |_, editor, cx| {
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
            if let Some(current_anchor) = &self.current_anchor {
                if current_anchor != &newest
                    && let Some(tx_id) = self.current_tx.take()
                {
                    self.update_editor(cx, |_, editor, cx| {
                        editor.group_until_transaction(tx_id, cx)
                    });
                }
            } else {
                self.current_anchor = Some(newest);
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
            self.switch_mode(Mode::Normal, false, window, cx);
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
                        Vim::globals(cx).last_find = Some(sneak.clone());
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
                        Vim::globals(cx).last_find = Some(sneak.clone());
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
            Some(Operator::ChangeSurrounds { target, opening }) => match self.mode {
                Mode::Normal => {
                    if let Some(target) = target {
                        self.change_surrounds(text, target, opening, window, cx);
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
            Some(Operator::HelixSurroundAdd) => match self.mode {
                Mode::HelixNormal | Mode::HelixSelect => {
                    self.update_editor(cx, |_, editor, cx| {
                        editor.change_selections(Default::default(), window, cx, |s| {
                            s.move_with(|map, selection| {
                                if selection.is_empty() {
                                    selection.end = movement::right(map, selection.start);
                                }
                            });
                        });
                    });
                    self.helix_surround_add(&text, window, cx);
                    self.switch_mode(Mode::HelixNormal, false, window, cx);
                    self.clear_operator(window, cx);
                }
                _ => self.clear_operator(window, cx),
            },
            Some(Operator::HelixSurroundReplace {
                replaced_char: Some(old),
            }) => match self.mode {
                Mode::HelixNormal | Mode::HelixSelect => {
                    if let Some(new_char) = text.chars().next() {
                        self.helix_surround_replace(old, new_char, window, cx);
                    }
                    self.clear_operator(window, cx);
                }
                _ => self.clear_operator(window, cx),
            },
            Some(Operator::HelixSurroundReplace {
                replaced_char: None,
            }) => match self.mode {
                Mode::HelixNormal | Mode::HelixSelect => {
                    if let Some(ch) = text.chars().next() {
                        self.pop_operator(window, cx);
                        self.push_operator(
                            Operator::HelixSurroundReplace {
                                replaced_char: Some(ch),
                            },
                            window,
                            cx,
                        );
                    }
                }
                _ => self.clear_operator(window, cx),
            },
            Some(Operator::HelixSurroundDelete) => match self.mode {
                Mode::HelixNormal | Mode::HelixSelect => {
                    if let Some(ch) = text.chars().next() {
                        self.helix_surround_delete(ch, window, cx);
                    }
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
                    self.update_editor(cx, |_, editor, cx| {
                        if let Some(register) = Vim::update_globals(cx, |globals, cx| {
                            globals.read_register(text.chars().next(), Some(editor), cx)
                        }) {
                            editor.do_paste(
                                &register.text.to_string(),
                                register.clipboard_selections,
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
                    self.update_editor(cx, |_, editor, cx| {
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
        let state = self.state_for_editor_settings(cx);
        self.update_editor(cx, |_, editor, cx| {
            Vim::sync_vim_settings_to_editor(&state, editor, window, cx);
        });
        cx.notify()
    }

    fn state_for_editor_settings(&self, cx: &App) -> VimEditorSettingsState {
        VimEditorSettingsState {
            cursor_shape: self.cursor_shape(cx),
            clip_at_line_ends: self.clip_at_line_ends(),
            collapse_matches: !HelixModeSetting::get_global(cx).0,
            input_enabled: self.editor_input_enabled(),
            autoindent: self.should_autoindent(),
            cursor_offset_on_selection: self.mode.is_visual(),
            line_mode: matches!(self.mode, Mode::VisualLine),
            hide_edit_predictions: !matches!(self.mode, Mode::Insert | Mode::Replace),
        }
    }

    fn sync_vim_settings_to_editor(
        state: &VimEditorSettingsState,
        editor: &mut Editor,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        editor.set_cursor_shape(state.cursor_shape, cx);
        editor.set_clip_at_line_ends(state.clip_at_line_ends, cx);
        editor.set_collapse_matches(state.collapse_matches);
        editor.set_input_enabled(state.input_enabled);
        editor.set_autoindent(state.autoindent);
        editor.set_cursor_offset_on_selection(state.cursor_offset_on_selection);
        editor.selections.set_line_mode(state.line_mode);
        editor.set_edit_predictions_hidden_for_vim_mode(state.hide_edit_predictions, window, cx);
    }
}

struct VimEditorSettingsState {
    cursor_shape: CursorShape,
    clip_at_line_ends: bool,
    collapse_matches: bool,
    input_enabled: bool,
    autoindent: bool,
    cursor_offset_on_selection: bool,
    line_mode: bool,
    hide_edit_predictions: bool,
}

#[derive(Clone, RegisterSetting)]
struct VimSettings {
    pub default_mode: Mode,
    pub toggle_relative_line_numbers: bool,
    pub use_system_clipboard: settings::UseSystemClipboard,
    pub use_smartcase_find: bool,
    pub gdefault: bool,
    pub custom_digraphs: HashMap<String, Arc<str>>,
    pub highlight_on_yank_duration: u64,
    pub cursor_shape: CursorShapeSettings,
}

/// Cursor shape configuration for insert mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InsertModeCursorShape {
    /// Inherit cursor shape from the editor's base cursor_shape setting.
    /// This allows users to set their preferred editor cursor and have
    /// it automatically apply to vim insert mode.
    Inherit,
    /// Use an explicit cursor shape for insert mode.
    Explicit(CursorShape),
}

/// The settings for cursor shape.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CursorShapeSettings {
    /// Cursor shape for the normal mode.
    ///
    /// Default: block
    pub normal: CursorShape,
    /// Cursor shape for the replace mode.
    ///
    /// Default: underline
    pub replace: CursorShape,
    /// Cursor shape for the visual mode.
    ///
    /// Default: block
    pub visual: CursorShape,
    /// Cursor shape for the insert mode.
    ///
    /// Default: Inherit (follows editor.cursor_shape)
    pub insert: InsertModeCursorShape,
}

impl From<settings::VimInsertModeCursorShape> for InsertModeCursorShape {
    fn from(shape: settings::VimInsertModeCursorShape) -> Self {
        match shape {
            settings::VimInsertModeCursorShape::Inherit => InsertModeCursorShape::Inherit,
            settings::VimInsertModeCursorShape::Bar => {
                InsertModeCursorShape::Explicit(CursorShape::Bar)
            }
            settings::VimInsertModeCursorShape::Block => {
                InsertModeCursorShape::Explicit(CursorShape::Block)
            }
            settings::VimInsertModeCursorShape::Underline => {
                InsertModeCursorShape::Explicit(CursorShape::Underline)
            }
            settings::VimInsertModeCursorShape::Hollow => {
                InsertModeCursorShape::Explicit(CursorShape::Hollow)
            }
        }
    }
}

impl From<settings::CursorShapeSettings> for CursorShapeSettings {
    fn from(settings: settings::CursorShapeSettings) -> Self {
        Self {
            normal: settings.normal.unwrap().into(),
            replace: settings.replace.unwrap().into(),
            visual: settings.visual.unwrap().into(),
            insert: settings.insert.unwrap().into(),
        }
    }
}

impl From<settings::ModeContent> for Mode {
    fn from(mode: ModeContent) -> Self {
        match mode {
            ModeContent::Normal => Self::Normal,
            ModeContent::Insert => Self::Insert,
        }
    }
}

impl Settings for VimSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let vim = content.vim.clone().unwrap();
        Self {
            default_mode: vim.default_mode.unwrap().into(),
            toggle_relative_line_numbers: vim.toggle_relative_line_numbers.unwrap(),
            use_system_clipboard: vim.use_system_clipboard.unwrap(),
            use_smartcase_find: vim.use_smartcase_find.unwrap(),
            gdefault: vim.gdefault.unwrap(),
            custom_digraphs: vim.custom_digraphs.unwrap(),
            highlight_on_yank_duration: vim.highlight_on_yank_duration.unwrap(),
            cursor_shape: vim.cursor_shape.unwrap().into(),
        }
    }
}
