use anyhow::{Context as _, anyhow};
use gpui::{App, InspectorElementId, IntoElement, Window};
use std::{
    cell::{OnceCell, RefCell},
    path::Path,
    rc::Rc,
    sync::OnceLock,
};
use ui::{Label, prelude::*};
use util::{ResultExt as _, command::new_smol_command};

use crate::interactivity_inspector::InteractivityInspector;

// todo!
//
// * Distinct "picker" mode for the inspector
//
// * Show bounds / size info. On hover, highlight element

// TODO: Move logic of the gpui `Inspector` entity into this crate:
//
// * `Inspector` trait with methods like `on_click` and `on_hover` that are given
// InspectorElementId.
//
// * Add `with_rendered_inspector_states` to `Window`. gets set on `App`.
//
// Motivations:
//
// * No need for InteractivityInspector to keep track of InspectorElementId to detect if it changes
// to rebuild Editor.
//
// * Can get invoked when inspected element changes instead of on render. This would allow things
// like modes where clicks or even hovers open the source code.
//
// * GPUI just implement what's needed to implement an inspector, since so much of the inspector
// logic is already outside GPUI (due to access to editor / theme / ui components / etc).

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

    cx.set_inspector_renderer(render_inspector);

    let interactivity_inspector = OnceCell::new();
    cx.register_inspector_element(move |id, state, window, cx| {
        let interactivity_inspector = interactivity_inspector
            .get_or_init(|| cx.new(|cx| InteractivityInspector::new(window, cx)));
        interactivity_inspector.update(cx, |interactivity_inspector, cx| {
            interactivity_inspector.update_inspected_element(&id, state, window, cx);
            interactivity_inspector
                .render(window, cx)
                .into_any_element()
        })
    })
}

fn render_inspector(
    inspector_element_id: Option<&InspectorElementId>,
    rendered_inspector_states: Vec<AnyElement>,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement + use<> {
    v_flex()
        .id("gpui-inspector")
        .size_full()
        .bg(cx.theme().colors().panel_background)
        .text_color(cx.theme().colors().text)
        .font(theme::setup_ui_font(window, cx))
        .p_2()
        .gap_2()
        .border_l_1()
        .border_color(cx.theme().colors().border)
        .overflow_y_scroll()
        .child(
            h_flex()
                .w_full()
                .pb_2()
                .border_b_1()
                .border_color(cx.theme().colors().border_variant)
                .items_center()
                .justify_center()
                .child(Label::new("GPUI Inspector").size(LabelSize::Large)),
        )
        .when_some(inspector_element_id, |this, inspector_element_id| {
            let source_location = inspector_element_id.source_location;
            this.child(
                Button::new("view-source", "View Source").on_click(|_, _window, cx| {
                    cx.background_spawn(open_zed_source_location(source_location))
                        .detach_and_log_err(cx);
                }),
            )
            .child(
                v_flex()
                    .child(
                        Label::new(inspector_element_id.global_id.to_string())
                            .size(LabelSize::Small),
                    )
                    // todo! Make this link-styled and clickable
                    .child(Label::new(format!("{}", source_location)).size(LabelSize::Small))
                    .child(
                        Label::new(format!("Instance {}", inspector_element_id.instance_id))
                            .size(LabelSize::Small),
                    ),
            )
        })
        .children(
            rendered_inspector_states
                .into_iter()
                .map(|e| {
                    div()
                        .child(e)
                        .border_b_1()
                        .border_color(cx.theme().colors().border_variant)
                })
                .collect::<Vec<_>>(),
        )
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
