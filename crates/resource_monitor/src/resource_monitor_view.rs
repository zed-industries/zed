use std::any::TypeId;

use gpui::{
    App, ClipboardItem, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ParentElement, Render, SharedString, Styled, Subscription, Task, Window, div,
};
use human_bytes::human_bytes;
use project::Project;
use ui::prelude::*;
use workspace::{Item, WorkspaceId, item::ItemEvent};

use crate::process_collector::{
    ProcessActions, ProcessCategory, ProcessCollector, ProcessEntry, ProcessSnapshot,
};

/// Column to sort by.
#[derive(Debug, Clone, Copy, PartialEq)]
enum SortColumn {
    Name,
    Pid,
    Cpu,
    Memory,
}

pub struct ResourceMonitorView {
    collector: Entity<ProcessCollector>,
    focus_handle: FocusHandle,
    sort_column: SortColumn,
    sort_ascending: bool,
    _subscription: Subscription,
}

#[derive(Debug, Clone)]
pub enum ResourceMonitorEvent {
    // Placeholder for future events (e.g. close).
}

impl EventEmitter<ItemEvent> for ResourceMonitorView {}
impl EventEmitter<ResourceMonitorEvent> for ResourceMonitorView {}

impl ResourceMonitorView {
    pub fn new(
        project: Entity<Project>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let collector = cx.new(|cx| ProcessCollector::new(project, cx));
        let subscription = cx.observe(&collector, |_, _, cx| cx.notify());
        let focus_handle = cx.focus_handle();

        Self {
            collector,
            focus_handle,
            sort_column: SortColumn::Cpu,
            sort_ascending: false,
            _subscription: subscription,
        }
    }

    fn sorted_entries(&self, snapshot: &ProcessSnapshot) -> Vec<ProcessEntry> {
        let mut entries = snapshot.entries.clone();
        let ascending = self.sort_ascending;
        match self.sort_column {
            SortColumn::Name => entries.sort_by(|a, b| {
                let cmp = a.name.to_lowercase().cmp(&b.name.to_lowercase());
                if ascending { cmp } else { cmp.reverse() }
            }),
            SortColumn::Pid => entries.sort_by(|a, b| {
                let cmp = a.pid.cmp(&b.pid);
                if ascending { cmp } else { cmp.reverse() }
            }),
            SortColumn::Cpu => entries.sort_by(|a, b| {
                let cmp = a
                    .cpu_percent
                    .partial_cmp(&b.cpu_percent)
                    .unwrap_or(std::cmp::Ordering::Equal);
                if ascending { cmp } else { cmp.reverse() }
            }),
            SortColumn::Memory => entries.sort_by(|a, b| {
                let cmp = a.memory_bytes.cmp(&b.memory_bytes);
                if ascending { cmp } else { cmp.reverse() }
            }),
        }
        entries
    }

    fn toggle_sort(&mut self, column: SortColumn, cx: &mut Context<Self>) {
        if self.sort_column == column {
            self.sort_ascending = !self.sort_ascending;
        } else {
            self.sort_column = column;
            self.sort_ascending = false;
        }
        cx.notify();
    }

    fn copy_report(&self, _window: &mut Window, cx: &mut Context<Self>) {
        let snapshot = self.collector.read(cx).snapshot().clone();
        let entries = self.sorted_entries(&snapshot);

        let mut report = String::new();
        report.push_str("Zed Resource Monitor Report\n");
        report.push_str(&format!(
            "Total: {:.1}% CPU · {}\n\n",
            snapshot.total_cpu_percent,
            human_bytes(snapshot.total_memory_bytes as f64)
        ));

        let mut current_category: Option<ProcessCategory> = None;
        for entry in &entries {
            if current_category != Some(entry.category) {
                current_category = Some(entry.category);
                report.push_str(&format!("{}:\n", entry.category.label()));
            }
            let ctx = entry
                .context
                .as_ref()
                .map(|c| format!(" ({})", c))
                .unwrap_or_default();
            report.push_str(&format!(
                "  {}{:<30} PID:{:<8} CPU:{:>5.1}%  Mem:{}\n",
                entry.name,
                ctx,
                entry.pid.map_or("—".to_string(), |p| p.to_string()),
                entry.cpu_percent,
                human_bytes(entry.memory_bytes as f64),
            ));
        }

        cx.write_to_clipboard(ClipboardItem::new_string(report));
    }

