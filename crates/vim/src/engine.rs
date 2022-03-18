use editor::{self, CursorShape, GoToDefinition};
use gpui::{keymap::Keystroke, AnyAction};
use workspace::{self, pane};

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Normal,
    Visual,
    VisualLine,
    Replace,
    Insert,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    None,
    Delete,
    Change,
    GPrefix,
}

impl Default for Operation {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Motion {
    Left,
    Right,
    Down,
    Up,
    StartOfLine,
    EndOfLine,
    NextWord { ignore_punctuation: bool },
    PreviousWord { ignore_punctuation: bool },
    EndOfWord { ignore_punctuation: bool },
    StartOfDocument,
    EndOfDocument,
}

#[derive(Debug)]
pub enum Region {
    Selection,
    SelectionLines,
    FromCursor(Motion),
    CurrentLine,
}

pub enum Effect {
    Move(Motion),
    Select(Motion),
    Delete(Region),
    ReplaceWithCharacter(String),
    NewLine { above: bool },
    SwapHead,
    ClearSelection,
    EditorAction(Box<dyn AnyAction>),
    WorkspaceAction(Box<dyn AnyAction>),
    ModeChanged(Mode),
}

pub struct EngineOutput {
    pub effects: Vec<Effect>,
    pub should_consume_keystroke: bool,
}

#[derive(Default)]
pub struct VimEngine {
    pub mode: Mode,
    pub pending: Operation,

    queued_effects: Vec<Effect>,
}

impl VimEngine {
    pub fn current_cursor_shape(&self) -> CursorShape {
        match self.mode {
            Mode::Normal => CursorShape::Block,
            Mode::Visual => CursorShape::Block,
            Mode::VisualLine => CursorShape::Block,
            Mode::Insert => CursorShape::Bar,
            Mode::Replace => CursorShape::Underscore,
        }
    }

    fn motion_for_keystroke(&self, keystroke: &str) -> Option<Motion> {
        if let Operation::GPrefix = self.pending {
            match keystroke {
                "g" => Some(Motion::StartOfDocument),
                _ => None,
            }
        } else {
            match keystroke {
                "h" => Some(Motion::Left),
                "l" => Some(Motion::Right),
                "j" => Some(Motion::Down),
                "k" => Some(Motion::Up),
                "0" => Some(Motion::StartOfLine),
                "S-$" => Some(Motion::EndOfLine),
                "w" => Some(Motion::NextWord {
                    ignore_punctuation: false,
                }),
                "W" => Some(Motion::NextWord {
                    ignore_punctuation: true,
                }),
                "b" => Some(Motion::PreviousWord {
                    ignore_punctuation: false,
                }),
                "B" => Some(Motion::PreviousWord {
                    ignore_punctuation: true,
                }),
                "e" => Some(Motion::EndOfWord {
                    ignore_punctuation: false,
                }),
                "E" => Some(Motion::EndOfWord {
                    ignore_punctuation: true,
                }),
                "G" => Some(Motion::EndOfDocument),
                _ => None,
            }
        }
    }

    fn queue(&mut self, effect: Effect) {
        self.queued_effects.push(effect);
    }

    fn editor_action(&mut self, action: impl AnyAction + 'static) {
        self.queued_effects
            .push(Effect::EditorAction(Box::new(action)));
    }

    fn workspace_action(&mut self, action: impl AnyAction + 'static) {
        self.queued_effects
            .push(Effect::WorkspaceAction(Box::new(action)));
    }

    pub fn switch_mode(&mut self, new_mode: Mode) {
        self.mode = new_mode;
        self.queued_effects.push(Effect::ModeChanged(new_mode));
        self.pending = Operation::None;
    }

