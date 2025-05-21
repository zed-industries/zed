#[cfg(debug_assertions)]
mod inspector;
#[cfg(debug_assertions)]
mod interactivity_inspector;

#[cfg(debug_assertions)]
pub use inspector::init;

#[cfg(not(debug_assertions))]
pub fn init(cx: &mut gpui::App) {
    use workspace::notifications::NotifyResultExt;

    cx.on_action(|_: &zed_actions::dev::ToggleInspector, cx| {
        Err::<(), anyhow::Error>(anyhow::anyhow!(
            "dev::ToggleInspector is only available in debug builds"
        ))
        .notify_app_err(cx);
    });

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_action_types(&[zed_actions::dev::ToggleInspector::type_id()]);
    });
}
