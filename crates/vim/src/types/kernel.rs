use super::{helix_engine::*, shared::*, vim_engine::*};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ModalEngineState {
    pub(crate) shared: SharedModalState,
    pub(crate) engine: ModalEngineVariant,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ModalEngineVariant {
    Vim(VimEngineState),
    Helix(HelixEngineState),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SharedModalState {
    pub(crate) count: CountState,
    pub(crate) registers: RegisterStoreSnapshot,
    pub(crate) marks: MarkStoreSnapshot,
    pub(crate) recording: RecordingState,
    pub(crate) recorded_selection: RecordedSelection,
    pub(crate) search: SearchStateSnapshot<ModalPendingCommand>,
    pub(crate) transactions: TransactionState,
    pub(crate) replacements: Vec<ReplacementRecord>,
    pub(crate) status_label: Option<CommandName>,
    pub(crate) last_yank: Option<KeyText>,
    pub(crate) last_find: Option<ModalMotion>,
    pub(crate) last_command: Option<CommandName>,
    pub(crate) change_list: ChangeListSnapshot,
    pub(crate) jump_list: JumpListSnapshot,
    pub(crate) selection_history: SelectionHistorySnapshot,
    pub(crate) extended_pending_selection_id: Option<SelectionId>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ModalCommand {
    Vim(VimCommand),
    Helix(HelixCommand),
    Shared(SharedCommand),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SharedCommand {
    Cancel,
    PushCount(Count),
    SelectRegister(RegisterScope),
    OpenCommandPalette,
    OpenSearch,
    Rename,
    GoToDefinition,
    Hover,
    ToggleComment,
    SaveLocation,
    PushDigraph { first_char: Option<char> },
    PushLiteral { prefix: Option<KeyText> },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SharedPendingCommand {
    Digraph { first_char: Option<char> },
    Literal { prefix: Option<KeyText> },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ModalPendingCommand {
    Vim(VimPendingCommand),
    Helix(HelixPendingCommand),
    Shared(SharedPendingCommand),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ModalMotion {
    Vim(VimMotion),
    Helix(HelixMovement),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ModalEngineOutput {
    Vim(VimEngineOutput),
    Helix(HelixEngineOutput),
    Shared(SharedOutput),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SharedOutput {
    pub(crate) selections: Option<SelectionSnapshot>,
    pub(crate) settings: Option<HostEditorSettings>,
    pub(crate) ui_feedback: UiFeedback,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EngineSwitch {
    pub(crate) from: EngineKind,
    pub(crate) to: EngineKind,
    pub(crate) preserve_selection: bool,
    pub(crate) target_mode: Option<ModalMode>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ModalMode {
    Vim(VimMode),
    Helix(HelixMode),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HostEditorSettings {
    pub(crate) cursor_shapes: ModalCursorShapes,
    pub(crate) cursor_semantics: CursorSemantics,
    pub(crate) point_command_semantics: PointCommandSemantics,
    pub(crate) selection_storage: SelectionStorage,
    pub(crate) clip_at_line_ends: bool,
    pub(crate) collapse_matches: bool,
    pub(crate) autoindent: bool,
    pub(crate) line_mode: bool,
    pub(crate) input_enabled: bool,
    pub(crate) expects_character_input: bool,
    pub(crate) hide_edit_predictions: bool,
    pub(crate) default_mode: ModalMode,
    pub(crate) use_system_clipboard: UseSystemClipboardPolicy,
    pub(crate) use_smartcase_find: bool,
    pub(crate) use_regex_search: bool,
    pub(crate) gdefault: bool,
    pub(crate) custom_digraphs: Vec<(KeyText, KeyText)>,
    pub(crate) highlight_on_yank_duration_ms: u64,
    pub(crate) toggle_relative_line_numbers: bool,
    pub(crate) helix_mode: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PointCommandRequest {
    pub(crate) command: SharedCommand,
    pub(crate) semantics: PointCommandSemantics,
    pub(crate) selection: SelectionSnapshot,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct KernelBoundaryRequest {
    pub(crate) engine: EngineKind,
    pub(crate) state: ModalEngineState,
    pub(crate) command: ModalCommand,
    pub(crate) settings: HostEditorSettings,
}
