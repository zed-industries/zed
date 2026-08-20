#[cfg(any(debug_assertions, feature = "inspector"))]
mod div_inspector;
#[cfg(any(debug_assertions, feature = "inspector"))]
mod inspector;

#[cfg(any(debug_assertions, feature = "inspector"))]
pub use inspector::init;

#[cfg(not(any(debug_assertions, feature = "inspector")))]
pub fn init(_app_state: std::sync::Arc<workspace::AppState>, cx: &mut gpui::App) {
    use std::any::TypeId;
    use workspace::notifications::NotifyResultExt as _;

    cx.on_action(|_: &zed_actions::dev::ToggleInspector, cx| {
        Err::<(), anyhow::Error>(anyhow::anyhow!(
            "dev::ToggleInspector is only available in debug builds and Nightly"
        ))
        .notify_app_err(cx);
    });

    command_palette_hooks::CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_action_types(&[TypeId::of::<zed_actions::dev::ToggleInspector>()]);
    });
}
