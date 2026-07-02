use std::time::Duration;

use gpui::{
    actions, div, AnyElement, App, ClipboardItem, Context, Entity, EventEmitter, FocusHandle,
    Focusable, Hsla, IntoElement, ParentElement, Render, SharedString, Styled, Task, Window,
};
use project::Project;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};
use ui::prelude::*;
use workspace::{Item, SplitDirection, Workspace, WorkspaceId};

use crate::get_or_create_tool;

/// Major UI improvements:
/// - Summary header with process/CPU/memory totals + action buttons
/// - Section grouping ("Zed" highlighted first, then "Language Servers")
/// - Main table implemented with GPUI `.grid().grid_cols(5)` + col_span for perfect column alignment
/// - Improved padding, hovers (`ghost_element_hover`), header separators
/// - Color coding: CPU (Muted / Warning / Error), high memory red tint on row
/// - Clean, professional Zed-style appearance, better empty state
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

/// Returns semantic color for CPU usage display.
fn cpu_color(percent: f32) -> Color {
    if percent > 50.0 {
        Color::Error
    } else if percent >= 20.0 {
        Color::Warning
    } else {
        Color::Muted
    }
}

/// Threshold for visually highlighting high memory usage per process.
const HIGH_MEMORY_THRESHOLD: u64 = 800 * 1024 * 1024; // ~800 MB

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

        // Compute summary aggregates
        let num_processes = snapshots.len();
        let total_cpu: f32 = if has_cpu_data {
            snapshots.iter().map(|s| s.cpu_percent).sum()
        } else {
            0.0
        };
        let total_memory: u64 = snapshots.iter().map(|s| s.memory_bytes).sum();

        let cpu_summary = if has_cpu_data {
            format!("{:.1}%", total_cpu)
        } else {
            "—".to_string()
        };
        let summary = format!(
            "Resource Monitor — {} process{}  {} CPU  {}",
            num_processes,
            if num_processes == 1 { "" } else { "es" },
            cpu_summary,
            format_memory(total_memory)
        );

        // Partition for grouped sections (Zed first)
        let (zed_snapshots, ls_snapshots): (Vec<ProcessSnapshot>, Vec<ProcessSnapshot>) =
            snapshots
                .into_iter()
                .partition(|s| s.kind == ProcessKind::ZedMain);

        let has_data = !zed_snapshots.is_empty() || !ls_snapshots.is_empty();

        v_flex()
            .id("resource-monitor")
            .key_context("ResourceMonitor")
            .track_focus(&self.focus_handle)
            .size_full()
            .overflow_y_scroll()
            .pb_4()
            // === Top Summary Header ===
            .child(
                h_flex()
                    .px_3()
                    .py_2()
                    .justify_between()
                    .items_center()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(Label::new(summary).size(LabelSize::Default))
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Button::new("copy-report", "Copy Report")
                                    .style(ButtonStyle::Filled)
                                    .label_size(LabelSize::Small)
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.copy_report(cx);
                                    })),
                            )
                            .child(
                                Button::new("refresh", "Refresh")
                                    .style(ButtonStyle::Filled)
                                    .label_size(LabelSize::Small)
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.force_refresh(cx);
                                    })),
                            ),
                    ),
            )
            // === Zed Section ===
            .when(!zed_snapshots.is_empty(), |this| {
                this.child(
                    div()
                        .px_2()
                        .py_1()
                        .mt_2()
                        .rounded_sm()
                        .bg(cx.theme().colors().ghost_element_selected.opacity(0.1))
                        .child(
                            Label::new("Zed")
                                .size(LabelSize::Small)
                                .color(Color::Accent),
                        ),
                )
                .child(self.render_grid(&zed_snapshots, has_cpu_data, true, cx))
            })
            // === Language Servers Section ===
            .when(!ls_snapshots.is_empty(), |this| {
                this.child(
                    div()
                        .px_2()
                        .py_1()
                        .mt_2()
                        .child(
                            Label::new("Language Servers")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
                .child(self.render_grid(&ls_snapshots, has_cpu_data, false, cx))
            })
            // === Empty State ===
            .when(!has_data, |this| {
                this.child(
                    v_flex()
                        .py_8()
                        .items_center()
                        .justify_center()
                        .size_full()
                        .child(
                            Label::new("No processes found")
                                .size(LabelSize::Default)
                                .color(Color::Muted),
                        )
                        .child(
                            Label::new("Waiting for Zed and language servers…")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
            })
    }
}

impl ResourceMonitor {
    /// Renders a table using GPUI Grid (.grid().grid_cols(5)) for perfect alignment.
    /// Name column spans 2 tracks so it has more room; numeric columns are right-aligned.
    fn render_grid(
        &self,
        snaps: &[ProcessSnapshot],
        has_cpu_data: bool,
        is_zed_section: bool,
        cx: &App,
    ) -> impl IntoElement {
        let mut children: Vec<AnyElement> = Vec::new();

        // Header row via Grid (name header spans like the data cells)
        children.push(
            self.grid_header_cell("Name", false, cx)
                .col_span(2)
                .into_any_element(),
        );
        children.push(self.grid_header_cell("PID", true, cx).into_any_element());
        children.push(self.grid_header_cell("CPU %", true, cx).into_any_element());
        children.push(self.grid_header_cell("Memory", true, cx).into_any_element());

        // Data rows (flat grid children = perfect columns)
        for (i, snap) in snaps.iter().enumerate() {
            let cpu_text = if has_cpu_data {
                format!("{:.1}%", snap.cpu_percent)
            } else {
                "—".to_string()
            };
            let mem_text = format_memory(snap.memory_bytes);

            let cpu_col = if has_cpu_data {
                cpu_color(snap.cpu_percent)
            } else {
                Color::Muted
            };

            let high_mem = snap.memory_bytes > HIGH_MEMORY_THRESHOLD;
            let zed_row = is_zed_section;

            let base_bg = if zed_row {
                cx.theme().colors().ghost_element_selected.opacity(0.035)
            } else if high_mem {
                cx.theme().status().warning_background.opacity(0.06)
            } else {
                Hsla::default()
            };

            let hover_bg = if high_mem {
                cx.theme().status().warning_background.opacity(0.13)
            } else {
                cx.theme().colors().ghost_element_hover
            };

            let row_base = (if zed_row { 0u64 } else { 1000 }) + (i as u64 * 4);

            // Name (spans two tracks for better proportion)
            children.push(
                div()
                    .id(ElementId::NamedInteger("grid-cell".into(), row_base))
                    .col_span(2)
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .bg(base_bg)
                    .hover(|s| s.bg(hover_bg))
                    .child(
                        Label::new(snap.name.clone())
                            .size(LabelSize::Small)
                            .color(Color::Default),
                    )
                    .into_any_element(),
            );

            // PID
            children.push(
                div()
                    .id(ElementId::NamedInteger("grid-cell".into(), row_base + 1))
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .bg(base_bg)
                    .hover(|s| s.bg(hover_bg))
                    .text_right()
                    .child(
                        Label::new(snap.pid.to_string())
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .into_any_element(),
            );

            // CPU %
            children.push(
                div()
                    .id(ElementId::NamedInteger("grid-cell".into(), row_base + 2))
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .bg(base_bg)
                    .hover(|s| s.bg(hover_bg))
                    .text_right()
                    .child(
                        Label::new(cpu_text)
                            .size(LabelSize::Small)
                            .color(cpu_col),
                    )
                    .into_any_element(),
            );

            // Memory
            children.push(
                div()
                    .id(ElementId::NamedInteger("grid-cell".into(), row_base + 3))
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .bg(base_bg)
                    .hover(|s| s.bg(hover_bg))
                    .text_right()
                    .child(
                        Label::new(mem_text)
                            .size(LabelSize::Small)
                            .color(if high_mem { Color::Warning } else { Color::Muted }),
                    )
                    .into_any_element(),
            );
        }

        div()
            .grid()
            .grid_cols(5)
            .w_full()
            .children(children)
    }

    fn grid_header_cell(&self, label: &str, right: bool, cx: &App) -> Div {
        let cell = div()
            .px_2()
            .py_1()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                Label::new(label)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            );
        if right { cell.text_right() } else { cell }
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
