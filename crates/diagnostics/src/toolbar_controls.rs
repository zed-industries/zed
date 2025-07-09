use crate::{BufferDiagnosticsEditor, ProjectDiagnosticsEditor, ToggleDiagnosticsRefresh};
use gpui::{Context, EventEmitter, ParentElement, Render, WeakEntity, Window};
use ui::prelude::*;
use ui::{IconButton, IconButtonShape, IconName, Tooltip};
use workspace::{ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, item::ItemHandle};

pub struct ToolbarControls {
    editor: Option<DiagnosticsEditorHandle>,
}

enum DiagnosticsEditorHandle {
    Project(WeakEntity<ProjectDiagnosticsEditor>),
    Buffer(WeakEntity<BufferDiagnosticsEditor>),
}

impl DiagnosticsEditorHandle {}

impl Render for ToolbarControls {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut has_stale_excerpts = false;
        let mut include_warnings = false;
        let mut is_updating = false;

        match &self.editor {
            Some(DiagnosticsEditorHandle::Project(editor)) => {
                if let Some(editor) = editor.upgrade() {
                    let diagnostics = editor.read(cx);
                    include_warnings = diagnostics.include_warnings;
                    has_stale_excerpts = !diagnostics.paths_to_update.is_empty();
                    is_updating = diagnostics.update_excerpts_task.is_some()
                        || diagnostics
                            .project
                            .read(cx)
                            .language_servers_running_disk_based_diagnostics(cx)
                            .next()
                            .is_some();
                }
            }
            Some(DiagnosticsEditorHandle::Buffer(editor)) => {
                if let Some(editor) = editor.upgrade() {
                    let diagnostics = editor.read(cx);
                    include_warnings = diagnostics.include_warnings;
                    // TODO: How to calculate this for the
                    // `BufferDiagnosticsEditor`? Should we simply keep track if
                    // there are any updates to the diagnostics for the path and
                    // mark that instead of automatically updating?
                    has_stale_excerpts = false;
                    is_updating = diagnostics.update_excerpts_task.is_some()
                        || diagnostics
                            .project
                            .read(cx)
                            .language_servers_running_disk_based_diagnostics(cx)
                            .next()
                            .is_some();
                }
            }
            None => {}
        }

        let warning_tooltip = if include_warnings {
            "Exclude Warnings"
        } else {
            "Include Warnings"
        };

        let warning_color = if include_warnings {
            Color::Warning
        } else {
            Color::Muted
        };

        h_flex()
            .gap_1()
            .map(|div| {
                if is_updating {
                    div.child(
                        IconButton::new("stop-updating", IconName::Stop)
                            .icon_color(Color::Info)
                            .shape(IconButtonShape::Square)
                            .tooltip(Tooltip::for_action_title(
                                "Stop diagnostics update",
                                &ToggleDiagnosticsRefresh,
                            ))
                            .on_click(cx.listener(move |toolbar_controls, _, _, cx| {
                                match toolbar_controls.editor() {
                                    Some(DiagnosticsEditorHandle::Buffer(
                                        buffer_diagnostics_editor,
                                    )) => {
                                        let _ = buffer_diagnostics_editor.update(
                                            cx,
                                            |buffer_diagnostics_editor, cx| {
                                                buffer_diagnostics_editor.update_excerpts_task =
                                                    None;
                                                cx.notify();
                                            },
                                        );
                                    }
                                    Some(DiagnosticsEditorHandle::Project(
                                        project_diagnostics_editor,
                                    )) => {
                                        let _ = project_diagnostics_editor.update(
                                            cx,
                                            |project_diagnostics_editor, cx| {
                                                project_diagnostics_editor.update_excerpts_task =
                                                    None;
                                                cx.notify();
                                            },
                                        );
                                    }
                                    None => {}
                                }
                            })),
                    )
                } else {
                    div.child(
                        IconButton::new("refresh-diagnostics", IconName::ArrowCircle)
                            .icon_color(Color::Info)
                            .shape(IconButtonShape::Square)
                            .disabled(!has_stale_excerpts)
                            .tooltip(Tooltip::for_action_title(
                                "Refresh diagnostics",
                                &ToggleDiagnosticsRefresh,
                            ))
                            .on_click(cx.listener({
                                move |toolbar_controls, _, window, cx| match toolbar_controls
                                    .editor()
                                {
                                    Some(DiagnosticsEditorHandle::Buffer(
                                        buffer_diagnostics_editor,
                                    )) => {
                                        let _ = buffer_diagnostics_editor.update(
                                            cx,
                                            |buffer_diagnostics_editor, cx| {
                                                buffer_diagnostics_editor
                                                    .update_all_excerpts(window, cx);
                                            },
                                        );
                                    }
                                    Some(DiagnosticsEditorHandle::Project(
                                        project_diagnostics_editor,
                                    )) => {
                                        let _ = project_diagnostics_editor.update(
                                            cx,
                                            |project_diagnostics_editor, cx| {
                                                project_diagnostics_editor
                                                    .update_all_excerpts(window, cx);
                                            },
                                        );
                                    }
                                    None => {}
                                }
                            })),
                    )
                }
            })
            .child(
                IconButton::new("toggle-warnings", IconName::Warning)
                    .icon_color(warning_color)
                    .shape(IconButtonShape::Square)
                    .tooltip(Tooltip::text(warning_tooltip))
                    .on_click(cx.listener(|this, _, window, cx| match &this.editor {
                        Some(DiagnosticsEditorHandle::Project(project_diagnostics_editor)) => {
                            let _ = project_diagnostics_editor.update(
                                cx,
                                |project_diagnostics_editor, cx| {
                                    project_diagnostics_editor.toggle_warnings(
                                        &Default::default(),
                                        window,
                                        cx,
                                    );
                                },
                            );
                        }
                        Some(DiagnosticsEditorHandle::Buffer(buffer_diagnostics_editor)) => {
                            let _ = buffer_diagnostics_editor.update(
                                cx,
                                |buffer_diagnostics_editor, cx| {
                                    buffer_diagnostics_editor.toggle_warnings(
                                        &Default::default(),
                                        window,
                                        cx,
                                    );
                                },
                            );
                        }
                        _ => {}
                    })),
            )
    }
}

impl EventEmitter<ToolbarItemEvent> for ToolbarControls {}

impl ToolbarItemView for ToolbarControls {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        if let Some(pane_item) = active_pane_item.as_ref() {
            if let Some(editor) = pane_item.downcast::<ProjectDiagnosticsEditor>() {
                self.editor = Some(DiagnosticsEditorHandle::Project(editor.downgrade()));
                ToolbarItemLocation::PrimaryRight
            } else if let Some(editor) = pane_item.downcast::<BufferDiagnosticsEditor>() {
                self.editor = Some(DiagnosticsEditorHandle::Buffer(editor.downgrade()));
                ToolbarItemLocation::PrimaryRight
            } else {
                ToolbarItemLocation::Hidden
            }
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

impl Default for ToolbarControls {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolbarControls {
    pub fn new() -> Self {
        ToolbarControls { editor: None }
    }

    fn editor(&self) -> Option<&DiagnosticsEditorHandle> {
        self.editor.as_ref()
    }
}
