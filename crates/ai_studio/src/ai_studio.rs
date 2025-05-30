mod studio;
mod model_manager;
mod provider_registry;
mod chat_interface;
pub mod workflow;
pub mod ai_config;

pub use studio::*;
pub use model_manager::*;
pub use provider_registry::*;
pub use chat_interface::*;
pub use workflow::*;
pub use ai_config::*;

use gpui::{actions, App, AppContext, Window};

actions!(ai_studio, [OpenAiStudio]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut workspace::Workspace, _window: Option<&mut Window>, _cx| {
        workspace.register_action(open_ai_studio);
    })
    .detach();
}

fn open_ai_studio(
    workspace: &mut workspace::Workspace,
    _: &OpenAiStudio,
    window: &mut Window,
    cx: &mut gpui::Context<workspace::Workspace>,
) {
    let ai_studio = cx.new(|cx| AiStudio::new(window, cx));
    workspace.add_item_to_active_pane(Box::new(ai_studio), None, true, window, cx);
} 