use gpui::{actions, AppContext, Context, ViewContext};
use log;
use manager::ContextServerManager;
use workspace::Workspace;

pub mod client;
pub mod manager;
pub mod protocol;
mod registry;
pub mod types;

pub use registry::*;

actions!(context_servers, [Restart]);

pub fn init(cx: &mut AppContext) {
    log::info!("initializing context server client");
    manager::init(cx);
    ContextServerRegistry::register(cx);

    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace.register_action(restart_servers);
        },
    )
    .detach();
}

fn restart_servers(_workspace: &mut Workspace, _action: &Restart, cx: &mut ViewContext<Workspace>) {
    let model = ContextServerManager::global(&cx);
    cx.update_model(&model, |manager, cx| {
        for server in manager.servers() {
            manager.restart_server(&server.id, cx).detach();
        }
    });
}
