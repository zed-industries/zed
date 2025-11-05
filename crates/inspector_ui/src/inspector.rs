use anyhow::{Context as _, anyhow};
use gpui::{App, DivInspectorState, Inspector, InspectorElementId, IntoElement, Window};
use std::{cell::OnceCell, path::Path, sync::Arc};
use title_bar::platform_title_bar::PlatformTitleBar;
use ui::{Label, Tooltip, prelude::*};
use util::{ResultExt as _, command::new_smol_command};
use workspace::AppState;

use crate::div_inspector::DivInspector;

pub fn init(app_state: Arc<AppState>, cx: &mut App) {
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

    // Project used for editor buffers with LSP support
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
    cx.register_inspector_element(move |id, state: &DivInspectorState, window, cx| {
        let div_inspector = div_inspector
            .get_or_init(|| cx.new(|cx| DivInspector::new(project.clone(), window, cx)));
        div_inspector.update(cx, |div_inspector, cx| {
            div_inspector.update_inspected_element(&id, state.clone(), window, cx);
            div_inspector.render(window, cx).into_any_element()
        })
    });

    cx.set_inspector_renderer(Box::new(render_inspector));
}

fn render_inspector(
    inspector: &mut Inspector,
    window: &mut Window,
    cx: &mut Context<Inspector>,
) -> AnyElement {
    let ui_font = theme::setup_ui_font(window, cx);
    let colors = cx.theme().colors();
    let inspector_id = inspector.active_element_id();
    let toolbar_height = PlatformTitleBar::height(window);

    v_flex()
        .size_full()
        .bg(colors.panel_background)
        .text_color(colors.text)
        .font(ui_font)
        .border_l_1()
        .border_color(colors.border)
        .child(
            h_flex()
                .justify_between()
                .pr_2()
                .pl_1()
                .mt_px()
                .h(toolbar_height)
                .border_b_1()
                .border_color(colors.border_variant)
                .child(
                    IconButton::new("pick-mode", IconName::MagnifyingGlass)
                        .tooltip(Tooltip::text("Start inspector pick mode"))
                        .selected_icon_color(Color::Selected)
                        .toggle_state(inspector.is_picking())
                        .on_click(cx.listener(|inspector, _, window, _cx| {
                            inspector.start_picking();
                            window.refresh();
                        })),
                )
                .child(h_flex().justify_end().child(Label::new("GPUI Inspector"))),
        )
        .child(
            v_flex()
                .id("gpui-inspector-content")
                .overflow_y_scroll()
                .px_2()
                .py_0p5()
                .gap_2()
                .when_some(inspector_id, |this, inspector_id| {
                    this.child(render_inspector_id(inspector_id, cx))
                })
                .children(inspector.render_inspector_states(window, cx)),
        )
        .into_any_element()
}

fn render_inspector_id(inspector_id: &InspectorElementId, cx: &App) -> Div {
    let source_location = inspector_id.path.source_location;
    // For unknown reasons, for some elements the path is absolute.
    let source_location_string = source_location.to_string();
    let source_location_string = source_location_string
        .strip_prefix(env!("ZED_REPO_DIR"))
        .and_then(|s| s.strip_prefix("/"))
        .map(|s| s.to_string())
        .unwrap_or(source_location_string);

    v_flex()
        .child(
            h_flex()
                .justify_between()
                .child(Label::new("Element ID").size(LabelSize::Large))
                .child(
                    div()
                        .id("instance-id")
                        .text_ui(cx)
                        .tooltip(Tooltip::text(
                            "Disambiguates elements from the same source location",
                        ))
                        .child(format!("Instance {}", inspector_id.instance_id)),
                ),
        )
        .child(
            div()
                .id("source-location")
                .text_ui(cx)
                .bg(cx.theme().colors().editor_foreground.opacity(0.025))
                .underline()
                .font_buffer(cx)
                .text_xs()
                .child(source_location_string)
                .tooltip(Tooltip::text("Click to open by running Zed CLI"))
                .on_click(move |_, _window, cx| {
                    cx.background_spawn(open_zed_source_location(source_location))
                        .detach_and_log_err(cx);
                }),
        )
        .child(
            div()
                .id("global-id")
                .text_ui(cx)
                .min_h_20()
                .tooltip(Tooltip::text(
                    "GlobalElementId of the nearest ancestor with an ID",
                ))
                .child(inspector_id.path.global_id.to_string()),
        )
}

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
