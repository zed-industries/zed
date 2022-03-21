mod engine;

use collections::HashMap;
use editor::{
    char_kind, display_map::DisplaySnapshot, movement, CharKind, DeleteLine, DisplayPoint, Editor,
    EditorBlurred, EditorCreated, EditorFocused, EditorMode, EditorReleased, Newline,
};
use engine::*;
use gpui::{keymap::Keystroke, MutableAppContext, ViewContext, ViewHandle, WeakViewHandle};
use language::{CharKindOptions, SelectionGoal};
use workspace::{self, Settings, Workspace};

pub fn init(cx: &mut MutableAppContext) {
    cx.hook_keystrokes(VimState::handle_keystroke);
    cx.subscribe_global(VimState::editor_created).detach();
    cx.subscribe_global(VimState::editor_focused).detach();
    cx.subscribe_global(VimState::editor_blurred).detach();
    cx.subscribe_global(VimState::editor_released).detach();
    cx.observe_global::<Settings, _>(VimState::settings_changed)
        .detach();
}

#[derive(Default)]
pub struct VimState {
    editors: HashMap<usize, WeakViewHandle<Editor>>,
    active_editor: Option<WeakViewHandle<Editor>>,

    enabled: bool,
    engine: VimEngine,
}

impl VimState {
    fn handle_keystroke(
        window_id: usize,
        keystroke: &Keystroke,
        cx: &mut MutableAppContext,
    ) -> bool {
        if !cx.global::<Self>().enabled {
            // Don't consume keystrokes if not enabled
            return false;
        }

        if let Some(workspace) = cx.root_view(window_id) {
            if let Some(active_editor) = cx.global::<Self>().active_editor.clone() {
                if let Some(active_editor) = active_editor.upgrade(cx) {
                    let engine_output = cx.update_default_global(|this: &mut VimState, _| {
                        this.engine.handle_keystroke(keystroke)
                    });

                    VimState::execute_engine_effects(
                        engine_output.effects,
                        active_editor,
                        workspace,
                        cx,
                    );

                    return engine_output.should_consume_keystroke;
                }
            }
        }
        false
    }

    fn editor_created(EditorCreated(editor): &EditorCreated, cx: &mut MutableAppContext) {
        cx.update_default_global(|this: &mut Self, cx| {
            this.editors.insert(editor.id(), editor.downgrade());
            if this.enabled {
                Self::update_cursor_shapes(cx);
            }
        })
    }

    fn editor_focused(EditorFocused(editor): &EditorFocused, cx: &mut MutableAppContext) {
        cx.update_default_global(|this: &mut Self, cx| {
            if this.enabled {
                if matches!(editor.read(cx).mode(), EditorMode::SingleLine) {
                    this.engine.mode = Mode::Insert;
                } else {
                    this.engine.mode = Mode::Normal;
                }

                Self::update_cursor_shapes(cx);
            }

            this.active_editor = Some(editor.downgrade());
        })
    }

    fn editor_blurred(EditorBlurred(editor): &EditorBlurred, cx: &mut MutableAppContext) {
        cx.update_default_global(|this: &mut Self, _| {
            if let Some(previous_editor) = this.active_editor.clone() {
                if previous_editor == editor.clone() {
                    this.active_editor = None;
                }
            }
        })
    }

    fn editor_released(EditorReleased(editor): &EditorReleased, cx: &mut MutableAppContext) {
        cx.update_default_global(|this: &mut Self, _| {
            this.editors.remove(&editor.id());
            if let Some(previous_editor) = this.active_editor.clone() {
                if previous_editor == editor.clone() {
                    this.active_editor = None;
                }
            }
        });
    }

    fn settings_changed(cx: &mut MutableAppContext) -> bool {
        cx.update_default_global(|this: &mut Self, cx| {
            let settings = cx.global::<Settings>();
            if this.enabled != settings.vim_mode {
                this.enabled = settings.vim_mode;
                this.engine.mode = if settings.vim_mode {
                    Mode::Normal
                } else {
                    Mode::Insert
                };
                Self::update_cursor_shapes(cx);
            }
        });

        true
    }

