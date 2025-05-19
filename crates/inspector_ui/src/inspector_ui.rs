use std::path::Path;

use anyhow::{Context as _, anyhow};
use gpui::{App, DivInspectorState, InspectorElementId, IntoElement, Window};
use ui::prelude::*;
use ui::{Button, Label, LabelSize, h_flex, v_flex};
use util::ResultExt as _;
use util::command::new_smol_command;

pub fn init(cx: &mut App) {
    // TODO: Instead toggle a global debug mode? Not all windows support the command pallete.
    cx.on_action(|_: &zed_actions::dev::ToggleInspector, cx| {
        let Some(active_window) = cx
            .active_window()
            .context("no active window to toggle inspector")
            .log_err()
        else {
            return;
        };
        // This is deferred to avoid double lease due to window already being updated.
        cx.defer(move |cx| {
            active_window
                .update(cx, |_, window, cx| window.toggle_inspector(cx))
                .log_err();
        });
    });
    cx.register_inspector_element(div_inspector)
}

pub fn div_inspector(
    id: InspectorElementId,
    _state: &DivInspectorState,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement + use<> {
    v_flex()
        .bg(cx.theme().colors().elevated_surface_background)
        .p_4()
        .mt_4()
        .mr_4()
        .rounded_lg()
        .shadow_lg()
        .child(h_flex().child(Label::new(id.to_string()).size(LabelSize::XSmall)))
        .child(Button::new("open", "Open").on_click({
            let id = id.clone();
            move |_event, _window, cx| {
                cx.background_spawn(open_zed_source_location(id.source))
                    .detach_and_log_err(cx);
            }
        }))
}

// TODO: Move to some other crate (along with build.rs) and also use this in error notifications.
async fn open_zed_source_location(
    location: &'static std::panic::Location<'static>,
) -> anyhow::Result<()> {
    let mut path = Path::new(env!("ZED_REPO_DIR")).to_path_buf();
    path.push(Path::new(location.file()));
    let path_arg = format!(
        "{}:{}:{}",
        path.display(),
        location.line(),
        location.column()
    );

    let output = new_smol_command("zed")
        .arg(&path_arg)
        .output()
        .await
        .with_context(|| format!("running zed to open {path_arg} failed"))?;

    if !output.status.success() {
        Err(anyhow!(
            "running zed to open {path_arg} failed with stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    } else {
        Ok(())
    }
}
