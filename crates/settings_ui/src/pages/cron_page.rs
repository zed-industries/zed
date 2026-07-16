//! Cron Jobs settings page — view, add, pause/resume, and delete
//! scheduled agent tasks. Reads from `~/.zed/cron.json`.

use gpui::{ScrollHandle, prelude::*};
use ui::{prelude::*, *};

use crate::SettingsWindow;
use crate::page_data::SubPageLink;

pub(crate) fn render_cron_page(
    _settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let store = agent::scheduler::global_store();
    let jobs = store.all();

    v_flex()
        .id("cron-page")
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .child(Label::new("Cron Jobs"))
        .child(
            Label::new("Scheduled agent tasks that run automatically on a timer.")
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
        .child(div().h_4())
        .child(render_add_job_form(_settings_window, _window, cx))
        .child(div().h_6())
        .child(if jobs.is_empty() {
            render_empty_state(cx)
        } else {
            render_job_list(&jobs, _window, cx)
        })
        .into_any_element()
}

// ── Add Job Form ──

fn render_add_job_form(
    _settings_window: &SettingsWindow,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    div()
        .p_4()
        .rounded_md()
        .border_1()
        .border_color(cx.theme().colors().border)
        .child(
            v_flex()
                .gap_3()
                .child(Label::new("Add New Cron Job").size(LabelSize::Large))
                .child(
                    v_flex()
                        .gap_2()
                        .child(Label::new("Schedule").size(LabelSize::Small).color(Color::Muted))
                        .child(
                            h_flex()
                                .gap_2()
                                .child(
                                    // Simple schedule picker — common presets as buttons
                                    h_flex()
                                        .gap_1()
                                        .child(schedule_button("30m", "30 min", _window, cx))
                                        .child(schedule_button("1h", "1 hour", _window, cx))
                                        .child(schedule_button("2h", "2 hours", _window, cx))
                                        .child(schedule_button("@daily", "Daily", _window, cx))
                                        .child(schedule_button("@weekly", "Weekly", _window, cx)),
                                ),
                        )
                        .child(
                            Label::new("Or use a custom cron expression (e.g. \"0 9 * * 1-5\" for weekdays at 9am). Then ask the agent to schedule it.")
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        ),
                )
                .child(
                    Label::new("Tip: Tell the agent \"Run tests every 30 minutes\" or \"Check for vulnerabilities daily at 9am\". The agent will create the cron job for you.")
                        .size(LabelSize::XSmall)
                        .color(Color::Accent),
                ),
        )
}

fn schedule_button(
    schedule: &'static str,
    label: &'static str,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    let sched = schedule;
    let lbl = label;
    Button::new(format!("sched-{schedule}"), lbl)
        .style(ButtonStyle::OutlinedGhost)
        .on_click(cx.listener(move |_, _, w, _cx| {
            // Copy the schedule to clipboard or prompt the agent
            w.platform().write_to_clipboard(platform::ClipboardItem::new_string(
                format!("Schedule a cron job with schedule: {}", sched)
            ));
        }))
}

// ── Empty State ──

fn render_empty_state(cx: &mut Context<SettingsWindow>) -> impl IntoElement {
    div()
        .p_8()
        .w_full()
        .child(
            v_flex()
                .gap_2()
                .child(Icon::new(IconName::Clock).size(IconSize::XLarge).color(Color::Muted))
                .child(Label::new("No cron jobs scheduled").size(LabelSize::Large).color(Color::Muted))
                .child(
                    Label::new("Use the schedule buttons above or tell the agent to create one.")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
        .into_any_element()
}

// ── Job List ──

fn render_job_list(
    jobs: &[agent::scheduler::CronJob],
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    v_flex()
        .gap_3()
        .child(
            v_flex()
                .gap_2()
                .children(jobs.iter().map(|job| render_job_card(job, window, cx))),
        )
}

fn render_job_card(
    job: &agent::scheduler::CronJob,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    let id = job.id.clone();
    let paused = job.paused;
    let status_text = if paused { "Paused" } else { "Active" };
    let status_color = if paused { Color::Warning } else { Color::Success };
    let status_icon = if paused { IconName::CircleSlash } else { IconName::Check };

    let total_runs = job.run_count;
    let successes = job.success_count;
    let failures = job.failure_count;

    v_flex()
        .p_4()
        .rounded_md()
        .border_1()
        .border_color(cx.theme().colors().border)
        .child(
            v_flex()
                .gap_2()
                .child(
                    h_flex()
                        .justify_between()
                        .child(
                            h_flex()
                                .gap_2()
                                .child(Label::new(&job.id).size(LabelSize::Large))
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .child(Icon::new(status_icon).size(IconSize::XSmall).color(status_color))
                                        .child(Label::new(status_text).size(LabelSize::XSmall).color(status_color)),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                // Pause/Resume toggle
                                .child(
                                    Button::new(format!("pause-{id}"), if paused { "Resume" } else { "Pause" })
                                        .style(ButtonStyle::OutlinedGhost)
                                        .on_click(cx.listener({
                                            let id = id.clone();
                                            move |_, _, _, cx| {
                                                let store = agent::scheduler::global_store();
                                                store.update(&id, |job| job.paused = !job.paused);
                                                cx.notify();
                                            }
                                        })),
                                )
                                // Delete button
                                .child(
                                    Button::new(format!("del-{id}"), "Delete")
                                        .style(ButtonStyle::OutlinedGhost)
                                        .color(Color::Error)
                                        .on_click(cx.listener({
                                            let id = id.clone();
                                            move |_, _, _, cx| {
                                                let store = agent::scheduler::global_store();
                                                store.remove(&id);
                                                cx.notify();
                                            }
                                        })),
                                ),
                        ),
                )
                .child(Label::new(&job.prompt).size(LabelSize::Small).color(Color::Muted))
                .child(
                    h_flex()
                        .gap_4()
                        .child(
                            h_flex()
                                .gap_1()
                                .child(Label::new("Schedule:").size(LabelSize::XSmall).color(Color::Muted))
                                .child(Label::new(&job.schedule).size(LabelSize::XSmall)),
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .child(Label::new("Next:").size(LabelSize::XSmall).color(Color::Muted))
                                .child(
                                    Label::new(format_timestamp(job.next_run_at))
                                        .size(LabelSize::XSmall),
                                ),
                        ),
                )
                .child(
                    h_flex()
                        .gap_4()
                        .child(
                            h_flex()
                                .gap_1()
                                .child(Icon::new(IconName::Check).size(IconSize::XSmall).color(Color::Success))
                                .child(Label::new(format!("{successes} ok")).size(LabelSize::XSmall).color(Color::Success)),
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .child(Icon::new(IconName::X).size(IconSize::XSmall).color(Color::Error))
                                .child(Label::new(format!("{failures} fail")).size(LabelSize::XSmall).color(Color::Error)),
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .child(
                                    Label::new(format!("{total_runs} total runs")).size(LabelSize::XSmall).color(Color::Muted),
                                ),
                        ),
                ),
        )
}

fn format_timestamp(ts: u64) -> String {
    if ts == 0 {
        return "unknown".into();
    }
    // Simple relative format
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    if ts <= now {
        return "now".into();
    }
    let diff = ts - now;
    if diff < 60 {
        format!("{diff}s")
    } else if diff < 3600 {
        format!("{}m", diff / 60)
    } else if diff < 86400 {
        format!("{}h", diff / 3600)
    } else {
        format!("{}d", diff / 86400)
    }
}

pub(crate) fn cron_sub_page_link() -> SubPageLink {
    SubPageLink {
        title: "Cron Jobs".into(),
        r#type: Default::default(),
        json_path: Some("cron_jobs"),
        description: Some(
            "View and manage scheduled agent tasks that run automatically."
                .into(),
        ),
        search_aliases: &[
            "cron", "scheduler", "scheduled task", "automation",
            "background task", "recurring", "cron job", "timer",
        ],
        in_json: false,
        files: settings::SettingsFile::User,
        render: render_cron_page,
    }
}
