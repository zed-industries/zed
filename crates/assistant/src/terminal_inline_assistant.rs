use std::sync::Arc;

use client::telemetry::Telemetry;
use collections::{HashMap, VecDeque};
use gpui::{AppContext, Global, View, WeakView};
use terminal_view::TerminalView;
use ui::WindowContext;
use workspace::Workspace;

pub fn init(telemetry: Arc<Telemetry>, cx: &mut AppContext) {
    cx.set_global(TerminalInlineAssistant::new(telemetry));
}

const PROMPT_HISTORY_MAX_LEN: usize = 20;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
struct TerminalInlineAssistId(usize);

impl TerminalInlineAssistId {
    fn post_inc(&mut self) -> TerminalInlineAssistId {
        let id = *self;
        self.0 += 1;
        id
    }
}

pub struct TerminalInlineAssistant {
    next_assist_id: TerminalInlineAssistId,
    assists: HashMap<TerminalInlineAssistId, TerminalInlineAssist>,
    assists_by_editor: HashMap<WeakView<TerminalView>, TerminalInlineAssistId>,
    prompt_history: VecDeque<String>,
    telemetry: Option<Arc<Telemetry>>,
}

impl Global for TerminalInlineAssistant {}

impl TerminalInlineAssistant {
    pub fn new(telemetry: Arc<Telemetry>) -> Self {
        Self {
            next_assist_id: TerminalInlineAssistId::default(),
            assists: HashMap::default(),
            assists_by_editor: HashMap::default(),
            prompt_history: VecDeque::default(),
            telemetry: Some(telemetry),
        }
    }

    pub fn assist(
        &mut self,
        terminal: &View<TerminalView>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut WindowContext,
    ) {
        dbg!("Inline terminal assistant");
    }
}

struct TerminalInlineAssist {}