    fn render_header(
        &self,
        snapshot: &ProcessSnapshot,
        cx: &App,
    ) -> impl IntoElement {
        let process_count = snapshot.entries.len();

        h_flex()
            .id("resource-monitor-header")
            .px_3()
            .py_2()
            .w_full()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Icon::new(IconName::Sliders)
                            .size(IconSize::Small)
                            .color(Color::Accent),
                    )
                    .child(
                        Label::new("Resource Monitor")
                            .size(LabelSize::Default)
                            .weight(FontWeight::SEMIBOLD),
                    )
                    .child(
                        Label::new(format!(
                            "— {} process{} · {:.1}% CPU · {}",
                            process_count,
                            if process_count == 1 { "" } else { "es" },
                            snapshot.total_cpu_percent,
                            human_bytes(snapshot.total_memory_bytes as f64),
                        ))
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                    ),
            )
    }

    fn sort_indicator(&self, column: SortColumn) -> &'static str {
        if self.sort_column == column {
            if self.sort_ascending {
                " ▲"
            } else {
                " ▼"
            }
        } else {
            ""
        }
    }

    fn render_column_headers(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let col_name = format!("Name{}", self.sort_indicator(SortColumn::Name));
        let col_pid = format!("PID{}", self.sort_indicator(SortColumn::Pid));
        let col_cpu = format!("CPU{}", self.sort_indicator(SortColumn::Cpu));
        let col_mem = format!("Memory{}", self.sort_indicator(SortColumn::Memory));

        h_flex()
            .id("column-headers")
            .px_3()
            .py_1()
            .w_full()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .bg(cx.theme().colors().surface_background)
            .child(
                div()
                    .id("col-name-header")
                    .flex_1()
                    .min_w(px(140.))
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _window, cx| this.toggle_sort(SortColumn::Name, cx)))
                    .child(Label::new(col_name).size(LabelSize::Small).color(Color::Muted)),
            )
            .child(
                div()
                    .id("col-pid-header")
                    .w(px(70.))
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _window, cx| this.toggle_sort(SortColumn::Pid, cx)))
                    .child(Label::new(col_pid).size(LabelSize::Small).color(Color::Muted)),
            )
            .child(
                div()
                    .id("col-cpu-header")
                    .w(px(70.))
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _window, cx| this.toggle_sort(SortColumn::Cpu, cx)))
                    .child(Label::new(col_cpu).size(LabelSize::Small).color(Color::Muted)),
            )
            .child(
                div()
                    .id("col-mem-header")
                    .w(px(90.))
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _window, cx| this.toggle_sort(SortColumn::Memory, cx)))
                    .child(Label::new(col_mem).size(LabelSize::Small).color(Color::Muted)),
            )
    }

    fn render_category_header(
        &self,
        category: ProcessCategory,
        cx: &App,
    ) -> impl IntoElement {
        h_flex()
            .id(SharedString::from(format!("category-{:?}", category)))
            .px_3()
            .py_1()
            .w_full()
            .bg(cx.theme().colors().surface_background)
            .child(
                Label::new(category.label())
                    .size(LabelSize::Small)
                    .weight(FontWeight::SEMIBOLD)
                    .color(Color::Muted),
            )
    }

    fn render_row(
        &self,
        entry: &ProcessEntry,
        row_index: usize,
        cx: &App,
    ) -> impl IntoElement {
        let cpu_color = if entry.cpu_percent > 50.0 {
            Color::Error
        } else if entry.cpu_percent > 20.0 {
            Color::Warning
        } else {
            Color::Muted
        };

        let mem_color = if entry.memory_bytes > 2 * 1024 * 1024 * 1024 {
            Color::Error
        } else if entry.memory_bytes > 500 * 1024 * 1024 {
            Color::Warning
        } else {
            Color::Default
        };

        h_flex()
            .id(ElementId::NamedInteger(
                "process-row".into(),
                row_index,
            ))
            .px_3()
            .py(px(3.))
            .w_full()
            .hover(|s| s.bg(cx.theme().colors().ghost_element_hover))
            // Name column
            .child(
                h_flex()
                    .flex_1()
                    .min_w(px(140.))
                    .gap_1()
                    .child(self.category_icon(entry.category, cx))
                    .child(
                        Label::new(entry.name.clone())
                            .size(LabelSize::Small),
                    )
                    .when_some(entry.context.as_ref(), |el, ctx| {
                        el.child(
                            Label::new(format!("({})", ctx))
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        )
                    }),
            )
            // PID column
            .child(
                div()
                    .w(px(70.))
                    .child(
                        Label::new(
                            entry
                                .pid
                                .map_or("—".to_string(), |p| p.to_string()),
                        )
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                    ),
            )
            // CPU column
            .child(
                div()
                    .w(px(70.))
                    .child(
                        Label::new(format!("{:.1}%", entry.cpu_percent))
                            .size(LabelSize::Small)
                            .color(cpu_color),
                    ),
            )
            // Memory column
            .child(
                div()
                    .w(px(90.))
                    .child(
                        Label::new(human_bytes(entry.memory_bytes as f64))
                            .size(LabelSize::Small)
                            .color(mem_color),
                    ),
            )
    }

    fn category_icon(&self, category: ProcessCategory, _cx: &App) -> impl IntoElement {
        let (icon, color) = match category {
            ProcessCategory::MainProcess => (IconName::Server, Color::Accent),
            ProcessCategory::LanguageServer => (IconName::FileCode, Color::Info),
            ProcessCategory::Terminal => (IconName::Terminal, Color::Default),
        };
        Icon::new(icon).size(IconSize::Small).color(color)
    }
}

