use agent::TerminalJobManager;
use gpui::{
    App, AppContext, Entity, EventEmitter, IntoElement, ParentElement, Render, Styled,
    Subscription, Task, ViewContext, WeakEntity, div, prelude::*,
};
use std::time::{Duration, SystemTime};
use ui::{
    Button, ButtonCommon, ButtonSize, ButtonStyle, Icon, IconName, IconSize, Label, Tooltip,
    h_flex, v_flex,
};

/// Panel showing running terminal jobs
pub struct TerminalJobsPanel {
    expanded: bool,
    job_cards: Vec<JobCardState>,
    _update_task: Task<()>,
}

#[derive(Clone)]
struct JobCardState {
    job_id: String,
    command: String,
    working_dir: String,
    status: String,
    start_time: SystemTime,
    exit_code: Option<i32>,
    output_visible: bool,
    output: String,
}

impl TerminalJobsPanel {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let this = cx.view().downgrade();

        // Spawn background task to poll for job updates
        let update_task = cx.spawn(|this, mut cx| async move {
            loop {
                cx.background_executor().timer(Duration::from_secs(1)).await;

                if let Some(this) = this.upgrade() {
                    let _ = this.update(&mut cx, |this, cx| {
                        this.refresh_jobs(cx);
                        cx.notify();
                    });
                } else {
                    break;
                }
            }
        });

