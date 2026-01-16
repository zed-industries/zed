mod sign_in;

use copilot::{Reinstall, SignIn, SignOut};
use gpui::App;
use workspace::Workspace;

pub use sign_in::{
    ConfigurationMode, ConfigurationView, CopilotCodeVerification, initiate_sign_in,
    reinstall_and_sign_in,
};

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|_, _: &SignIn, window, cx| {
            sign_in::initiate_sign_in(window, cx);
        });
        workspace.register_action(|_, _: &Reinstall, window, cx| {
            sign_in::reinstall_and_sign_in(window, cx);
        });
        workspace.register_action(|_, _: &SignOut, window, cx| {
            sign_in::initiate_sign_out(window, cx);
        });
    })
    .detach();
}