impl Render for ResourceMonitorView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let snapshot = self.collector.read(cx).snapshot().clone();
        let entries = self.sorted_entries(&snapshot);

        let mut children: Vec<gpui::AnyElement> = Vec::new();

        let mut current_category: Option<ProcessCategory> = None;
        for (i, entry) in entries.iter().enumerate() {
            if current_category != Some(entry.category) {
                current_category = Some(entry.category);
                children.push(self.render_category_header(entry.category, cx).into_any_element());
            }
            children.push(self.render_row(entry, i, cx).into_any_element());
        }

        if entries.is_empty() {
            children.push(
                div()
                    .id("empty-state")
                    .p_4()
                    .child(
                        Label::new("Collecting process data…")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .into_any_element(),
            );
        }

        div()
            .id("resource-monitor")
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().colors().background)
            .child(self.render_header(&snapshot, cx))
            .child(
                h_flex()
                    .px_3()
                    .py_1()
                    .gap_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        Button::new("copy-report", "Copy Report")
                            .style(ButtonStyle::Subtle)
                            .size(ButtonSize::Compact)
                            .icon(IconName::Copy)
                            .icon_size(IconSize::Small)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.copy_report(window, cx);
                            })),
                    ),
            )
            .child(self.render_column_headers(cx))
            .child(
                div()
                    .id("process-rows-scroll")
                    .flex_grow()
                    .overflow_y_scroll()
                    .children(children),
            )
    }
}

impl Focusable for ResourceMonitorView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ResourceMonitorView {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Resource Monitor".into()
    }

    fn tab_icon(&self, _cx: &App) -> Option<IconName> {
        Some(IconName::Sliders)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Resource Monitor Opened")
    }

    fn can_split(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        Task::ready(None)
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else {
            None
        }
    }
}
