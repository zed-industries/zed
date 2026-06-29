use std::time::Duration;

use gpui::{
    actions, div, App, ClipboardItem, Context, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, ParentElement, Render, SharedString, Styled, Task, Window,
};
use project::Project;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};
use ui::prelude::*;
use workspace::{Item, SplitDirection, Workspace, WorkspaceId};

use crate::get_or_create_tool;

const POLL_INTERVAL: Duration = Duration::from_secs(3);

actions!(
    dev,
    [
        /// Opens the Resource Monitor, showing CPU and memory usage
        /// of Zed and its language servers.
        OpenResourceMonitor
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &OpenResourceMonitor, window, cx| {
            let project = workspace.project().clone();
            get_or_create_tool(
                workspace,
                SplitDirection::Right,
                window,
                cx,
                move |window, cx| ResourceMonitor::new(project, window, cx),
            );
        });
    })
    .detach();
}

#[derive(Clone)]
struct ProcessSnapshot {
    name: SharedString,
    pid: u32,
    cpu_percent: f32,
    memory_bytes: u64,
    kind: ProcessKind,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ProcessKind {
    ZedMain,
    LanguageServer,
}

struct ResourceMonitor {
    project: Entity<Project>,
    focus_handle: FocusHandle,
    snapshots: Vec<ProcessSnapshot>,
    has_sampled_twice: bool,
    _poll_task: Task<()>,
}

impl ResourceMonitor {
    fn new(project: Entity<Project>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        let poll_task = cx.spawn(async move |this, mut cx| {
            let mut system = System::new();
            let refresh_kind = ProcessRefreshKind::nothing()
                .with_cpu()
                .with_memory()
                .without_tasks();
            let mut sample_count: u32 = 0;

            loop {
                // 1. Collect PIDs from the main thread
                let pids_and_names = match this.update(cx, |this, cx| {
                    this.collect_pids_and_names(cx)
                }) {
                    Ok(v) => v,
                    Err(_) => break, // Entity dropped
                };

                if !pids_and_names.is_empty() {
                    let pids: Vec<Pid> = pids_and_names
                        .iter()
                        .map(|(pid, _, _)| Pid::from_u32(*pid))
                        .collect();

                    // 2. Refresh sysinfo (System is Send in sysinfo 0.37)
                    system.refresh_processes_specifics(
                        ProcessesToUpdate::Some(&pids),
                        true,
                        refresh_kind,
                    );
                    sample_count = sample_count.saturating_add(1);

                    // 3. Build snapshots
                    let snapshots: Vec<ProcessSnapshot> = pids_and_names
                        .into_iter()
                        .filter_map(|(pid, name, kind)| {
                            let proc = system.process(Pid::from_u32(pid))?;
                            Some(ProcessSnapshot {
                                name,
                                pid,
                                cpu_percent: proc.cpu_usage(),
                                memory_bytes: proc.memory(),
                                kind,
                            })
                        })
                        .collect();

                    // 4. Push to main thread
                    let has_two = sample_count >= 2;
                    if this
                        .update(cx, |this, cx| {
                            this.apply_snapshots(snapshots, has_two, cx);
                        })
                        .is_err()
                    {
                        break; // Entity dropped
                    }
                }

                cx.background_executor().timer(POLL_INTERVAL).await;
            }
        });

        Self {
            project,
            focus_handle,
            snapshots: Vec::new(),
            has_sampled_twice: false,
            _poll_task: poll_task,
        }
    }

    /// Collects (pid, display_name, kind) for all processes we want to monitor.
    fn collect_pids_and_names(
        &self,
        cx: &App,
    ) -> Vec<(u32, SharedString, ProcessKind)> {
        let mut result = Vec::new();

        // Zed main process
        if let Ok(pid) = sysinfo::get_current_pid() {
            result.push((pid.as_u32(), "Zed".into(), ProcessKind::ZedMain));
        }

        // Language server PIDs
        let lsp_store = self.project.read(cx).lsp_store().read(cx);
        for (_id, status) in lsp_store.language_server_statuses() {
            if let Some(pid) = status.process_id {
                result.push((pid, status.name.0.clone(), ProcessKind::LanguageServer));
            }
        }

        result
    }

