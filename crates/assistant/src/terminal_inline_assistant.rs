use std::sync::Arc;

use client::telemetry::Telemetry;
use gpui::{AppContext, Global, View, WeakView};
use terminal_view::TerminalView;
use ui::WindowContext;
use workspace::Workspace;

pub fn init(telemetry: Arc<Telemetry>, cx: &mut AppContext) {
    cx.set_global(TerminalInlineAssistant::new(telemetry));
}

pub struct TerminalInlineAssistant {
    telemetry: Option<Arc<Telemetry>>,
}

impl Global for TerminalInlineAssistant {}

impl TerminalInlineAssistant {
    pub fn new(telemetry: Arc<Telemetry>) -> Self {
        Self {
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
