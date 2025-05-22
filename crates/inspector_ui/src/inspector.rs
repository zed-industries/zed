use anyhow::{Context as _, anyhow};
use gpui::{App, Entity, IntoElement, Window};
use std::{cell::RefCell, path::Path, rc::Rc};
use ui::{CheckboxWithLabel, Label, prelude::*};
use util::{ResultExt as _, command::new_smol_command};

// todo!
//
// * Refine "Open code" - confusing that it doesn't do anything when checked. Add another way to
// open code?
//
// * Distinct "picker" mode for the inspector
//
// * Way to open actual user code instead of ui component

// * Ability to tell gpui if it is in picker mode or not
//
// * GPUI then calls the inspector on hovers / picks

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

    let inspector_options = cx.new(|_cx| InspectorOptions {
        open_code_on_inspect: false,
    });

    cx.set_inspector_renderer(move |states, window, cx| {
        render_inspector(inspector_options.clone(), states, window, cx)
    });

    /*
    inspector_options
        .read_with(cx, |inspector_options, cx| {
            if inspector_options.open_code_on_inspect {
                cx.background_spawn(open_zed_source_location(id.source))
                    .detach_and_log_err(cx);
            }
        })
        .log_err();
    */

    let load_state = Rc::new(RefCell::new(None));
    cx.register_inspector_element(move |id, state, window, cx| {
        crate::interactivity_inspector::render_or_load(&load_state, id, state, window, cx)
    })
}

pub(crate) struct InspectorOptions {
    pub open_code_on_inspect: bool,
}

impl Render for InspectorOptions {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        CheckboxWithLabel::new(
            "open-code-on-inspect",
            Label::new("Open code"),
            self.open_code_on_inspect.into(),
            cx.listener(|this, selection: &ToggleState, _, _| {
                this.open_code_on_inspect = selection.selected();
            }),
        )
    }
}

fn render_inspector(
    inspector_options: Entity<InspectorOptions>,
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
        .child(inspector_options)
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