    fn apply_snapshots(
        &mut self,
        mut snapshots: Vec<ProcessSnapshot>,
        has_sampled_twice: bool,
        cx: &mut Context<Self>,
    ) {
        // Sort: Zed first, then language servers alphabetically
        snapshots.sort_by(|a, b| a.kind.cmp(&b.kind).then_with(|| a.name.cmp(&b.name)));
        self.snapshots = snapshots;
        self.has_sampled_twice = has_sampled_twice;
        cx.notify();
    }

    fn copy_report(&self, cx: &mut Context<Self>) {
        let mut report = String::from("Zed Resource Monitor Report\n");
        report.push_str(&"=".repeat(60));
        report.push('\n');
        report.push_str(&format!(
            "{:<30} {:>8} {:>8} {:>10}\n",
            "Name", "PID", "CPU %", "Memory"
        ));
        report.push_str(&"-".repeat(60));
        report.push('\n');

        for snap in &self.snapshots {
            let cpu = if self.has_sampled_twice {
                format!("{:.1}%", snap.cpu_percent)
            } else {
                "—".to_string()
            };
            report.push_str(&format!(
                "{:<30} {:>8} {:>8} {:>10}\n",
                snap.name.as_ref(),
                snap.pid,
                cpu,
                format_memory(snap.memory_bytes),
            ));
        }

        cx.write_to_clipboard(ClipboardItem::new_string(report));
    }