    pub fn handle_keystroke(&mut self, keystroke: &Keystroke) -> EngineOutput {
        let mut should_consume_keystroke = true;
        if keystroke.cmd {
            // No vim bindings use cmd
            should_consume_keystroke = false;
        } else {
            let keystroke_text = keystroke.to_string();
            let motion = self.motion_for_keystroke(&keystroke_text);
            match self.mode {
                Mode::Normal => match self.pending {
                    Operation::None => {
                        if let Some(motion) = motion {
                            self.pending = Operation::None;
                            self.queue(Effect::Move(motion));
                        } else {
                            match keystroke_text.as_ref() {
                                "c" => self.pending = Operation::Change,
                                "d" => self.pending = Operation::Delete,
                                "g" => self.pending = Operation::GPrefix,
                                "x" => self.queue(Effect::Delete(Region::Selection)),
                                "i" => self.switch_mode(Mode::Insert),
                                "a" => {
                                    self.switch_mode(Mode::Insert);
                                    self.queue(Effect::Move(Motion::Right));
                                }
                                "A" => {
                                    self.queue(Effect::Move(Motion::EndOfLine));
                                    self.switch_mode(Mode::Insert);
                                }
                                "I" => {
                                    self.queue(Effect::Move(Motion::StartOfLine));
                                    self.switch_mode(Mode::Insert);
                                }
                                "r" => self.switch_mode(Mode::Replace),
                                "v" => self.switch_mode(Mode::Visual),
                                "V" => self.switch_mode(Mode::VisualLine),
                                "o" => {
                                    self.queue(Effect::NewLine { above: false });
                                    self.switch_mode(Mode::Insert);
                                }
                                "O" => {
                                    self.queue(Effect::NewLine { above: true });
                                    self.switch_mode(Mode::Insert);
                                }
                                "u" => self.editor_action(editor::Undo),
                                "C-r" => self.editor_action(editor::Redo),
                                "C-o" => self.workspace_action(pane::GoBack(None)),
                                "escape" => should_consume_keystroke = false,
                                _ => self.pending = Operation::None,
                            }
                        }
                    }
                    Operation::Change => {
                        self.pending = Operation::None;

                        if let Some(motion) = motion {
                            self.queue(Effect::Delete(Region::FromCursor(motion)));
                        } else if keystroke_text == "d" {
                            self.queue(Effect::Delete(Region::CurrentLine));
                        }
                    }
                    Operation::Delete => {
                        self.pending = Operation::None;

                        if let Some(motion) = motion {
                            self.queue(Effect::Delete(Region::FromCursor(motion)))
                        } else if keystroke_text == "d" {
                            self.queue(Effect::Delete(Region::CurrentLine))
                        }
                    }
                    Operation::GPrefix => {
                        self.pending = Operation::None;
                        match keystroke_text.as_ref() {
                            "g" => self.queue(Effect::Move(Motion::StartOfDocument)),
                            "d" => self.editor_action(GoToDefinition),
                            _ => {}
                        }
                    }
                },
                Mode::Visual | Mode::VisualLine => {
                    if let Some(motion) = motion {
                        self.pending = Operation::None;
                        self.queue(Effect::Select(motion));
                    } else {
                        match self.pending {
                            Operation::None => {
                                match keystroke_text.as_ref() {
                                    "g" => self.pending = Operation::GPrefix,
                                    "c" => {
                                        self.queue(Effect::Delete(Region::Selection));
                                        self.switch_mode(Mode::Insert);
                                    }
                                    "d" | "x" => {
                                        if matches!(self.mode, Mode::Visual) {
                                            self.queue(Effect::Delete(Region::Selection));
                                        } else {
                                            // VisualLines
                                            self.queue(Effect::Delete(Region::SelectionLines));
                                        }
                                        self.switch_mode(Mode::Normal);
                                    }
                                    "r" => self.switch_mode(Mode::Replace),
                                    "o" | "O" => self.queue(Effect::SwapHead),
                                    // Input::Text("u", _) => change region case,
                                    "escape" => {
                                        self.switch_mode(Mode::Normal);
                                        self.queue(Effect::ClearSelection);
                                    }
                                    _ => {}
                                }
                            }
                            _ => self.pending = Operation::None,
                        }
                    }
                }
                Mode::Replace => {
                    self.switch_mode(Mode::Normal);
                    if !keystroke.ctrl
                        && !keystroke.alt
                        && !keystroke.cmd
                        && keystroke_text.len() == 0
                    {
                        self.queue(Effect::ReplaceWithCharacter(keystroke_text.to_owned()));
                    }
                }
                Mode::Insert => match keystroke_text.as_ref() {
                    "escape" => {
                        self.queue(Effect::Move(Motion::Left));
                        self.switch_mode(Mode::Normal);
                    }
                    _ => should_consume_keystroke = false,
                },
            }
        }

        let mut effects = Vec::new();
        std::mem::swap(&mut effects, &mut self.queued_effects);

        EngineOutput {
            effects,
            should_consume_keystroke,
        }
    }
}