    fn execute_engine_effects(
        effects: Vec<Effect>,
        editor: ViewHandle<Editor>,
        workspace: ViewHandle<Workspace>,
        cx: &mut MutableAppContext,
    ) {
        let mut current_mode = cx.global::<VimState>().engine.mode;
        for effect in effects {
            match effect {
                Effect::Move(motion) => editor.update(cx, |editor, cx| {
                    editor.move_cursors(cx, |map, cursor, goal| {
                        Self::apply_motion_to_point(map, cursor, goal, motion)
                    })
                }),
                Effect::Select(motion) => editor.update(cx, |editor, cx| {
                    editor.move_selection_heads(cx, |map, cursor, goal| {
                        Self::apply_motion_to_point(map, cursor, goal, motion)
                    })
                }),
                Effect::Delete(region) => editor.update(cx, |editor, cx| match region {
                    Region::Selection | Region::SelectionLines => {
                        editor.transact(cx, |editor, cx| {
                            editor.move_selection_heads(cx, |map, head, goal| {
                                Self::apply_motion_to_point(map, head, goal, Motion::Right)
                            });
                            editor.insert("", cx);
                        })
                    }
                    Region::FromCursor(motion) => editor.transact(cx, |editor, cx| {
                        editor.move_selection_heads(cx, |map, head, goal| {
                            Self::apply_motion_to_point(map, head, goal, motion)
                        });
                        editor.insert("", cx);
                    }),
                    Region::CurrentLine => editor.delete_line(&DeleteLine, cx),
                }),
                Effect::ReplaceWithCharacter(character) => editor.update(cx, |editor, cx| {
                    // TODO: This currently ignores replacing all characters in selection
                    // and just replaces all selections with a single character
                    editor.transact(cx, |editor, cx| {
                        editor.move_selection_heads(cx, |map, head, goal| {
                            Self::apply_motion_to_point(map, head, goal, Motion::Right)
                        });
                        editor.insert(&character, cx);
                    })
                }),
                Effect::NewLine { above: false } => editor.update(cx, |editor, cx| {
                    editor.move_cursors(cx, |map, cursor, goal| {
                        Self::apply_motion_to_point(map, cursor, goal, Motion::EndOfLine)
                    });
                    editor.newline(&Newline, cx);
                }),
                Effect::NewLine { above: true } => editor.update(cx, |editor, cx| {
                    editor.move_cursors(cx, |map, cursor, goal| {
                        Self::apply_motion_to_point(map, cursor, goal, Motion::StartOfLine)
                    });
                    editor.newline(&Newline, cx);
                    editor.move_cursors(cx, |map, cursor, goal| {
                        Self::apply_motion_to_point(map, cursor, goal, Motion::Up)
                    });
                }),
                Effect::SwapHead => editor.update(cx, |editor, cx| {
                    editor.move_selections(cx, |_, selection| {
                        selection.reversed = !selection.reversed
                    })
                }),
                Effect::ClearSelection => editor.update(cx, |editor, cx| {
                    editor.move_selections(cx, |_, selection| {
                        let cursor = selection.head();
                        selection.start = cursor;
                        selection.end = cursor;
                    })
                }),
                Effect::EditorAction(action) => {
                    cx.dispatch_action_any(editor.window_id(), &[editor.id()], action.as_ref());
                }
                Effect::WorkspaceAction(action) => {
                    cx.dispatch_action_any(
                        workspace.window_id(),
                        &[workspace.id()],
                        action.as_ref(),
                    );
                }
                Effect::ModeChanged(new_mode) => {
                    current_mode = new_mode;
                    Self::update_cursor_shapes(cx);
                }
            }
        }

        editor.update(cx, |editor, cx| {
            Self::apply_mode_specific_selection_fixes(current_mode, editor, cx);
        });
    }

