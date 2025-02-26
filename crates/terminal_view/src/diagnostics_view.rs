use std::collections::BTreeSet;
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::time::Duration;

use collections::IndexMap;
use diagnostics::{IncludeWarnings, ProjectDiagnosticsSettings, ToggleWarnings};
use editor::Editor;
use gpui::{
    list, uniform_list, AppContext, Entity, EventEmitter, Flatten, FocusHandle, Focusable,
    FontWeight, ListAlignment, ListState, Rgba, StatefulInteractiveElement, Subscription, Task,
    UniformListScrollHandle, WeakEntity,
};
use language::{
    Anchor, Buffer, DiagnosticEntry, DiagnosticGroup, DiagnosticSeverity, LanguageServerId,
    OffsetRangeExt,
};
use project::Project;
use project::{DiagnosticSummary, ProjectPath};
use settings::Settings;
use ui::{
    div, h_flex, px, v_flex, AnyElement, App, ButtonCommon, Clickable, Color, Context, Element,
    FluentBuilder, Icon, IconButton, IconButtonShape, IconName, IconSize, InteractiveElement,
    IntoElement, Label, LabelCommon, LabelSize, List, ListHeader, ListItem, ParentElement, Render,
    Styled, Tooltip, Window,
};
use util::ResultExt;
use workspace::item::TabContentParams;
use workspace::SerializableItem;
use workspace::{item::ItemEvent, searchable::SearchEvent, Event, Item, Workspace, WorkspaceId};

///A terminal view, maintains the PTY's file handles and communicates with the terminal
pub struct DiagnosticsView {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    summary: DiagnosticSummary,
    paths_to_update: BTreeSet<(ProjectPath, Option<LanguageServerId>)>,
    _subscription: Subscription,
    update_diagnostics_task: Option<Task<anyhow::Result<()>>>,
    include_warnings: bool,
    diagnostic_groups:
        IndexMap<ProjectPath, Vec<(Entity<Buffer>, LanguageServerId, DiagnosticEntry<Anchor>)>>,
    diagnostic_list: ListState,
}

