use anyhow::{Context as _, anyhow};
use gpui::{App, Hsla, InspectorElementId, IntoElement, Window};
use std::{cell::OnceCell, path::Path, sync::Arc};
use ui::{Label, Tooltip, prelude::*};
use util::{ResultExt as _, command::new_smol_command};
use workspace::AppState;

use crate::div_inspector::DivInspector;

// todo!
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
// * No need for DivInspector to keep track of InspectorElementId to detect if it changes
// to rebuild Editor.
//
// * Can get invoked when inspected element changes instead of on render. This would allow things
// like modes where clicks or even hovers open the source code.
//
// * GPUI just implement what's needed to implement an inspector, since so much of the inspector
// logic is already outside GPUI (due to access to editor / theme / ui components / etc).

// TODO: Related to above TODO, it could make sense to not even have special handling of rendering
// the inspector to the side - it could just be a workspace item.

pub fn init(app_state: Arc<AppState>, cx: &mut App) {
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

    // Project used for editor buffers + LSP support
    let project = project::Project::local(
        app_state.client.clone(),
        app_state.node_runtime.clone(),
        app_state.user_store.clone(),
        app_state.languages.clone(),
        app_state.fs.clone(),
        None,
        cx,
    );

    let div_inspector = OnceCell::new();
    cx.register_inspector_element(move |id, state, window, cx| {
        let div_inspector = div_inspector
            .get_or_init(|| cx.new(|cx| DivInspector::new(project.clone(), window, cx)));
        div_inspector.update(cx, |div_inspector, cx| {
            div_inspector.update_inspected_element(&id, state, window, cx);
            div_inspector.render(window, cx).into_any_element()
        })
    });

    cx.set_inspector_renderer(render_inspector);
}

fn render_inspector(
    inspector_id: Option<&InspectorElementId>,
    rendered_inspector_states: Vec<AnyElement>,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement + use<> {
    let ui_font = theme::setup_ui_font(window, cx);
    let colors = cx.theme().colors();
    v_flex()
        .id("gpui-inspector")
        .size_full()
        // TODO: Choose an appropriate color from the theme.
        .bg(cx
            .theme()
            .colors()
            .panel_background
            .blend(Hsla::black().alpha(0.05)))
        .text_color(colors.text)
        .font(ui_font)
        .p_2()
        .gap_2()
        .border_l_1()
        .border_color(colors.border)
        .overflow_y_scroll()
        .child(
            h_flex()
                .w_full()
                .pb_2()
                .border_b_1()
                .border_color(colors.border_variant)
                .child(
                    IconButton::new("pick-mode", IconName::MagnifyingGlass)
                        .tooltip(Tooltip::text("Start inspector pick mode"))
                        // TODO: Why isn't the icon colored when inspecting?
                        .selected_icon_color(Color::Selected)
                        .toggle_state(window.is_inspector_picking(cx))
                        .on_click(|_, window, cx| {
                            window.start_inspector_picking(cx);
                        }),
                )
                .child(
                    h_flex()
                        .w_full()
                        .justify_end()
                        .child(Label::new("GPUI Inspector").size(LabelSize::Large)),
                ),
        )
        .when_some(inspector_id, |this, inspector_id| {
            let source_location = inspector_id.source_location;
            this.child(
                v_flex()
                    .pb_2()
                    .border_b_1()
                    .border_color(colors.border_variant)
                    .child(
                        div()
                            .text_ui_sm(cx)
                            .child(inspector_id.global_id.to_string()),
                    )
                    .child(
                        div()
                            .id("source-location")
                            .text_ui_sm(cx)
                            .bg(colors.editor_foreground.opacity(0.025))
                            .underline()
                            .child(format!("{}", source_location))
                            .tooltip(Tooltip::text("Run Zed cli to open this source location"))
                            .on_click(move |_, _window, cx| {
                                cx.background_spawn(open_zed_source_location(source_location))
                                    .detach_and_log_err(cx);
                            }),
                    )
                    .child(
                        Label::new(format!("Instance {}", inspector_id.instance_id))
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
                        .border_color(colors.border_variant)
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
