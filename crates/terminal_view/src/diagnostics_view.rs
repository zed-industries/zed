use std::collections::BTreeSet;
use std::ops::Range;
use std::time::Duration;

use collections::IndexMap;
use diagnostics::{IncludeWarnings, ToggleWarnings};
use editor::{actions, Editor};
use gpui::{
    list, AppContext, ClickEvent, Entity, EventEmitter, FocusHandle, Focusable, FontWeight,
    ListAlignment, ListState, Subscription, Task, WeakEntity,
};
use itertools::Itertools;
use language::{
    Anchor, Buffer, DiagnosticEntry, DiagnosticSeverity, LanguageServerId, OffsetRangeExt,
};
use menu::{Confirm, SecondaryConfirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use project::project_settings::ProjectSettings;
use project::Project;
use project::{DiagnosticSummary, ProjectPath};
use settings::Settings;
use ui::{
    div, h_flex, px, v_flex, AnyElement, App, ButtonCommon, Clickable, Color, Context, Element,
    FluentBuilder, Icon, IconButton, IconButtonShape, IconName, IconSize, InteractiveElement,
    IntoElement, Label, LabelCommon, LabelSize, List, ListHeader, ListItem, ParentElement, Render,
    Styled, Toggleable, Tooltip, Window,
};
use util::ResultExt;
use workspace::item::TabContentParams;
use workspace::SerializableItem;
use workspace::{item::ItemEvent, searchable::SearchEvent, Event, Item, Workspace, WorkspaceId};

pub struct DiagnosticsView {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    summary: DiagnosticSummary,
    paths_to_update: BTreeSet<(ProjectPath, Option<LanguageServerId>)>,
    _subscriptions: Vec<Subscription>,
    update_diagnostics_task: Option<Task<anyhow::Result<()>>>,
    include_warnings: bool,
    diagnostic_groups:
        IndexMap<ProjectPath, Vec<(Entity<Buffer>, LanguageServerId, DiagnosticEntry<Anchor>)>>,
    diagnostic_list: ListState,
    selected_entry: Option<(usize, usize)>,
}

impl DiagnosticsView {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(move |cx: &mut Context<'_, Self>| {
            let focus_handle = cx.focus_handle();

            cx.observe_global_in::<IncludeWarnings>(window, |this, window, cx| {
                this.include_warnings = cx.global::<IncludeWarnings>().0;
                this.update_diagnostics(window, cx);
                cx.notify();
            })
            .detach();

            let project_event_subscription = cx.subscribe_in(
                &project,
                window,
                |this, _project, event, window, cx| match event {
                    project::Event::DiskBasedDiagnosticsStarted { .. } => {
                        cx.notify();
                    }
                    project::Event::DiskBasedDiagnosticsFinished { .. } => {
                        this.update_diagnostics(window, cx);
                    }
                    project::Event::DiagnosticsUpdated { .. } => {
                        this.update_diagnostics(window, cx);
                    }
                    _ => {}
                },
            );

            let project_handle = project.read(cx);
            let summary = project_handle.diagnostic_summary(false, cx);
            let paths_to_update = project_handle
                .diagnostic_summaries(false, cx)
                .map(|(path, lsp_id, _)| (path, Some(lsp_id)))
                .collect::<BTreeSet<_>>();
            let include_warnings = match cx.try_global::<IncludeWarnings>() {
                Some(include_warnings) => include_warnings.0,
                None => ProjectSettings::get_global(cx).diagnostics.include_warnings,
            };

            let entity = cx.entity().downgrade();
            let diagnostic_list = ListState::new(
                paths_to_update.len(),
                ListAlignment::Top,
                px(100.),
                move |ix, _window, cx| {
                    entity
                        .upgrade()
                        .and_then(|entity| {
                            entity.update(cx, |this, cx| this.render_diagnostic_group(ix, cx))
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
                _subscriptions: vec![project_event_subscription],
                update_diagnostics_task: None,
                diagnostic_groups: IndexMap::default(),
                include_warnings,
                diagnostic_list,
                selected_entry: None,
            };
            diagnostics_view.update_diagnostics(window, cx);

            diagnostics_view
        })
    }

    fn update_diagnostics(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let project = self.project.read(cx);
        self.paths_to_update = project
            .diagnostic_summaries(false, cx)
            .map(|(path, lsp_id, _)| (path, Some(lsp_id)))
            .collect::<BTreeSet<_>>();
        self.summary = project.diagnostic_summary(false, cx);
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
                let Some((path, language_server_id)) = this.update(&mut cx, |this, _| {
                    if first {
                        this.diagnostic_groups.clear();
                        first = false;
                    }

                    let Some((path, language_server_id)) = this.paths_to_update.pop_first() else {
                        this.update_diagnostics_task.take();
                        return None;
                    };
                    Some((path, language_server_id))
                })?
                else {
                    this.update(&mut cx, |_this, cx| {
                        cx.notify();
                    })
                    .log_err();
                    break;
                };

                if let Some(buffer) = project_handle
                    .update(&mut cx, |project, cx| project.open_buffer(path.clone(), cx))?
                    .await
                    .ok()
                {
                    let snapshot = this.update(&mut cx, |_, cx| buffer.read(cx).snapshot())?;
                    let diagnostic_groups = snapshot.diagnostic_groups(language_server_id);
                    this.update(&mut cx, |diag_view, cx| {
                        let diag_entry: Vec<(
                            Entity<Buffer>,
                            LanguageServerId,
                            DiagnosticEntry<Anchor>,
                        )> = diagnostic_groups
                            .into_iter()
                            .filter_map(|(lsp_id, mut diag_group)| {
                                let diag = diag_group.entries.remove(diag_group.primary_ix);
                                if diag_view.include_warnings {
                                    Some((buffer.clone(), lsp_id, diag))
                                } else {
                                    (diag.diagnostic.severity < DiagnosticSeverity::WARNING)
                                        .then_some((buffer.clone(), lsp_id, diag))
                                }
                            })
                            .collect();
                        if diag_entry.is_empty() {
                            return;
                        }
                        match diag_view.diagnostic_groups.get_mut(&path) {
                            Some(e) => {
                                e.extend(diag_entry);
                            }
                            None => {
                                diag_view.diagnostic_groups.insert(path.clone(), diag_entry);
                            }
                        }

                        diag_view
                            .diagnostic_list
                            .splice(Range::default(), diag_view.diagnostic_groups.len());
                        diag_view.selected_entry = None;
                        cx.notify();
                    })?;
                } else {
                    break;
                }
            }

            Ok(())
        }));
    }

    /// When it's triggered from action listener
    fn toggle_warnings(&mut self, _: &ToggleWarnings, window: &mut Window, cx: &mut Context<Self>) {
        self.include_warnings = !self.include_warnings;
        self.update_diagnostics(window, cx);
        cx.notify();
    }

    /// When it's triggered from a button click for example
    fn toggle_warnings_click(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.include_warnings = !self.include_warnings;
        cx.set_global(IncludeWarnings(self.include_warnings));
        self.update_diagnostics(window, cx);
        cx.notify();
    }

    fn render_diagnostic_group(&mut self, ix: usize, cx: &mut Context<Self>) -> Option<AnyElement> {
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
                let point_sec = point.clone();
                let is_active = match self.selected_entry {
                    Some(selected_entry) => selected_entry.0 == ix && selected_entry.1 == idx,
                    None => false,
                };

                let task_workspace = task_workspace.clone();
                let task_project_path = task_project_path.clone();
                let task_sec_workspace = task_workspace.clone();
                let task_sec_project_path = task_project_path.clone();
                ListItem::new(idx)
                    .child(icon)
                    .toggle_state(is_active)
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
                    .on_secondary_mouse_down(cx.listener(move |this, _, window, cx| {
                        let task_workspace = task_sec_workspace.clone();
                        let task_project_path = task_sec_project_path.clone();
                        this.select_entry((idx, ix), false, window, cx);

                        Self::open_diag(
                            point_sec.clone(),
                            true,
                            task_project_path,
                            task_workspace,
                            window,
                            cx,
                        );
                    }))
                    .on_click(cx.listener(move |this, click: &ClickEvent, window, cx| {
                        this.select_entry((idx, ix), false, window, cx);
                        let task_workspace = task_workspace.clone();
                        let task_project_path = task_project_path.clone();
                        let platform_key_pressed = click.modifiers().platform;
                        Self::open_diag(
                            point.clone(),
                            platform_key_pressed,
                            task_project_path,
                            task_workspace,
                            window,
                            cx,
                        );
                    }))
            })
            .collect();

        List::new()
            .id(ix)
            .header(ListHeader::new(
                project_path.path.to_string_lossy().to_string(),
            ))
            .children(diags_per_file)
            .into_any_element()
            .into()
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        match self.selected_entry() {
            Some(selected_entry) => {
                if self
                    .diagnostic_groups
                    .get_index(selected_entry.0)
                    .and_then(|(_, diags)| diags.get(selected_entry.1 + 1))
                    .is_some()
                {
                    self.select_entry((selected_entry.0, selected_entry.1 + 1), true, window, cx);
                } else if self
                    .diagnostic_groups
                    .get_index(selected_entry.0 + 1)
                    .and_then(|(_, diags)| diags.get(0))
                    .is_some()
                {
                    self.select_entry((selected_entry.0 + 1, 0), true, window, cx);
                } else {
                    return;
                }
            }
            None => self.select_first(&SelectFirst {}, window, cx),
        }
    }

    fn select_prev(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        match self.selected_entry() {
            Some(selected_entry) => {
                if self
                    .diagnostic_groups
                    .get_index(selected_entry.0)
                    .and_then(|(_, diags)| diags.get(selected_entry.1.checked_sub(1)?))
                    .is_some()
                {
                    self.select_entry(
                        (selected_entry.0, selected_entry.1.saturating_sub(1)),
                        true,
                        window,
                        cx,
                    );
                } else if selected_entry.0 > 0
                    && self
                        .diagnostic_groups
                        .get_index(selected_entry.0.saturating_sub(1))
                        .map(|(_, diags)| diags.get(0))
                        .is_some()
                {
                    self.select_entry(
                        (
                            selected_entry.0.saturating_sub(1),
                            self.diagnostic_groups
                                .get_index(selected_entry.0.saturating_sub(1))
                                .expect("already checked in the condition")
                                .1
                                .len()
                                .saturating_sub(1),
                        ),
                        true,
                        window,
                        cx,
                    );
                } else {
                    return;
                }
            }
            None => self.select_last(&SelectLast {}, window, cx),
        }
    }

    fn select_first(&mut self, _: &SelectFirst, window: &mut Window, cx: &mut Context<Self>) {
        if !self.diagnostic_groups.is_empty()
            && self
                .diagnostic_groups
                .iter()
                .any(|(_, diags)| !diags.is_empty())
        {
            self.select_entry((0, 0), true, window, cx);
        }
    }

    fn select_last(&mut self, _: &SelectLast, window: &mut Window, cx: &mut Context<Self>) {
        let Some((position, (_, diags))) = self
            .diagnostic_groups
            .iter()
            .find_position(|(_, diags)| !diags.is_empty())
        else {
            return;
        };

        self.select_entry(
            (
                self.diagnostic_groups
                    .len()
                    .saturating_sub(1)
                    .saturating_sub(position),
                diags.len().saturating_sub(1),
            ),
            true,
            window,
            cx,
        );
    }

    fn on_confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.open_selected_entry(false, window, cx);
    }

    fn on_secondary_confirm(
        &mut self,
        _: &SecondaryConfirm,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_selected_entry(true, window, cx);
    }

    fn open_selected_entry(
        &mut self,
        secondary_action: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((project_path, (buffer, _lsp_id, diag))) =
            self.selected_entry.and_then(|(diag_group_idx, diag_idx)| {
                self.diagnostic_groups
                    .get_index(diag_group_idx)
                    .and_then(|(project_path, diags)| Some((project_path, diags.get(diag_idx)?)))
            })
        {
            let point = diag.range.to_point(&buffer.read(cx).snapshot());
            let task_workspace = self.workspace.clone();
            let project_path = project_path.clone();
            Self::open_diag(
                point,
                secondary_action,
                project_path,
                task_workspace,
                window,
                cx,
            );
        }
    }

    /// Open diagnostic in editor
    fn open_diag(
        point: Range<rope::Point>,
        platform_key_pressed: bool,
        project_path: ProjectPath,
        task_workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<DiagnosticsView>,
    ) {
        cx.spawn_in(window, |_diagnostic_view, mut cx| async move {
            let open_path = task_workspace
                .update_in(&mut cx, |workspace, window, cx| {
                    workspace.open_path(project_path, None, true, window, cx)
                })
                .log_err()?
                .await
                .log_err()?;

            if let Some(active_editor) = open_path.downcast::<Editor>() {
                active_editor
                    .downgrade()
                    .update_in(&mut cx, |editor, window, cx| {
                        editor.go_to_singleton_buffer_point(point.start, window, cx);

                        if platform_key_pressed {
                            window.dispatch_action(
                                Box::new(actions::ToggleCodeActions {
                                    deployed_from_indicator: Some(editor::display_map::DisplayRow(
                                        point.start.row,
                                    )),
                                }),
                                cx,
                            );
                        }
                    })
                    .log_err()?;
            }

            Some(())
        })
        .detach();
    }

    fn selected_entry(&self) -> Option<(usize, usize)> {
        self.selected_entry
    }

    fn select_entry(
        &mut self,
        entry: (usize, usize),
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if focus {
            self.focus_handle.focus(window);
        }
        self.selected_entry = entry.into();

        self.autoscroll(cx);
    }

    fn autoscroll(&mut self, cx: &mut Context<Self>) {
        // FIXME: if the number of diagnostics per file are higher than the number of items we can display on the screen it doesn't work properly
        if let Some(selected_entry) = self.selected_entry() {
            self.diagnostic_list.scroll_to_reveal_item(selected_entry.0);
            cx.notify();
        }
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
            .on_action(cx.listener(Self::on_confirm))
            .on_action(cx.listener(Self::on_secondary_confirm))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::toggle_warnings))
            .track_focus(&self.focus_handle(cx))
            .child(
                h_flex().justify_end().child(
                    IconButton::new("toggle-warnings", IconName::Warning)
                        .tooltip(Tooltip::text(tooltip))
                        .icon_color(warning_color)
                        .shape(IconButtonShape::Square)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.toggle_warnings_click(window, cx);
                        })),
                ),
            )
            .child(list(self.diagnostic_list.clone()).size_full().flex_grow())
            .flex_grow()
    }
}

impl Item for DiagnosticsView {
    type Event = ItemEvent;

    fn tab_content(&self, _params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        const DIAG_TITLE: &str = "Diagnostics ";
        let title = if self.update_diagnostics_task.is_some()
            || self
                .project
                .read(cx)
                .language_servers_running_disk_based_diagnostics(cx)
                .next()
                .is_some()
        {
            h_flex().map(|this| {
                this.child(Label::new(DIAG_TITLE)).child(
                    Icon::new(IconName::Update)
                        .size(IconSize::Small)
                        .color(Color::Default),
                )
            })
        } else {
            match (self.summary.error_count, self.summary.warning_count) {
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
            }
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
        _workspace_id: workspace::WorkspaceId,
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
                    DiagnosticsView::new(workspace_diag, project, window, cx)
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