    fn apply_motion_to_point(
        map: &DisplaySnapshot,
        point: DisplayPoint,
        goal: SelectionGoal,
        motion: Motion,
    ) -> (DisplayPoint, SelectionGoal) {
        match motion {
            Motion::Left => (movement::left(map, point, false), SelectionGoal::None),
            Motion::Right => (movement::right(map, point, false), SelectionGoal::None),
            Motion::Down => movement::down(map, point, goal),
            Motion::Up => movement::up(map, point, goal),
            Motion::StartOfLine => (
                movement::line_beginning(map, point, false),
                SelectionGoal::None,
            ),
            Motion::EndOfLine => (
                movement::line_end(map, point, false),
                SelectionGoal::Column(u32::MAX),
            ),
            Motion::NextWord { ignore_punctuation } => {
                let char_kind_options = CharKindOptions::new(ignore_punctuation, false);
                let mut point = movement::next_word_start(map, point, char_kind_options);
                let next_char_kind = map
                    .chars_at(point)
                    .next()
                    .map(|c| char_kind(c, char_kind_options));

                if matches!(next_char_kind, Some(CharKind::Whitespace)) {
                    point = movement::next_word_start(map, point, char_kind_options)
                }

                (point, SelectionGoal::None)
            }
            Motion::PreviousWord { ignore_punctuation } => {
                let char_kind_options = CharKindOptions::new(ignore_punctuation, false);
                let mut point = movement::previous_word_start(map, point, ignore_punctuation);
                let next_char_kind = map
                    .chars_at(point)
                    .next()
                    .map(|c| char_kind(c, char_kind_options));

                if matches!(next_char_kind, Some(CharKind::Whitespace)) {
                    point = movement::previous_word_start(map, point, ignore_punctuation)
                }

                (point, SelectionGoal::None)
            }
            Motion::EndOfWord { ignore_punctuation } => {
                let point = movement::right(map, point, false);
                let point = movement::next_word_end(
                    map,
                    point,
                    CharKindOptions::new(ignore_punctuation, false),
                );
                let point = movement::left(map, point, false);
                (point, SelectionGoal::None)
            }
            Motion::StartOfDocument => (DisplayPoint::new(0, 0), SelectionGoal::None),
            Motion::EndOfDocument => (map.max_point(), SelectionGoal::None),
        }
    }

    fn apply_mode_specific_selection_fixes(
        current_mode: Mode,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) {
        if matches!(current_mode, Mode::VisualLine) {
            editor.move_selections(cx, |map, selection| {
                selection.start = movement::line_beginning(map, selection.start, false);
                selection.end = movement::line_end(map, selection.end, false);
                selection.goal = SelectionGoal::None;
                // This isn't quite right, but is a good enough hack for now
            });
        }

        if !matches!(current_mode, Mode::Insert) {
            editor.move_selections(cx, |map, selection| {
                let cursor = movement::left(map, selection.head(), false);

                let should_backtrack = map
                    .chars_at(selection.head())
                    .next()
                    .map(|character| character == '\n')
                    .unwrap_or(true); // No more characters

                if should_backtrack {
                    if selection.start != selection.end {
                        selection.set_head(cursor, SelectionGoal::None);
                    } else {
                        selection.start = cursor;
                        selection.end = cursor;
                    }
                }
                // TODO: Fixup the goal column
            });
        }
    }

    fn update_cursor_shapes(cx: &mut MutableAppContext) {
        cx.defer(move |cx| {
            cx.update_default_global(|this: &mut VimState, cx| {
                let cursor_shape = this.engine.current_cursor_shape();
                for editor in this.editors.values() {
                    if let Some(editor) = editor.upgrade(cx) {
                        editor.update(cx, |editor, cx| {
                            editor.set_cursor_shape(cursor_shape, cx);
                        });
                    }
                }
            });
        });
    }
}
