use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use anyhow::{Context as _, anyhow};
use editor::{Editor, EditorEvent};
use gpui::{App, DivInspectorState, Empty, Entity, InspectorElementId, IntoElement, Window};
use language::language_settings::SoftWrap;
use ui::{Button, Label, LabelSize, h_flex, v_flex};
use ui::{CheckboxWithLabel, prelude::*};
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

    let inspector_settings = cx.new(|_cx| InspectorSettings {
        open_code_on_inspect: true,
    });

    let last_div_inspector = Rc::new(RefCell::new(None));
    cx.register_inspector_element({
        move |id, state, window, cx| {
            div_inspector(
                inspector_settings.clone(),
                &last_div_inspector,
                id,
                state,
                window,
                cx,
            )
        }
    })
}

struct DivInspector {
    id: Rc<InspectorElementId>,
    style_editor: Entity<Editor>,
}

impl DivInspector {
    fn render(&self, inspector_settings: Entity<InspectorSettings>, cx: &App) -> Div {
        v_flex()
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .p_2()
            .gap_2()
            .child(inspector_settings)
            .child(Label::new(self.id.to_string()).size(LabelSize::Small))
            .child(
                v_flex().gap_1().child(Label::new("Style")).child(
                    div()
                        .elevation_2(cx)
                        .p_1()
                        .h_128()
                        .child(self.style_editor.clone()),
                ),
            )
    }
}

fn div_inspector(
    inspector_settings: Entity<InspectorSettings>,
    last_div_inspector: &Rc<RefCell<Option<DivInspector>>>,
    id: InspectorElementId,
    state: &DivInspectorState,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement + use<> {
    let mut last_div_inspector = last_div_inspector.borrow_mut();
    if let Some(last_div_inspector) = &*last_div_inspector {
        if last_div_inspector.id.as_ref() == &id {
            return last_div_inspector.render(inspector_settings, cx);
        }
    }

    // todo! Better error handling
    let Some(json_text) = serde_json::to_string_pretty(state).log_err() else {
        return div();
    };
    let style_editor = cx.new(|cx| {
        let mut editor = Editor::multi_line(window, cx);
        editor.set_text(json_text, window, cx);
        editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
        editor.set_show_line_numbers(false, cx);
        editor.set_show_code_actions(false, cx);
        editor.set_show_breakpoints(false, cx);
        editor.set_show_git_diff_gutter(false, cx);
        editor.set_show_runnables(false, cx);
        editor.set_show_edit_predictions(Some(false), window, cx);
        editor
    });

    let id = Rc::new(id);

    window
        .subscribe(&style_editor, cx, {
            let id = id.clone();
            move |editor, event: &EditorEvent, window, cx| {
                match event {
                    EditorEvent::BufferEdited => {
                        let json_text = editor.read(cx).text(cx);
                        // todo! error handling
                        let Some(new_state) = serde_json_lenient::from_str(&json_text).log_err()
                        else {
                            return;
                        };
                        window.update_inspector_state::<DivInspectorState, _>(
                            &id,
                            |state, _window| {
                                *state = new_state;
                            },
                        )
                    }
                    _ => {}
                }
            }
        })
        .detach();

    if inspector_settings.read(cx).open_code_on_inspect {
        cx.background_spawn(open_zed_source_location(id.source))
            .detach_and_log_err(cx);
    }

    let div_inspector = DivInspector { id, style_editor };
    let rendered = div_inspector.render(inspector_settings, cx);
    *last_div_inspector = Some(div_inspector);
    return rendered;
}

struct InspectorSettings {
    open_code_on_inspect: bool,
}

impl Render for InspectorSettings {
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