impl DiagnosticsView {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        workspace_id: Option<WorkspaceId>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(move |cx: &mut Context<'_, Self>| {
            let focus_handle = cx.focus_handle();

            cx.observe_global_in::<IncludeWarnings>(window, |this, window, cx| {
                this.include_warnings = cx.global::<IncludeWarnings>().0;
                this.paths_to_update = this
                    .project
                    .read(cx)
                    .diagnostic_summaries(false, cx)
                    .map(|(path, lsp_id, _)| (path, Some(lsp_id)))
                    .collect::<BTreeSet<_>>();
                this.update_diagnostics(window, cx);
                cx.notify();
            })
            .detach();

            let project_event_subscription = cx.subscribe_in(
                &project,
                window,
                |this, project, event, window, cx| match event {
                    project::Event::DiskBasedDiagnosticsStarted { .. } => {
                        cx.notify();
                    }
                    project::Event::DiskBasedDiagnosticsFinished { language_server_id } => {
                        // log::debug!("disk based diagnostics finished for server {language_server_id}");
                        // this.update_diagnostics(window, cx);
                    }
                    project::Event::DiagnosticsUpdated {
                        language_server_id,
                        path,
                    } => {
                        this.paths_to_update = project
                            .read(cx)
                            .diagnostic_summaries(false, cx)
                            .map(|(path, lsp_id, _)| (path, Some(lsp_id)))
                            .collect::<BTreeSet<_>>();
                        this.summary = project.read(cx).diagnostic_summary(false, cx);

                        this.update_diagnostics(window, cx);
                    }
                    _ => {}
                },
            );

            let summary = project.read(cx).diagnostic_summary(false, cx);

            let paths_to_update = project
                .read(cx)
                .diagnostic_summaries(false, cx)
                .map(|(path, lsp_id, _)| (path, Some(lsp_id)))
                .collect::<BTreeSet<_>>();
            let include_warnings = match cx.try_global::<IncludeWarnings>() {
                Some(include_warnings) => include_warnings.0,
                None => ProjectDiagnosticsSettings::get_global(cx).include_warnings,
            };

            let entity = cx.entity().downgrade();
            let diagnostic_list = ListState::new(
                paths_to_update.len(),
                ListAlignment::Top,
                px(100.),
                move |ix, window, cx| {
                    entity
                        .upgrade()
                        .and_then(|entity| {
                            entity
                                .update(cx, |this, cx| this.render_diagnostic_group(ix, window, cx))
                        })
                        .unwrap_or_else(|| div().into_any())
                },
            );

            let mut diagnostics_view = Self {
                workspace,
                summary,
                project,
                focus_handle,
                paths_to_update,
                _subscription: project_event_subscription,
                update_diagnostics_task: None,
                diagnostic_groups: IndexMap::default(),
                include_warnings,
                diagnostic_list,
            };
            diagnostics_view.update_diagnostics(window, cx);

            diagnostics_view
        })
    }

    fn update_diagnostics(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.update_diagnostics_task.is_some() {
            return;
        }
        let project_handle = self.project.clone();
        self.update_diagnostics_task = Some(cx.spawn_in(window, |this, mut cx| async move {
            cx.background_executor()
                .timer(DIAGNOSTICS_UPDATE_DEBOUNCE)
                .await;
            let mut first = true;
            loop {
                let Some((path, language_server_id)) = this
                    .update(&mut cx, |this, _| {
                        if first {
                            this.diagnostic_groups.clear();
                            first = false;
                        }

                        let Some((path, language_server_id)) = this.paths_to_update.pop_first()
                        else {
                            this.update_diagnostics_task.take();
                            return None;
                        };
                        Some((path, language_server_id))
                    })
                    .unwrap()
                else {
                    break;
                };

                if let Some(buffer) = project_handle
                    .update(&mut cx, |project, cx| project.open_buffer(path.clone(), cx))
                    .unwrap()
                    .await
                    .ok()
                {
                    let snapshot = this.update(&mut cx, |_, cx| buffer.read(cx).snapshot())?;
                    let diagnostic_groups = snapshot.diagnostic_groups(language_server_id);
                    this.update(&mut cx, |diag_view, cx| {
                        let diag_entry =
                            diagnostic_groups
                                .into_iter()
                                .filter_map(|(lsp_id, mut diag_group)| {
                                    let diag = diag_group.entries.remove(diag_group.primary_ix);

                                    if diag_view.include_warnings {
                                        Some((buffer.clone(), lsp_id, diag))
                                    } else {
                                        (diag.diagnostic.severity > DiagnosticSeverity::WARNING)
                                            .then_some((buffer.clone(), lsp_id, diag))
                                    }
                                });
                        match diag_view.diagnostic_groups.get_mut(&path) {
                            Some(e) => {
                                e.extend(diag_entry);
                            }
                            None => {
                                diag_view
                                    .diagnostic_groups
                                    .insert(path.clone(), diag_entry.collect());
                            }
                        }

                        diag_view
                            .diagnostic_list
                            .splice(Range::default(), diag_view.diagnostic_groups.len());
                        cx.notify();
                    })?;
                } else {
                    break;
                }
            }

            Ok(())
        }));
    }

    fn toggle_warnings(&mut self, _: &ToggleWarnings, window: &mut Window, cx: &mut Context<Self>) {
        self.include_warnings = !self.include_warnings;
        cx.set_global(IncludeWarnings(self.include_warnings));
        self.paths_to_update = self
            .project
            .read(cx)
            .diagnostic_summaries(false, cx)
            .map(|(path, lsp_id, _)| (path, Some(lsp_id)))
            .collect::<BTreeSet<_>>();
        self.update_diagnostics(window, cx);
        cx.notify();
    }

    fn render_diagnostic_group(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let (project_path, diags) = self.diagnostic_groups.get_index(ix)?;
        let task_workspace = self.workspace.clone();
        let task_project_path = project_path.clone();

        if diags.is_empty() {
            return None;
        }
        let diags_per_file: Vec<ListItem> = diags
            .iter()
            .enumerate()
            .map(|(idx, (buffer, _, diag))| {
                let icon = match diag.diagnostic.severity {
                    DiagnosticSeverity::ERROR => Icon::new(IconName::X).color(Color::Error),
                    DiagnosticSeverity::HINT => Icon::new(IconName::Book).color(Color::Hint),
                    DiagnosticSeverity::INFORMATION => Icon::new(IconName::Info).color(Color::Info),
                    DiagnosticSeverity::WARNING => {
                        Icon::new(IconName::Warning).color(Color::Warning)
                    }
                    _ => unreachable!("should not happen"),
                };
                let point = diag.range.to_point(&buffer.read(cx).snapshot());

                let task_workspace = task_workspace.clone();
                let task_project_path = task_project_path.clone();
                ListItem::new(idx)
                    .child(icon)
                    .child(
                        div().size_full().child(Label::new(
                            diag.diagnostic
                                .message
                                .split('\n')
                                .next()
                                .unwrap()
                                .to_string(),
                        )),
                    )
                    .child(
                        div()
                            .right_0()
                            .child(Label::new(format!(
                                "{}:{}",
                                point.start.row, point.start.column
                            )))
                            .font_weight(FontWeight::THIN),
                    )
                    .tooltip(Tooltip::text(
                        diag.diagnostic
                            .data
                            .as_ref()
                            .and_then(|data| data.get("rendered"))
                            .and_then(|rendered_text| rendered_text.as_str())
                            .map(|t| t.to_string())
                            .unwrap_or_else(|| diag.diagnostic.message.clone()),
                    ))
                    .on_click(cx.listener(move |_, _, window, cx| {
                        let task_workspace = task_workspace.clone();
                        let task_project_path = task_project_path.clone();

                        cx.spawn_in(window, |_diagnostic_view, mut cx| async move {
                            let open_path = task_workspace
                                .update_in(&mut cx, |workspace, window, cx| {
                                    workspace.open_path(
                                        task_project_path.clone(),
                                        None,
                                        true,
                                        window,
                                        cx,
                                    )
                                })
                                .log_err()?
                                .await
                                .log_err()?;

                            if let Some(active_editor) = open_path.downcast::<Editor>() {
                                active_editor
                                    .downgrade()
                                    .update_in(&mut cx, |editor, window, cx| {
                                        editor.go_to_singleton_buffer_point(point.start, window, cx)
                                    })
                                    .log_err()?;
                            }

                            Some(())
                        })
                        .detach();
                    }))
            })
            .collect();

        List::new()
            .header(ListHeader::new(
                project_path.path.to_string_lossy().to_string(),
            ))
            .children(diags_per_file)
            .into_any_element()
            .into()
    }
}

