use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use anyhow::{Context as _, anyhow};
use editor::{Editor, EditorEvent, EditorMode, MultiBuffer};
use futures::{FutureExt as _, future::Shared};
use gpui::{
    App, AsyncWindowContext, DivInspectorState, Entity, InspectorElementId, IntoElement, Task,
    Window,
};
use language::language_settings::SoftWrap;
use project::ProjectPath;
use ui::{CheckboxWithLabel, prelude::*};
use ui::{Label, LabelSize, v_flex};
use util::ResultExt as _;
use util::command::new_smol_command;
use workspace::Workspace;

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

    let load_state = Rc::new(RefCell::new(None));
    cx.register_inspector_element(move |id, state, window, cx| {
        render_or_load_div_inspector(
            inspector_settings.clone(),
            &load_state,
            id,
            state,
            window,
            cx,
        )
    })
}

struct DivInspectorLoadState {
    id: Rc<InspectorElementId>,
    task: Shared<Task<DivInspector>>,
}

#[derive(Clone)]
struct DivInspector {
    style_editor: Entity<Editor>,
}

fn render_or_load_div_inspector(
    inspector_settings: Entity<InspectorSettings>,
    load_state: &Rc<RefCell<Option<DivInspectorLoadState>>>,
    id: InspectorElementId,
    state: &DivInspectorState,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement + use<> {
    let mut load_state = load_state.borrow_mut();
    let mut start_load = true;
    if let Some(load_state) = &*load_state {
        if load_state.id.as_ref() == &id {
            if let Some(last_div_inspector) = load_state.task.clone().now_or_never() {
                return last_div_inspector
                    .render(&load_state.id.as_ref(), inspector_settings, cx)
                    .into_any_element();
            } else {
                start_load = false;
            }
        }
    }

    if start_load {
        // todo! Better error handling
        let json_text = serde_json::to_string_pretty(state).unwrap();
        let id = Rc::new(id);
        *load_state = Some(DivInspectorLoadState {
            id: id.clone(),
            task: window
                .spawn(cx, async move |cx| {
                    DivInspector::load(inspector_settings, &id, json_text, cx).await
                })
                .shared(),
        });
    }

    return Label::new("Loading...").into_any_element();
}

impl DivInspector {
    // todo! no unwraps / maybe no log_err
    async fn load(
        inspector_settings: Entity<InspectorSettings>,
        id: &InspectorElementId,
        json_text: String,
        cx: &mut AsyncWindowContext,
    ) -> DivInspector {
        let project = cx
            .update(|window, cx| {
                let workspace = window.root::<Workspace>().flatten();
                workspace.map(|workspace| workspace.read(cx).project().clone())
            })
            .unwrap()
            .unwrap();

        let worktree_id = project
            .read_with(cx, |project, cx| {
                project
                    .worktrees(cx)
                    .filter(|worktree| {
                        let worktree = worktree.read(cx);
                        !worktree.is_single_file() && worktree.is_local()
                    })
                    .next()
                    .unwrap()
                    .read(cx)
                    .id()
            })
            .unwrap();
        let project_path = ProjectPath {
            worktree_id,
            path: Path::new("zed-style-inspector.json").into(),
        };

        let style_buffer = project
            .update(cx, |project, cx| project.open_path(project_path, cx))
            .unwrap()
            .await
            .unwrap()
            .1;

        let style_editor = cx
            .new_window_entity(|window, cx| {
                /*
                let buffer = cx.new(|cx| {
                    let mut buffer = Buffer::local(json_text, cx);
                    // todO!
                    // buffer.file_updated(new_file, cx);
                    buffer.set_language(json_language.clone().now_or_never().flatten(), cx);
                    buffer
                });
                */
                style_buffer.update(cx, |style_buffer, cx| style_buffer.set_text(json_text, cx));
                let multi_buffer = cx.new(|cx| MultiBuffer::singleton(style_buffer, cx));
                let mut editor =
                    Editor::new(EditorMode::full(), multi_buffer, Some(project), window, cx);
                editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
                editor.set_show_line_numbers(false, cx);
                editor.set_show_code_actions(false, cx);
                editor.set_show_breakpoints(false, cx);
                editor.set_show_git_diff_gutter(false, cx);
                editor.set_show_runnables(false, cx);
                editor.set_show_edit_predictions(Some(false), window, cx);
                editor
            })
            .unwrap();

        cx.update(|window, cx| {
            window
                .subscribe(&style_editor, cx, {
                    let id = id.clone();
                    move |editor, event: &EditorEvent, window, cx| {
                        match event {
                            EditorEvent::BufferEdited => {
                                let json_text = editor.read(cx).text(cx);
                                // todo! error handling
                                let Some(new_state) =
                                    serde_json_lenient::from_str(&json_text).log_err()
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
                .detach()
        })
        .log_err();

        inspector_settings
            .read_with(cx, |inspector_settings, cx| {
                if inspector_settings.open_code_on_inspect {
                    cx.background_spawn(open_zed_source_location(id.source))
                        .detach_and_log_err(cx);
                }
            })
            .log_err();

        return DivInspector { style_editor };
    }

    fn render(
        &self,
        id: &InspectorElementId,
        inspector_settings: Entity<InspectorSettings>,
        cx: &App,
    ) -> Div {
        v_flex()
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .p_2()
            .gap_2()
            .child(inspector_settings)
            .child(Label::new(id.to_string()).size(LabelSize::Small))
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
