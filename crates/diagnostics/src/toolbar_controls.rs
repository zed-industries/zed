use crate::ProjectDiagnosticsEditor;
use gpui::{Context, Entity, EventEmitter, ParentElement, Render, WeakEntity, Window};
use project::project_settings::ProjectSettings;
use settings::Settings as _;
use ui::prelude::*;
use ui::{IconButton, IconButtonShape, IconName, Tooltip};
use workspace::{ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, item::ItemHandle};

pub struct ToolbarControls {
    editor: Option<WeakEntity<ProjectDiagnosticsEditor>>,
}

impl Render for ToolbarControls {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut include_warnings = false;
        let mut has_stale_excerpts = false;
        let mut is_updating = false;

        if let Some(editor) = self.diagnostics() {
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

        let zed_provides_cargo_diagnostics = ProjectSettings::get_global(cx)
            .diagnostics
            .fetch_cargo_diagnostics();

        let update_excerpts_tooltip = if has_stale_excerpts {
            Some("Update excerpts")
        } else if zed_provides_cargo_diagnostics {
            Some("Fetch cargo diagnostics")
        } else {
            None
        };

        let tooltip = if include_warnings {
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
            .when_some(update_excerpts_tooltip, |div, update_excerpts_tooltip| {
                div.child(
                    IconButton::new("update-excerpts", IconName::Update)
                        .icon_color(Color::Info)
                        .shape(IconButtonShape::Square)
                        .disabled(is_updating)
                        .tooltip(Tooltip::text(update_excerpts_tooltip))
                        .on_click(cx.listener(|this, _, window, cx| {
                            if let Some(diagnostics) = this.diagnostics() {
                                diagnostics.update(cx, |diagnostics, cx| {
                                    diagnostics.update_all_excerpts(window, cx);
                                });
                            }
                        })),
                )
            })
            .child(
                IconButton::new("toggle-warnings", IconName::Warning)
                    .icon_color(warning_color)
                    .shape(IconButtonShape::Square)
                    .tooltip(Tooltip::text(tooltip))
                    .on_click(cx.listener(|this, _, window, cx| {
                        if let Some(editor) = this.diagnostics() {
                            editor.update(cx, |editor, cx| {
                                editor.toggle_warnings(&Default::default(), window, cx);
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
        _window: &mut Window,
        _: &mut Context<Self>,
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

impl Default for ToolbarControls {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolbarControls {
    pub fn new() -> Self {
        ToolbarControls { editor: None }
    }

    fn diagnostics(&self) -> Option<Entity<ProjectDiagnosticsEditor>> {
        self.editor.as_ref()?.upgrade()
    }
}
