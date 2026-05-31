use crate::Editor;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Pixels, Render,
    WeakEntity, Window, px,
};
use project::Project;
use std::path::PathBuf;
use ui::{Button, KeyBinding, Label, prelude::*};
use workspace::{ModalView, Workspace};

pub struct CreateFileModal {
    filename_editor: Entity<Editor>,
    current_directory: PathBuf,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    error_message: Option<String>,
}

impl Focusable for CreateFileModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.filename_editor.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for CreateFileModal {}

impl ModalView for CreateFileModal {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        workspace::DismissDecision::Dismiss(true)
    }
}

impl CreateFileModal {
    pub fn new(
        current_directory: PathBuf,
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let filename_editor = cx.new(|cx| Editor::multi_line(window, cx));

        filename_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text("Enter name (end with / for directory)...", window, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_gutter(false, cx);
            editor.scroll_manager.show_scrollbars(window, cx);
            editor.set_vim_insert_on_focus(true);
        });

        let focus_handle = filename_editor.focus_handle(cx);

        cx.on_focus_out(&focus_handle, window, |_, _, _, cx| {
            cx.emit(DismissEvent);
        })
        .detach();

        cx.subscribe_in(
            &filename_editor,
            window,
            |this, _, _event: &crate::EditorEvent, window, cx| {
                this.validate_filename(window, cx);
            },
        )
        .detach();

        Self {
            filename_editor,
            current_directory,
            project,
            workspace,
            error_message: None,
        }
    }

    fn validate_filename(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let filename = self.filename_editor.read(cx).text(cx);
        let filename = filename.trim();

        if filename.is_empty() {
            self.error_message = Some("Please enter a non-blank file name".to_string());
            cx.notify();
            return;
        }

        let new_file_path = self.current_directory.join(filename);

        if new_file_path.exists() {
            self.error_message = Some(format!("'{}' already exists", filename));
        } else {
            self.error_message = None;
        }
        cx.notify();
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let filename = self.filename_editor.read(cx).text(cx);
        let filename = filename.trim();

        if filename.is_empty() {
            self.error_message = Some("Please enter a filename".to_string());
            cx.notify();
            return;
        }

        let is_directory = filename.ends_with('/');
        let filename = filename.trim_end_matches('/');
        let new_file_path = self.current_directory.join(filename);

        if new_file_path.exists() {
            self.error_message = Some(format!("'{}' already exists", filename));
            cx.notify();
            return;
        }

        let project = self.project.clone();
        let workspace = self.workspace.clone();

        cx.spawn_in(window, async move |_, cx| {
            let project_path = project.read_with(cx, |project, cx| {
                project.find_project_path(&new_file_path, cx)
            });

            if let Some(project_path) = project_path {
                let worktree = project.read_with(cx, |project, cx| {
                    project.worktree_for_id(project_path.worktree_id, cx)
                });

                if let Some(worktree) = worktree {
                    let abs_path = worktree
                        .read_with(cx, |worktree, _| worktree.absolutize(&project_path.path));

                    if is_directory {
                        let _ = std::fs::create_dir_all(&abs_path);
                    } else {
                        let write_result = std::fs::write(&abs_path, "");
                        if write_result.is_ok() {
                            let open_task = workspace.update_in(cx, |workspace, window, cx| {
                                workspace.open_abs_path(
                                    abs_path.clone(),
                                    workspace::OpenOptions::default(),
                                    window,
                                    cx,
                                )
                            });
                            if let Ok(task) = open_task {
                                let _result = task.await;
                            }
                        }
                    }
                }
            }
        })
        .detach();

        cx.emit(DismissEvent);
    }

    fn modal_width() -> Pixels {
        px(400.0)
    }
}

impl Render for CreateFileModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        let directory_display = {
            let path_str = self.current_directory.to_string_lossy().to_string();
            let home_dir = std::env::var("HOME").unwrap_or_else(|_| "".to_string());
            if !home_dir.is_empty() && path_str.starts_with(&home_dir) {
                path_str.replacen(&home_dir, "~", 1)
            } else {
                path_str
            }
        };

        v_flex()
            .key_context("CreateFileModal")
            .elevation_3(cx)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .w(Self::modal_width())
            .p_4()
            .gap_3()
            .bg(cx.theme().colors().elevated_surface_background)
            .rounded_lg()
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(
                v_flex()
                    .child(Label::new("Create New File or Directory").size(LabelSize::Large))
                    .child(
                        Label::new(directory_display)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        div()
                            .w_full()
                            .px_2()
                            .py_1p5()
                            .bg(cx.theme().colors().editor_background)
                            .rounded_md()
                            .border_1()
                            .border_color(if self.error_message.is_some() {
                                Color::Error.color(cx)
                            } else {
                                cx.theme().colors().border_variant
                            })
                            .child(h_flex().h_6().w_full().child(self.filename_editor.clone())),
                    )
                    .when_some(self.error_message.clone(), |this, message| {
                        this.child(
                            Label::new(message)
                                .size(LabelSize::Small)
                                .color(Color::Error),
                        )
                    }),
            )
            .child(
                h_flex()
                    .w_full()
                    .justify_end()
                    .gap_2()
                    .child(
                        Button::new("cancel", "Cancel")
                            .key_binding(KeyBinding::for_action_in(
                                &menu::Cancel,
                                &focus_handle,
                                cx,
                            ))
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                    )
                    .child(
                        Button::new("create", "Create")
                            .key_binding(KeyBinding::for_action_in(
                                &menu::Confirm,
                                &focus_handle,
                                cx,
                            ))
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.confirm(&menu::Confirm, window, cx);
                            })),
                    ),
            )
    }
}
