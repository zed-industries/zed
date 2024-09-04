use crate::ProjectDiagnosticsEditor;
use gpui::{EventEmitter, ParentElement, Render, View, ViewContext, WeakView};
use ui::prelude::*;
use ui::{IconButton, IconName, Tooltip};
use workspace::{item::ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView};

pub struct ToolbarControls {
    editor: Option<WeakView<ProjectDiagnosticsEditor>>,
}

impl Render for ToolbarControls {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut include_warnings = false;
        let mut has_stale_excerpts = false;
        let mut is_updating = false;

        if let Some(editor) = self.editor() {
            let editor = editor.read(cx);
            include_warnings = editor.include_warnings;
            has_stale_excerpts = !editor.paths_to_update.is_empty();
            is_updating = editor.update_paths_tx.len() > 0
                || editor
                    .project
                    .read(cx)
                    .language_servers_running_disk_based_diagnostics(cx)
                    .next()
                    .is_some();
        }

        let tooltip = if include_warnings {
            "Exclude Warnings"
        } else {
            "Include Warnings"
        };

        h_flex()
            .when(has_stale_excerpts, |div| {
                div.child(
                    IconButton::new("update-excerpts", IconName::Update)
                        .icon_color(Color::Info)
                        .disabled(is_updating)
                        .tooltip(move |cx| Tooltip::text("Update excerpts", cx))
                        .on_click(cx.listener(|this, _, cx| {
                            if let Some(editor) = this.editor() {
                                editor.update(cx, |editor, _| {
                                    editor.enqueue_update_stale_excerpts(None);
                                });
                            }
                        })),
                )
            })
            .child(
                IconButton::new("toggle-warnings", IconName::ExclamationTriangle)
                    .tooltip(move |cx| Tooltip::text(tooltip, cx))
                    .on_click(cx.listener(|this, _, cx| {
                        if let Some(editor) = this.editor() {
                            editor.update(cx, |editor, cx| {
                                editor.toggle_warnings(&Default::default(), cx);
                            });
                        }
                    })),
            )
    }
}

impl EventEmitter<ToolbarItemEvent> for ToolbarControls {}

impl ToolbarItemView for ToolbarControls {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        if let Some(pane_item) = active_pane_item.as_ref() {
            if let Some(editor) = pane_item.downcast::<ProjectDiagnosticsEditor>() {
                self.editor = Some(editor.downgrade());
                ToolbarItemLocation::PrimaryRight
            } else {
                ToolbarItemLocation::Hidden
            }
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

impl ToolbarControls {
    pub fn new() -> Self {
        ToolbarControls { editor: None }
    }

    fn editor(&self) -> Option<View<ProjectDiagnosticsEditor>> {
        self.editor.as_ref()?.upgrade()
    }
}
