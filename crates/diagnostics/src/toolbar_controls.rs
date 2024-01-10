use crate::ProjectDiagnosticsEditor;
use gpui::{div, EventEmitter, ParentElement, Render, ViewContext, WeakView};
use ui::prelude::*;
use ui::{IconButton, IconName, Tooltip};
use workspace::{item::ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView};

pub struct ToolbarControls {
    editor: Option<WeakView<ProjectDiagnosticsEditor>>,
}

impl Render for ToolbarControls {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let include_warnings = self
            .editor
            .as_ref()
            .and_then(|editor| editor.upgrade())
            .map(|editor| editor.read(cx).include_warnings)
            .unwrap_or(false);

        let tooltip = if include_warnings {
            "Exclude Warnings"
        } else {
            "Include Warnings"
        };

        div().child(
            IconButton::new("toggle-warnings", IconName::ExclamationTriangle)
                .tooltip(move |cx| Tooltip::text(tooltip, cx))
                .on_click(cx.listener(|this, _, cx| {
                    if let Some(editor) = this.editor.as_ref().and_then(|editor| editor.upgrade()) {
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
}