    fn force_refresh(&mut self, cx: &mut Context<Self>) {
        // Restart the poll task to get an immediate refresh
        let project = self.project.clone();
        self._poll_task = cx.spawn(async move |this, mut cx| {
            let mut system = System::new();
            let refresh_kind = ProcessRefreshKind::nothing()
                .with_cpu()
                .with_memory()
                .without_tasks();

            // Do one immediate refresh, then continue the regular loop.
            // We need two samples for CPU, so we do a quick double-tap.
            for i in 0u32..2 {
                let pids_and_names = match this.update(cx, |this, cx| {
                    this.collect_pids_and_names(cx)
                }) {
                    Ok(v) => v,
                    Err(_) => return,
                };

                if !pids_and_names.is_empty() {
                    let pids: Vec<Pid> = pids_and_names
                        .iter()
                        .map(|(pid, _, _)| Pid::from_u32(*pid))
                        .collect();

                    system.refresh_processes_specifics(
                        ProcessesToUpdate::Some(&pids),
                        true,
                        refresh_kind,
                    );

                    if i == 1 {
                        let snapshots: Vec<ProcessSnapshot> = pids_and_names
                            .into_iter()
                            .filter_map(|(pid, name, kind)| {
                                let proc = system.process(Pid::from_u32(pid))?;
                                Some(ProcessSnapshot {
                                    name,
                                    pid,
                                    cpu_percent: proc.cpu_usage(),
                                    memory_bytes: proc.memory(),
                                    kind,
                                })
                            })
                            .collect();

                        if this
                            .update(cx, |this, cx| {
                                this.apply_snapshots(snapshots, true, cx);
                            })
                            .is_err()
                        {
                            return;
                        }
                    }
                }

                if i == 0 {
                    // Wait a short interval between two samples for CPU diff
                    cx.background_executor()
                        .timer(Duration::from_millis(500))
                        .await;
                }
            }

            // Continue regular polling loop
            let mut sample_count: u32 = 2;
            loop {
                cx.background_executor().timer(POLL_INTERVAL).await;

                let pids_and_names = match this.update(cx, |this, cx| {
                    this.collect_pids_and_names(cx)
                }) {
                    Ok(v) => v,
                    Err(_) => break,
                };

                if pids_and_names.is_empty() {
                    continue;
                }

                let pids: Vec<Pid> = pids_and_names
                    .iter()
                    .map(|(pid, _, _)| Pid::from_u32(*pid))
                    .collect();

                system.refresh_processes_specifics(
                    ProcessesToUpdate::Some(&pids),
                    true,
                    refresh_kind,
                );
                sample_count = sample_count.saturating_add(1);

                let snapshots: Vec<ProcessSnapshot> = pids_and_names
                    .into_iter()
                    .filter_map(|(pid, name, kind)| {
                        let proc = system.process(Pid::from_u32(pid))?;
                        Some(ProcessSnapshot {
                            name,
                            pid,
                            cpu_percent: proc.cpu_usage(),
                            memory_bytes: proc.memory(),
                            kind,
                        })
                    })
                    .collect();

                if this
                    .update(cx, |this, cx| {
                        this.apply_snapshots(snapshots, sample_count >= 2, cx);
                    })
                    .is_err()
                {
                    break;
                }
            }
        });

        // Stash the project reference back (force_refresh replaces _poll_task)
        self.project = project;
    }
}

fn format_memory(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

impl EventEmitter<()> for ResourceMonitor {}

impl Focusable for ResourceMonitor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ResourceMonitor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let snapshots = self.snapshots.clone();
        let has_cpu_data = self.has_sampled_twice;

        v_flex()
            .id("resource-monitor")
            .key_context("ResourceMonitor")
            .track_focus(&self.focus_handle)
            .size_full()
            .overflow_y_scroll()
            .p_4()
            .gap_1()
            // Header row
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .pb_2()
                    .child(Label::new("Resource Monitor").size(LabelSize::Large))
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("copy-report", "Copy Report")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.copy_report(cx);
                                    })),
                            )
                            .child(
                                Button::new("refresh", "Refresh")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.force_refresh(cx);
                                    })),
                            ),
                    ),
            )
            // Table header
            .child(
                h_flex()
                    .px_2()
                    .py_1()
                    .gap_2()
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(120.))
                            .child(Label::new("Name").size(LabelSize::Small).color(Color::Muted)),
                    )
                    .child(
                        div()
                            .w(px(80.))
                            .child(Label::new("PID").size(LabelSize::Small).color(Color::Muted)),
                    )
                    .child(
                        div()
                            .w(px(80.))
                            .child(Label::new("CPU %").size(LabelSize::Small).color(Color::Muted)),
                    )
                    .child(
                        div().w(px(100.)).child(
                            Label::new("Memory").size(LabelSize::Small).color(Color::Muted),
                        ),
                    ),
            )
            // Separator
            .child(div().h(px(1.)).bg(cx.theme().colors().border))
            // Process rows
            .children(snapshots.into_iter().enumerate().map(|(i, snap)| {
                let cpu_text = if has_cpu_data {
                    format!("{:.1}%", snap.cpu_percent)
                } else {
                    "—".to_string()
                };
                let mem_text = format_memory(snap.memory_bytes);

                h_flex()
                    .id(ElementId::NamedInteger("proc-row".into(), i as u64))
                    .px_2()
                    .py_1()
                    .gap_2()
                    .rounded_md()
                    .hover(|s| s.bg(cx.theme().colors().ghost_element_hover))
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(120.))
                            .child(Label::new(snap.name).size(LabelSize::Small)),
                    )
                    .child(div().w(px(80.)).child(
                        Label::new(snap.pid.to_string())
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ))
                    .child(
                        div()
                            .w(px(80.))
                            .child(Label::new(cpu_text).size(LabelSize::Small)),
                    )
                    .child(
                        div()
                            .w(px(100.))
                            .child(Label::new(mem_text).size(LabelSize::Small)),
                    )
            }))
            // Empty state
            .when(self.snapshots.is_empty(), |el| {
                el.child(
                    div().py_4().child(
                        Label::new("No processes found. Waiting for data…")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
                )
            })
    }
}

impl Item for ResourceMonitor {
    type Event = ();

    fn to_item_events(_: &Self::Event, _: &mut dyn FnMut(workspace::item::ItemEvent)) {}

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Resource Monitor".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn can_split(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        Task::ready(Some(
            cx.new(|cx| ResourceMonitor::new(self.project.clone(), window, cx)),
        ))
    }
}
