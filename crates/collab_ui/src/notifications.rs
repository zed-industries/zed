pub mod incoming_call_notification;
pub mod project_shared_notification;

use gpui::App;
use std::sync::Arc;
use workspace::AppState;

pub fn init(app_state: &Arc<AppState>, cx: &mut App) {
    incoming_call_notification::init(app_state, cx);
    project_shared_notification::init(app_state, cx);
}
