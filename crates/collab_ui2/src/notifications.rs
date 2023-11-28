use gpui::AppContext;
use std::sync::Arc;
use workspace::AppState;

pub mod incoming_call_notification;
// pub mod project_shared_notification;

pub fn init(app_state: &Arc<AppState>, cx: &mut AppContext) {
    incoming_call_notification::init(app_state, cx);
    //project_shared_notification::init(app_state, cx);
}
