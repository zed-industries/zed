#[cfg(debug_assertions)]
mod div_inspector;
#[cfg(debug_assertions)]
mod inspector;

#[cfg(debug_assertions)]
pub use inspector::init;

#[cfg(not(debug_assertions))]
pub fn init(_app_state: std::sync::Arc<workspace::AppState>, cx: &mut gpui::App) {
    use std::any::TypeId;
    use workspace::notifications::{NotificationSource, NotifyResultExt as _};

    cx.on_action(|_: &zed_actions::dev::ToggleInspector, cx| {
        Err::<(), anyhow::Error>(anyhow::anyhow!(
            "dev::ToggleInspector is only available in debug builds"
        ))
        .notify_app_err(NotificationSource::System, cx);
    });

    command_palette_hooks::CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_action_types(&[TypeId::of::<zed_actions::dev::ToggleInspector>()]);
    });
}
