use crate::{BufferDiagnosticsEditor, ProjectDiagnosticsEditor, ToggleDiagnosticsRefresh};
use gpui::{Context, EventEmitter, ParentElement, Render, Window};
use language::DiagnosticEntry;
use text::{Anchor, BufferId};
use ui::prelude::*;
use ui::{IconButton, IconButtonShape, IconName, Tooltip};
use workspace::{ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, item::ItemHandle};

pub struct ToolbarControls {
    editor: Option<Box<dyn DiagnosticsToolbarEditor>>,
}

pub(crate) trait DiagnosticsToolbarEditor: Send + Sync {
    /// Informs the toolbar whether warnings are included in the diagnostics.
    fn include_warnings(&self, cx: &App) -> bool;
    /// Toggles whether warning diagnostics should be displayed by the
    /// diagnostics editor.
    fn toggle_warnings(&self, window: &mut Window, cx: &mut App);
    /// Indicates whether the diagnostics editor is currently updating the
    /// diagnostics.
    fn is_updating(&self, cx: &App) -> bool;
    /// Requests that the diagnostics editor stop updating the diagnostics.
    fn stop_updating(&self, cx: &mut App);
    /// Requests that the diagnostics editor updates the displayed diagnostics
    /// with the latest information.
    fn refresh_diagnostics(&self, window: &mut Window, cx: &mut App);
    /// Returns a list of diagnostics for the provided buffer id.
    fn get_diagnostics_for_buffer(
        &self,
        buffer_id: BufferId,
        cx: &App,
    ) -> Vec<DiagnosticEntry<Anchor>>;
}

impl Render for ToolbarControls {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut include_warnings = false;
        let mut is_updating = false;

        match &self.editor {
            Some(editor) => {
                include_warnings = editor.include_warnings(cx);
                is_updating = editor.is_updating(cx);
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
                                if let Some(editor) = toolbar_controls.editor() {
                                    editor.stop_updating(cx);
                                    cx.notify();
                                }
                            })),
                    )
                } else {
                    div.child(
                        IconButton::new("refresh-diagnostics", IconName::ArrowCircle)
                            .icon_color(Color::Info)
                            .shape(IconButtonShape::Square)
                            .tooltip(Tooltip::for_action_title(
                                "Refresh diagnostics",
                                &ToggleDiagnosticsRefresh,
                            ))
                            .on_click(cx.listener({
                                move |toolbar_controls, _, window, cx| {
                                    if let Some(editor) = toolbar_controls.editor() {
                                        editor.refresh_diagnostics(window, cx)
                                    }
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
                    .on_click(cx.listener(|this, _, window, cx| {
                        if let Some(editor) = &this.editor {
                            editor.toggle_warnings(window, cx)
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
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        if let Some(pane_item) = active_pane_item.as_ref() {
            if let Some(editor) = pane_item.downcast::<ProjectDiagnosticsEditor>() {
                self.editor = Some(Box::new(editor.downgrade()));
                ToolbarItemLocation::PrimaryRight
            } else if let Some(editor) = pane_item.downcast::<BufferDiagnosticsEditor>() {
                self.editor = Some(Box::new(editor.downgrade()));
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

    fn editor(&self) -> Option<&dyn DiagnosticsToolbarEditor> {
        self.editor.as_deref()
    }
}
