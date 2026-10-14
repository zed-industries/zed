mod bencode;
mod client;
mod completion;
mod console;
mod discovery;
mod editor_session;
mod form_at_cursor;
mod jack_in;
mod navigation;
mod nrepl_sessions_ui;
mod nrepl_settings;
mod nrepl_store;
mod ops;
mod output_view;

use std::sync::Arc;

use gpui::App;
use project::Fs;
use settings::Settings;

pub use crate::bencode::{Value, dict};
pub use crate::client::{NreplClient, RequestStream};
pub use crate::completion::NreplCompletionProvider;
pub use crate::console::NreplConsole;
pub use crate::discovery::{DiscoveredPort, discover_port, discover_port_in};
pub use crate::editor_session::{
    NreplEditorSession, clear_outputs, eval_form_at_cursor, eval_selection, forget_editor,
    interrupt, load_file, switch_namespace, toast_workspace, workspace_for_editor,
};
pub use crate::form_at_cursor::{
    TopLevelForm, ns_form_text, parse_namespace, top_level_form_at_offset,
};
pub use crate::jack_in::{JackInCommand, ProjectKind, detect_project};
pub use crate::nrepl_sessions_ui::{
    ClearOutputs, Connect, Disconnect, Eval, EvalAndComment, EvalBuffer, EvalCurrentForm,
    EvalSelection, EvalToCursor, GoToDefinition, Interrupt, JackIn, LoadFile, NreplSessionsPage,
    OpenRepl, RequireReload, RequireReloadAll, SendToRepl, Sessions, ShowDocumentation,
    SwitchNamespace,
};
pub use crate::nrepl_settings::NreplSettings;
pub use crate::nrepl_store::{ConnectTarget, ConnectionState, NreplConnection, NreplStore};
pub use crate::output_view::{OutputChunk, OutputStatus, OutputView};

pub fn init(fs: Arc<dyn Fs>, cx: &mut App) {
    NreplSettings::register(cx);
    NreplStore::init(fs, cx);
    editor_session::init(cx);
    nrepl_sessions_ui::init(cx);
}