        Self {
            expanded: false,
            job_cards: Vec::new(),
            _update_task: update_task,
        }
    }

    fn refresh_jobs(&mut self, cx: &mut ViewContext<Self>) {
        let job_manager = TerminalJobManager::global(cx);
        let jobs = job_manager.list_jobs_filtered(
            Some(&[
                agent::TerminalJobStatus::Running,
                agent::TerminalJobStatus::Completed,
                agent::TerminalJobStatus::Failed,
            ]),
            Some(10),
        );

        self.job_cards = jobs
            .into_iter()
            .map(|job| {
                let status = match job.status {
                    agent::TerminalJobStatus::Running => "running".to_string(),
                    agent::TerminalJobStatus::Completed => "completed".to_string(),
                    agent::TerminalJobStatus::Failed => "failed".to_string(),
                    agent::TerminalJobStatus::Canceled => "canceled".to_string(),
                };

                // Find existing card to preserve output_visible state
                let output_visible = self
                    .job_cards
                    .iter()
                    .find(|c| c.job_id == job.job_id)
                    .map(|c| c.output_visible)
                    .unwrap_or(false);

                JobCardState {
                    job_id: job.job_id,
                    command: job.command,
                    working_dir: job.working_dir,
                    status,
                    start_time: job.started_at,
                    exit_code: job.exit_code,
                    output_visible,
                    output: job.output,
                }
            })
            .collect();

        // Auto-expand if we have running jobs
        if !self.expanded && self.has_running_jobs() {
            self.expanded = true;
        }
    }

    fn has_running_jobs(&self) -> bool {
        self.job_cards.iter().any(|job| job.status == "running")
    }

    fn running_count(&self) -> usize {
        self.job_cards
            .iter()
            .filter(|job| job.status == "running")
            .count()
    }

    fn toggle_expanded(&mut self, cx: &mut ViewContext<Self>) {
        self.expanded = !self.expanded;
        cx.notify();
    }

    fn toggle_job_output(&mut self, job_id: &str, cx: &mut ViewContext<Self>) {
        if let Some(card) = self.job_cards.iter_mut().find(|c| c.job_id == job_id) {
            card.output_visible = !card.output_visible;
            cx.notify();
        }
    }

    fn cancel_job(&mut self, job_id: &str, cx: &mut ViewContext<Self>) {
        let job_manager = TerminalJobManager::global(cx);
        let cx_async = cx.to_async();

        cx.spawn(|this, mut cx| async move {
            let _ = job_manager.cancel_job(job_id, &cx_async);

            // Refresh after canceling
            if let Some(this) = this.upgrade() {
                let _ = this.update(&mut cx, |this, cx| {
                    this.refresh_jobs(cx);
                    cx.notify();
                });
            }
        })
        .detach();
    }

    fn dismiss_job(&mut self, job_id: &str, cx: &mut ViewContext<Self>) {
        self.job_cards.retain(|card| card.job_id != job_id);
        cx.notify();
    }

    fn format_duration(start: SystemTime) -> String {
        if let Ok(duration) = SystemTime::now().duration_since(start) {
            let secs = duration.as_secs();
            if secs < 60 {
                format!("{}s", secs)
            } else if secs < 3600 {
                format!("{}m {}s", secs / 60, secs % 60)
            } else {
                format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
            }
        } else {
            "?".to_string()
        }
    }

    fn status_icon(status: &str) -> IconName {
        match status {
            "running" => IconName::Play,
            "completed" => IconName::Check,
            "failed" => IconName::Close,
            "canceled" => IconName::Stop,
            _ => IconName::Terminal,
        }
    }

    fn status_color(status: &str) -> ui::Color {
        match status {
            "running" => ui::Color::Accent,
            "completed" => ui::Color::Success,
            "failed" => ui::Color::Error,
            "canceled" => ui::Color::Muted,
            _ => ui::Color::Default,
        }
    }

    fn render_collapsed(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let running = self.running_count();
        let total = self.job_cards.len();

        if total == 0 {
            return div().into_any_element();
        }

        let text = if running > 0 {
            format!(
                "⚡ {} job{} running",
                running,
                if running == 1 { "" } else { "s" }
            )
        } else {
            format!(
                "✓ {} job{} completed",
                total,
                if total == 1 { "" } else { "s" }
            )
        };

        h_flex()
            .w_full()
            .px_2()
            .py_1()
            .bg(cx.theme().colors().editor_background)
            .border_y_1()
            .border_color(cx.theme().colors().border)
            .items_center()
            .justify_between()
            .child(
                Label::new(text)
                    .size(ui::LabelSize::Small)
                    .color(ui::Color::Muted),
            )
            .child(
                Button::new("expand-jobs", "Expand")
                    .icon(IconName::ChevronDown)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .size(ButtonSize::Compact)
                    .on_click(cx.listener(|this, _, cx| {
                        this.toggle_expanded(cx);
                    })),
            )
            .into_any_element()
    }

    fn render_expanded(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if self.job_cards.is_empty() {
            return div().into_any_element();
        }

        v_flex()
            .w_full()
            .bg(cx.theme().colors().editor_background)
            .border_y_1()
            .border_color(cx.theme().colors().border)
            .child(
                // Header
                h_flex()
                    .w_full()
                    .px_2()
                    .py_1()
                    .items_center()
                    .justify_between()
                    .child(
                        Label::new(format!("⚡ RUNNING JOBS ({})", self.job_cards.len()))
                            .size(ui::LabelSize::Small)
                            .color(ui::Color::Muted),
                    )
                    .child(
                        Button::new("collapse-jobs", "Collapse")
                            .icon(IconName::ChevronUp)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .size(ButtonSize::Compact)
                            .on_click(cx.listener(|this, _, cx| {
                                this.toggle_expanded(cx);
                            })),
                    ),
            )
            .child(
                // Job list
                v_flex().w_full().gap_1().p_2().children(
                    self.job_cards
                        .iter()
                        .map(|job| self.render_job_card(job, cx)),
                ),
            )
            .into_any_element()
    }

    fn render_job_card(&self, job: &JobCardState, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let job_id = job.job_id.clone();
        let job_id_for_output = job.job_id.clone();
        let job_id_for_cancel = job.job_id.clone();
        let job_id_for_dismiss = job.job_id.clone();

        let is_running = job.status == "running";
        let status_icon = Self::status_icon(&job.status);
        let status_color = Self::status_color(&job.status);

        v_flex()
            .w_full()
            .p_2()
            .bg(cx.theme().colors().element_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .gap_1()
            .child(
                // Header row
                h_flex()
                    .w_full()
                    .items_center()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(
                                Icon::new(status_icon)
                                    .size(IconSize::Small)
                                    .color(status_color),
                            )
                            .child(
                                Label::new(job.job_id.clone())
                                    .size(ui::LabelSize::Small)
                                    .color(ui::Color::Muted),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .when(!job.output_visible, |this| {
                                this.child(
                                    Button::new(format!("view-{}", job_id), "View")
                                        .icon(IconName::Eye)
                                        .icon_size(IconSize::Small)
                                        .style(ButtonStyle::Subtle)
                                        .size(ButtonSize::Compact)
                                        .tooltip(|cx| Tooltip::text("Show output", cx))
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.toggle_job_output(&job_id_for_output, cx);
                                        })),
                                )
                            })
                            .when(job.output_visible, |this| {
                                this.child(
                                    Button::new(format!("hide-{}", job_id), "Hide")
                                        .icon(IconName::EyeClosed)
                                        .icon_size(IconSize::Small)
                                        .style(ButtonStyle::Subtle)
                                        .size(ButtonSize::Compact)
                                        .tooltip(|cx| Tooltip::text("Hide output", cx))
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.toggle_job_output(&job_id_for_output, cx);
                                        })),
                                )
                            })
                            .when(is_running, |this| {
                                this.child(
                                    Button::new(format!("cancel-{}", job_id), "Cancel")
                                        .icon(IconName::Close)
                                        .icon_size(IconSize::Small)
                                        .style(ButtonStyle::Subtle)
                                        .size(ButtonSize::Compact)
                                        .tooltip(|cx| Tooltip::text("Cancel and kill process", cx))
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.cancel_job(&job_id_for_cancel, cx);
                                        })),
                                )
                            })
                            .when(!is_running, |this| {
                                this.child(
                                    Button::new(format!("dismiss-{}", job_id), "Dismiss")
                                        .icon(IconName::Close)
                                        .icon_size(IconSize::Small)
                                        .style(ButtonStyle::Subtle)
                                        .size(ButtonSize::Compact)
                                        .tooltip(|cx| Tooltip::text("Remove from list", cx))
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.dismiss_job(&job_id_for_dismiss, cx);
                                        })),
                                )
                            }),
                    ),
            )
            .child(
                // Command
                Label::new(format!("$ {}", job.command))
                    .size(ui::LabelSize::Small)
                    .color(ui::Color::Default),
            )
            .child(
                // Status line
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        Label::new(format!("⏱️ {}", Self::format_duration(job.start_time)))
                            .size(ui::LabelSize::XSmall)
                            .color(ui::Color::Muted),
                    )
                    .child(
                        Label::new(format!("📁 {}", job.working_dir))
                            .size(ui::LabelSize::XSmall)
                            .color(ui::Color::Muted),
                    )
                    .child(
                        Label::new(format!(
                            "📊 {}{}",
                            job.status,
                            job.exit_code
                                .map(|code| format!(" ({})", code))
                                .unwrap_or_default()
                        ))
                        .size(ui::LabelSize::XSmall)
                        .color(status_color),
                    ),
            )
            .when(job.output_visible, |this| {
                this.child(
                    // Output box
                    div()
                        .w_full()
                        .max_h_64()
                        .overflow_y_scroll()
                        .p_2()
                        .bg(cx.theme().colors().editor_background)
                        .border_1()
                        .border_color(cx.theme().colors().border)
                        .rounded_md()
                        .child(if job.output.is_empty() {
                            Label::new("No output yet.")
                                .size(ui::LabelSize::Small)
                                .color(ui::Color::Muted)
                                .into_any_element()
                        } else {
                            Label::new(job.output.clone())
                                .size(ui::LabelSize::Small)
                                .color(ui::Color::Default)
                                .into_any_element()
                        }),
                )
            })
    }
}

impl Render for TerminalJobsPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if self.job_cards.is_empty() {
            return div().into_any_element();
        }

        if self.expanded {
            self.render_expanded(cx)
        } else {
            self.render_collapsed(cx)
        }
    }
}

impl EventEmitter<()> for TerminalJobsPanel {}