const DIAGNOSTICS_UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

impl Render for DiagnosticsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tooltip = if self.include_warnings {
            "Exclude Warnings"
        } else {
            "Include Warnings"
        };

        let warning_color = if self.include_warnings {
            Color::Warning
        } else {
            Color::Muted
        };
        v_flex()
            .id("diagnostics-view")
            .size_full()
            .relative()
            .on_action(cx.listener(Self::toggle_warnings))
            .track_focus(&self.focus_handle(cx))
            .child(
                h_flex().justify_end().child(
                    IconButton::new("toggle-warnings", IconName::Warning)
                        .tooltip(Tooltip::text(tooltip))
                        .icon_color(warning_color)
                        .shape(IconButtonShape::Square)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.toggle_warnings(&ToggleWarnings {}, window, cx);
                        })),
                ),
            )
            .child(list(self.diagnostic_list.clone()).size_full().flex_grow())
            .flex_grow()
    }
}

impl Item for DiagnosticsView {
    type Event = ItemEvent;

    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        const DIAG_TITLE: &str = "Diagnostics ";
        let title = match (self.summary.error_count, self.summary.warning_count) {
            (0, 0) => h_flex().map(|this| {
                this.child(Label::new(DIAG_TITLE)).child(
                    Icon::new(IconName::Check)
                        .size(IconSize::Small)
                        .color(Color::Default),
                )
            }),
            (0, warning_count) => h_flex()
                .gap_1()
                .child(Label::new(DIAG_TITLE))
                .child(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .color(Color::Warning),
                )
                .child(Label::new(warning_count.to_string()).size(LabelSize::Small)),
            (error_count, 0) => h_flex()
                .gap_1()
                .child(Label::new(DIAG_TITLE))
                .child(
                    Icon::new(IconName::XCircle)
                        .size(IconSize::Small)
                        .color(Color::Error),
                )
                .child(Label::new(error_count.to_string()).size(LabelSize::Small)),
            (error_count, warning_count) => h_flex()
                .gap_1()
                .child(Label::new(DIAG_TITLE))
                .child(
                    Icon::new(IconName::XCircle)
                        .size(IconSize::Small)
                        .color(Color::Error),
                )
                .child(Label::new(error_count.to_string()).size(LabelSize::Small))
                .child(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .color(Color::Warning),
                )
                .child(Label::new(warning_count.to_string()).size(LabelSize::Small)),
        };

        h_flex()
            .gap_1()
            .group("diagnostics-tab-icon")
            .child(title)
            .into_any()
    }
}

impl Focusable for DiagnosticsView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<Event> for DiagnosticsView {}
impl EventEmitter<ItemEvent> for DiagnosticsView {}
impl EventEmitter<SearchEvent> for DiagnosticsView {}

impl SerializableItem for DiagnosticsView {
    fn serialized_item_kind() -> &'static str {
        "Diagnostics"
    }

    fn should_serialize(&self, event: &Self::Event) -> bool {
        matches!(event, ItemEvent::UpdateTab)
    }

    fn deserialize(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: workspace::WorkspaceId,
        _item_id: workspace::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<anyhow::Result<Entity<Self>>> {
        let window_handle = window.window_handle();
        let workspace_diag = workspace.clone();
        let project = project.clone();
        window.spawn(cx, |mut cx| async move {
            gpui::Flatten::flatten(cx.update_window(window_handle, |_view, window, cx| {
                workspace.update(cx, |_this, cx| {
                    DiagnosticsView::new(workspace_diag, Some(workspace_id), project, window, cx)
                })
            }))
        })
    }

    fn cleanup(
        _workspace_id: WorkspaceId,
        _alive_items: Vec<workspace::ItemId>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<gpui::Result<()>> {
        Task::ready(Ok(()))
    }

    fn serialize(
        &mut self,
        _workspace: &mut Workspace,
        _item_id: workspace::ItemId,
        _closing: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Task<gpui::Result<()>>> {
        Some(Task::ready(Ok(())))
    }
}
