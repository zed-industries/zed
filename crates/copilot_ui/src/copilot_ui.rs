pub mod copilot_button;
mod copilot_completion_provider;
mod sign_in;

use client::telemetry::Telemetry;
use copilot::Copilot;
pub use copilot_button::*;
pub use copilot_completion_provider::*;
pub use sign_in::*;
use std::sync::Arc;

pub fn init(telemetry: Arc<Telemetry>, cx: &mut AppContext) {
    if let Some(copilot) = Copilot::global(cx) {
        cx.observe_new_views(move |editor: &mut Editor, cx: &mut ViewContext<Editor>| {
            if editor.mode() == EditorMode::Full {
                let provider = cx.new_model(|_| {
                    let mut ccp = CopilotCompletionProvider::new(copilot.clone());
                    ccp.set_telemetry(telemetry.clone());
                    ccp
                });
                editor.set_inline_completion_provider(provider, cx)
            }
        })
        .detach();
    }
}
