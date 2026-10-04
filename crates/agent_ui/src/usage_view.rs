//! "Agent Usage": every agent account with its official quota, and token
//! usage over time estimated from the agents' local session logs.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use agent_accounts::quota::{QuotaStatus, QuotaWindow};
use agent_accounts::usage_history::{
    self, Amount, UsageEntry, UsageSummary, format_reset_in, format_tokens, format_usd,
};
use agent_accounts::{AccountProvider, AgentAccount};
use chrono::{Local, Utc};
use gpui::{
    Action, App, AppContext as _, Entity, EventEmitter, FocusHandle, Focusable, Hsla, Render, Task,
    Window, relative, rgb,
};
use project::{AgentId, Project};
use ui::{Divider, Tooltip, prelude::*};
use workspace::{Item, Workspace};

use crate::AddAgentAccount;
use crate::account_registry::{AccountRegistry, QuotaRegistry};

/// Opens the agent usage page.
#[derive(Clone, Default, PartialEq, serde::Deserialize, schemars::JsonSchema, Action)]
#[action(namespace = agent)]
pub struct OpenAgentUsage;

const HISTORY_DAYS: u32 = 90;
const HISTORY_TTL: Duration = Duration::from_secs(5 * 60);
const CHART_HEIGHT: f32 = 140.;

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &OpenAgentUsage, window, cx| {
            if let Some(existing) = workspace.item_of_type::<AgentUsageView>(cx) {
                workspace.activate_item(&existing, true, true, window, cx);
                return;
            }
            let project = workspace.project().clone();
            let view = cx.new(|cx| AgentUsageView::new(project, cx));
            workspace.add_item_to_active_pane(Box::new(view), None, true, window, cx);
        });
    })
    .detach();
}

#[derive(Clone, Copy, PartialEq)]
enum Metric {
    Cost,
    Tokens,
}

pub struct AgentUsageView {
    project: Entity<Project>,
    focus_handle: FocusHandle,
    hide_emails: bool,
    metric: Metric,
    days: u32,
    entries: Option<Vec<UsageEntry>>,
    scanned_at: Option<Instant>,
    scan: Option<Task<()>>,
    _observers: Vec<gpui::Subscription>,
    _poll: Task<()>,
}

impl AgentUsageView {
    fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let observers = vec![
            cx.observe_global::<AccountRegistry>(|_, cx| cx.notify()),
            cx.observe_global::<QuotaRegistry>(|_, cx| cx.notify()),
        ];
        let mut this = Self {
            project,
            focus_handle: cx.focus_handle(),
            hide_emails: false,
            metric: Metric::Cost,
            days: 30,
            entries: None,
            scanned_at: None,
            scan: None,
            _observers: observers,
            // Readings expire after five minutes; check for stale ones often.
            _poll: cx.spawn(async move |this, cx| {
                loop {
                    cx.background_executor()
                        .timer(Duration::from_secs(60))
                        .await;
                    let updated = this.update(cx, |this, cx| {
                        this.refresh_quota(false, cx);
                        this.scan_history(cx);
                    });
                    if updated.is_err() {
                        break;
                    }
                }
            }),
        };
        AccountRegistry::refresh_if_stale(cx);
        this.refresh_quota(false, cx);
        this.scan_history(cx);
        this
    }

    fn all_accounts(cx: &App) -> Vec<AgentAccount> {
        AccountProvider::ALL
            .iter()
            .flat_map(|provider| AccountRegistry::accounts_for_agent(provider.agent_id(), cx))
            .collect()
    }

    fn refresh_quota(&mut self, force: bool, cx: &mut Context<Self>) {
        let accounts = Self::all_accounts(cx);
        if force {
            QuotaRegistry::refresh(&accounts, cx);
        } else {
            QuotaRegistry::refresh_if_stale(&accounts, cx);
        }
    }

    fn scan_history(&mut self, cx: &mut Context<Self>) {
        if self.scan.is_some() || self.scanned_at.is_some_and(|at| at.elapsed() < HISTORY_TTL) {
            return;
        }
        let accounts = Self::all_accounts(cx);
        let http = cx.http_client();
        let homes: Vec<(AccountProvider, PathBuf)> = accounts
            .iter()
            .map(|account| (account.provider, account.home.clone()))
            .collect();
        let cursor_logins: Vec<(PathBuf, bool)> = accounts
            .iter()
            .filter(|account| account.provider == AccountProvider::Cursor)
            .map(|account| (account.home.clone(), account.is_default))
            .collect();
        let now = Utc::now();
        let cutoff = usage_history::range_start(HISTORY_DAYS, Local::now());
        let local = cx.background_spawn(async move {
            usage_history::collect_local_entries(&homes, cutoff, now)
        });
        let cursor = cx.background_spawn(async move {
            let mut entries = Vec::new();
            for (home, is_default) in cursor_logins {
                let Some(token) = usage_history::read_cursor_token(&home, is_default).await else {
                    continue;
                };
                match usage_history::fetch_cursor_entries(&token, http.clone(), cutoff, now).await {
                    Ok(found) => entries.extend(found),
                    Err(error) => log::info!("agent usage: Cursor history unavailable: {error:#}"),
                }
            }
            entries
        });
        self.scan = Some(cx.spawn(async move |this, cx| {
            let mut entries = local.await;
            entries.extend(cursor.await);
            this.update(cx, |this, cx| {
                this.entries = Some(entries);
                this.scanned_at = Some(Instant::now());
                this.scan = None;
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }

    fn refresh(&mut self, cx: &mut Context<Self>) {
        self.scanned_at = None;
        AccountRegistry::refresh_if_stale(cx);
        self.refresh_quota(true, cx);
        self.scan_history(cx);
    }

    /// The installed agent whose accounts a provider's section shows.
    fn agent_for(&self, provider: AccountProvider, cx: &App) -> AgentId {
        self.project
            .read(cx)
            .agent_server_store()
            .read(cx)
            .external_agents()
            .find(|agent_id| AccountProvider::for_agent(agent_id.as_ref()) == Some(provider))
            .cloned()
            .unwrap_or_else(|| AgentId::new(provider.agent_id()))
    }

    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .justify_between()
            .child(Headline::new("Agent Usage").size(HeadlineSize::Small))
            .child(
                h_flex()
                    .gap_3()
                    .child(
                        Label::new("Official quota · refreshes every 5 min")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        Button::new(
                            "toggle-emails",
                            if self.hide_emails {
                                "Show emails"
                            } else {
                                "Hide emails"
                            },
                        )
                        .label_size(LabelSize::Small)
                        .start_icon(
                            Icon::new(if self.hide_emails {
                                IconName::Eye
                            } else {
                                IconName::EyeOff
                            })
                            .size(IconSize::Small),
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.hide_emails = !this.hide_emails;
                            cx.notify();
                        })),
                    )
                    .child(
                        IconButton::new("refresh-usage", IconName::RotateCw)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Refresh quota and usage"))
                            .on_click(cx.listener(|this, _, _, cx| this.refresh(cx))),
                    ),
            )
    }

    fn render_provider(
        &self,
        provider: AccountProvider,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let accounts = AccountRegistry::accounts_for_agent(provider.agent_id(), cx);
        let always_shown = matches!(provider, AccountProvider::Claude | AccountProvider::Codex);
        if accounts.is_empty() && !always_shown {
            return None;
        }
        let agent_id = self.agent_for(provider, cx);
        let add_account = AddAgentAccount { agent: agent_id };
        let header = h_flex()
            .justify_between()
            .child(
                h_flex()
                    .gap_2()
                    .child(Icon::new(provider_icon(provider)).color(Color::Muted))
                    .child(Label::new(provider.display_name())),
            )
            .child(
                Button::new(
                    SharedString::from(format!("add-account-{}", provider.harness())),
                    "Add account",
                )
                .label_size(LabelSize::Small)
                .start_icon(Icon::new(IconName::Plus).size(IconSize::Small))
                .on_click(move |_, window, cx| {
                    window.dispatch_action(Box::new(add_account.clone()), cx)
                }),
            );
        let body = if accounts.is_empty() {
            Label::new(format!(
                "No {} logins on this computer — sign in and usage appears here.",
                provider.display_name()
            ))
            .size(LabelSize::Small)
            .color(Color::Muted)
            .into_any_element()
        } else {
            let several = accounts.len() > 1;
            let cards: Vec<AnyElement> = accounts
                .iter()
                .enumerate()
                .map(|(index, account)| self.render_account(index, account, several, cx))
                .collect();
            div()
                .grid()
                .grid_cols(2)
                .gap_3()
                .children(cards)
                .into_any_element()
        };
        Some(
            v_flex()
                .gap_2()
                .child(header)
                .child(body)
                .into_any_element(),
        )
    }

    fn render_account(
        &self,
        index: usize,
        account: &AgentAccount,
        several: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let provider = account.provider;
        let id_suffix = format!("{}-{index}", provider.harness());
        let is_preferred = AccountRegistry::is_preferred(account, cx);
        let quota = QuotaRegistry::quota(account, cx);
        let mut name = account.label();
        if self.hide_emails && name.contains('@') {
            name = "Email hidden".into();
        }

        let radio = several.then(|| {
            let account = account.clone();
            IconButton::new(
                SharedString::from(format!("default-{id_suffix}")),
                if is_preferred {
                    IconName::Check
                } else {
                    IconName::Circle
                },
            )
            .icon_size(IconSize::Small)
            .icon_color(if is_preferred {
                Color::Accent
            } else {
                Color::Muted
            })
            .tooltip(Tooltip::text(if is_preferred {
                "Default for new agents"
            } else {
                "Make default for new agents"
            }))
            .on_click(move |_, _, cx| {
                if !is_preferred {
                    AccountRegistry::set_preferred(&account, cx);
                }
            })
        });

        let plan = quota.as_ref().and_then(|quota| quota.plan.clone());
        let status_badge = quota.as_ref().and_then(|quota| match quota.status {
            QuotaStatus::ApiKey => Some("API"),
            QuotaStatus::TokenExpired => Some("Sign-in expired"),
            QuotaStatus::SignedOut => Some("Signed out"),
            QuotaStatus::Unavailable(_) if provider != AccountProvider::Cursor => {
                Some("Unavailable")
            }
            _ => None,
        });
        let header = h_flex()
            .justify_between()
            .gap_2()
            .child(
                h_flex()
                    .gap_1p5()
                    .flex_none()
                    .children(radio)
                    .child(Label::new(name))
                    .children(plan.map(|plan| badge(plan.to_uppercase().into(), Color::Muted, cx)))
                    .children(status_badge.map(|text| {
                        let color = if text == "API" {
                            Color::Muted
                        } else {
                            Color::Warning
                        };
                        badge(text.into(), color, cx)
                    })),
            )
            .child(
                div().min_w_0().flex_1().flex().justify_end().child(
                    Label::new(account.home_label.clone())
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .truncate(),
                ),
            );

        let body = match &quota {
            None => message("Reading usage…"),
            Some(quota) => match &quota.status {
                QuotaStatus::Ok if !quota.windows.is_empty() => v_flex()
                    .gap_1()
                    .children(quota.windows.iter().map(|window| quota_row(window, cx)))
                    .into_any_element(),
                QuotaStatus::Ok => message("No quota is reported for this login."),
                QuotaStatus::TokenStale => message(&format!(
                    "Refreshes when {} next runs.",
                    provider.display_name()
                )),
                QuotaStatus::ApiKey => message("Billed per token."),
                QuotaStatus::TokenExpired | QuotaStatus::SignedOut => message(&format!(
                    "Sign in by running `{}` in a terminal.",
                    login_command(account)
                )),
                QuotaStatus::Unavailable(detail) => message(detail),
            },
        };

        let footer = if is_preferred {
            h_flex()
                .gap_1()
                .child(
                    Icon::new(IconName::Check)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .child(
                    Label::new("Default for new agents")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element()
        } else {
            let account = account.clone();
            h_flex()
                .child(
                    Button::new(
                        SharedString::from(format!("make-default-{id_suffix}")),
                        "Make default",
                    )
                    .label_size(LabelSize::Small)
                    .style(ButtonStyle::Outlined)
                    .on_click(move |_, _, cx| AccountRegistry::set_preferred(&account, cx)),
                )
                .into_any_element()
        };

        v_flex()
            .p_3()
            .gap_2()
            .min_w_0()
            .overflow_hidden()
            .rounded_md()
            .border_1()
            .border_color(if is_preferred && several {
                cx.theme().colors().border_focused
            } else {
                cx.theme().colors().border_variant
            })
            .child(header)
            .child(body)
            .child(Divider::horizontal())
            .child(footer)
            .into_any_element()
    }

    fn render_history(&self, cx: &mut Context<Self>) -> AnyElement {
        let today = Local::now().date_naive();
        let summary = self
            .entries
            .as_ref()
            .map(|entries| usage_history::summarize(entries, self.days, today));
        let scanning = self.scan.is_some();
        let range = summary
            .as_ref()
            .and_then(|summary| summary.first_day)
            .map(|first| format!("{} – {}", first.format("%b %-d"), today.format("%b %-d")))
            .unwrap_or_default();
        let subtitle = if scanning && summary.is_none() {
            "Scanning transcript logs…".to_string()
        } else {
            format!("{range} · API-rate estimate from local session logs")
        };

        let toggles = h_flex()
            .gap_2()
            .child(
                h_flex()
                    .child(self.toggle(
                        "metric-cost",
                        "Cost",
                        self.metric == Metric::Cost,
                        cx,
                        |this| this.metric = Metric::Cost,
                    ))
                    .child(self.toggle(
                        "metric-tokens",
                        "Tokens",
                        self.metric == Metric::Tokens,
                        cx,
                        |this| this.metric = Metric::Tokens,
                    )),
            )
            .child({
                let mut ranges = h_flex();
                for days in [7, 30, 90] {
                    ranges = ranges.child(self.toggle(
                        SharedString::from(format!("days-{days}")),
                        format!("{days}d"),
                        self.days == days,
                        cx,
                        move |this| this.days = days,
                    ));
                }
                ranges
            });

        let header = h_flex()
            .flex_wrap()
            .gap_2()
            .justify_between()
            .child(
                h_flex()
                    .gap_3()
                    .child(Label::new("TOKEN USAGE").size(LabelSize::Small))
                    .child(
                        Label::new(subtitle)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(toggles);

        let Some(summary) = summary else {
            return v_flex().gap_3().child(header).into_any_element();
        };
        v_flex()
            .gap_4()
            .when(scanning, |this| this.opacity(0.7))
            .child(header)
            .child(
                h_flex()
                    .gap_6()
                    .items_start()
                    .child(self.render_totals_column(&summary, cx))
                    .child(self.render_chart(&summary, cx)),
            )
            .child(self.render_metric_tiles(&summary))
            .child(self.render_models(&summary))
            .into_any_element()
    }

    fn toggle(
        &self,
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        selected: bool,
        cx: &mut Context<Self>,
        select: impl Fn(&mut Self) + 'static,
    ) -> impl IntoElement {
        Button::new(id.into(), label)
            .label_size(LabelSize::Small)
            .toggle_state(selected)
            .style(if selected {
                ButtonStyle::Filled
            } else {
                ButtonStyle::Subtle
            })
            .on_click(cx.listener(move |this, _, _, cx| {
                select(this);
                cx.notify();
            }))
    }

    fn value(&self, amount: Amount) -> f64 {
        match self.metric {
            Metric::Cost => amount.usd,
            Metric::Tokens => amount.tokens as f64,
        }
    }

    fn format_value(&self, amount: Amount) -> String {
        match self.metric {
            Metric::Cost => format_usd(amount.usd),
            Metric::Tokens => format_tokens(amount.tokens),
        }
    }

    fn render_totals_column(&self, summary: &UsageSummary, cx: &App) -> impl IntoElement {
        let total = self.value(summary.total);
        let (headline, caption) = match self.metric {
            Metric::Cost => (
                format!("{}*", format_usd(summary.total.usd)),
                "* if billed at full API rate",
            ),
            Metric::Tokens => (
                format_tokens(summary.total.tokens),
                "input, cache and output tokens",
            ),
        };
        let mut providers: Vec<AccountProvider> = summary.by_provider.keys().copied().collect();
        if providers.is_empty() {
            providers = vec![AccountProvider::Claude, AccountProvider::Codex];
        }
        v_flex()
            .w(px(260.))
            .flex_none()
            .gap_1()
            .child(Headline::new(headline).size(HeadlineSize::Large))
            .child(
                Label::new(caption)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(div().h_2())
            .children(providers.into_iter().map(|provider| {
                let amount = summary
                    .by_provider
                    .get(&provider)
                    .copied()
                    .unwrap_or_default();
                let share = if total > 0. {
                    (self.value(amount) / total * 100.).round()
                } else {
                    0.
                };
                h_flex()
                    .gap_2()
                    .child(div().size_2().rounded_xs().bg(provider_color(provider)))
                    .child(
                        Icon::new(provider_icon(provider))
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(div().flex_1().child(Label::new(provider.display_name())))
                    .child(Label::new(self.format_value(amount)))
                    .child(
                        div()
                            .w(px(40.))
                            .flex()
                            .justify_end()
                            .child(Label::new(format!("{share}%")).color(Color::Muted)),
                    )
                    .text_color(cx.theme().colors().text)
            }))
    }

    fn render_chart(&self, summary: &UsageSummary, cx: &App) -> impl IntoElement {
        let max = summary
            .days
            .iter()
            .map(|day| self.value(day.total))
            .fold(0., f64::max);
        let empty = cx.theme().colors().border_variant;
        let columns = summary.days.iter().map(|day| {
            let total = self.value(day.total);
            let tooltip = if total > 0. {
                let mut lines = vec![day.date.format("%a %b %-d").to_string()];
                for (provider, amount) in &day.by_provider {
                    lines.push(format!(
                        "{}: {} · {}",
                        provider.display_name(),
                        format_usd(amount.usd),
                        format_tokens(amount.tokens)
                    ));
                }
                Some(SharedString::from(lines.join("\n")))
            } else {
                None
            };
            let segments: Vec<AnyElement> = day
                .by_provider
                .iter()
                .rev()
                .filter(|(_, amount)| self.value(**amount) > 0.)
                .map(|(provider, amount)| {
                    div()
                        .w_full()
                        .h(relative(
                            (self.value(*amount) / max.max(f64::EPSILON)) as f32,
                        ))
                        .bg(provider_color(*provider))
                        .into_any_element()
                })
                .collect();
            div()
                .id(SharedString::from(format!("usage-day-{}", day.date)))
                .flex_1()
                .h_full()
                .flex()
                .flex_col()
                .justify_end()
                .when(segments.is_empty(), |this| {
                    this.child(div().w_full().h(px(1.)).bg(empty))
                })
                .children(segments)
                .when_some(tooltip, |this, tooltip| {
                    this.tooltip(Tooltip::text(tooltip))
                })
        });
        let axis_label =
            |text: String| Label::new(text).size(LabelSize::XSmall).color(Color::Muted);
        let first = summary.days.first().map(|day| day.date);
        let middle = summary.days.get(summary.days.len() / 2).map(|day| day.date);
        let last = summary.days.last().map(|day| day.date);
        let max_label = match self.metric {
            Metric::Cost => format_usd(max),
            Metric::Tokens => format_tokens(max as u64),
        };
        v_flex()
            .flex_1()
            .min_w_0()
            .gap_1()
            .child(axis_label(max_label))
            .child(
                h_flex()
                    .h(px(CHART_HEIGHT))
                    .items_end()
                    .gap(px(2.))
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .children(columns),
            )
            .child(
                h_flex().justify_between().children(
                    [first, middle, last]
                        .into_iter()
                        .flatten()
                        .map(|date| axis_label(date.format("%b %-d").to_string())),
                ),
            )
    }

    fn render_metric_tiles(&self, summary: &UsageSummary) -> impl IntoElement {
        let input = summary.uncached_input + summary.cached_input + summary.cache_write;
        let cached_share = if input > 0 {
            (summary.cached_input as f64 / input as f64 * 100.).round()
        } else {
            0.
        };
        let savings_ratio = if summary.total.usd > 0. {
            summary.cache_savings_usd / summary.total.usd
        } else {
            0.
        };
        let tile = |title: &'static str, value: String| {
            v_flex()
                .flex_1()
                .gap_0p5()
                .child(Label::new(title).size(LabelSize::Small).color(Color::Muted))
                .child(Label::new(value))
        };
        h_flex()
            .gap_4()
            .child(tile(
                "Processed tokens",
                format_tokens(summary.total.tokens),
            ))
            .child(tile(
                "Cached input",
                format!("{} · {cached_share}%", format_tokens(summary.cached_input)),
            ))
            .child(tile(
                "Uncached input",
                format_tokens(summary.uncached_input),
            ))
            .child(tile("Output", format_tokens(summary.output)))
            .child(tile(
                "Cache savings",
                format!(
                    "{} · {savings_ratio:.1}x",
                    format_usd(summary.cache_savings_usd)
                ),
            ))
    }

    fn render_models(&self, summary: &UsageSummary) -> impl IntoElement {
        let total = self.value(summary.total);
        let row =
            |model: SharedString, cost: String, share: String, tokens: String, muted: bool| {
                let color = if muted { Color::Muted } else { Color::Default };
                h_flex()
                    .gap_2()
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(Label::new(model).color(color).truncate()),
                    )
                    .child(div().w(px(90.)).child(Label::new(cost).color(color)))
                    .child(div().w(px(60.)).child(Label::new(share).color(color)))
                    .child(div().w(px(90.)).child(Label::new(tokens).color(color)))
            };
        v_flex()
            .gap_1()
            .child(row(
                "Model".into(),
                "Cost".into(),
                "Share".into(),
                "Tokens".into(),
                true,
            ))
            .children(summary.models.iter().take(6).map(|model| {
                let share = if total > 0. {
                    format!("{:.0}%", self.value(model.amount) / total * 100.)
                } else {
                    "0%".into()
                };
                let cost = format!(
                    "{}{}",
                    if model.approximate { "~" } else { "" },
                    format_usd(model.amount.usd)
                );
                row(
                    format!("{} · {}", model.model, model.provider.display_name()).into(),
                    cost,
                    share,
                    format_tokens(model.amount.tokens),
                    false,
                )
            }))
            .child(row(
                "Total".into(),
                format!(
                    "{}{}",
                    if summary.approximate { "~" } else { "" },
                    format_usd(summary.total.usd)
                ),
                "100%".into(),
                format_tokens(summary.total.tokens),
                false,
            ))
    }
}

fn message(text: &str) -> AnyElement {
    Label::new(text.to_string())
        .size(LabelSize::Small)
        .color(Color::Muted)
        .into_any_element()
}

fn badge(text: SharedString, color: Color, cx: &App) -> impl IntoElement {
    div()
        .px_1()
        .rounded_xs()
        .bg(cx.theme().colors().element_background)
        .child(Label::new(text).size(LabelSize::XSmall).color(color))
}

fn quota_row(window: &QuotaWindow, cx: &App) -> impl IntoElement {
    let used = window.used_percent.min(100);
    let fill: Hsla = if used >= 90 {
        cx.theme().status().error
    } else if used >= 70 {
        cx.theme().status().warning
    } else {
        cx.theme().colors().text_accent
    };
    let now = Utc::now();
    let reset = window.resets_at.map(|resets_at| {
        let local = resets_at.with_timezone(&Local);
        let when = if resets_at - now >= chrono::Duration::hours(24) {
            local.format("%b %-d").to_string()
        } else {
            local.format("%-I:%M %p").to_string()
        };
        (
            format!("↺ {}", format_reset_in(resets_at, now)),
            format!("Resets in {} · {when}", format_reset_in(resets_at, now)),
        )
    });
    h_flex()
        .id(SharedString::from(format!("quota-{}", window.id)))
        .gap_3()
        .child(
            div().w(px(88.)).flex_none().child(
                Label::new(window.label.clone())
                    .size(LabelSize::Small)
                    .truncate(),
            ),
        )
        .child(
            div()
                .flex_1()
                .min_w(px(24.))
                .h(px(3.))
                .rounded_full()
                .bg(cx.theme().colors().element_background)
                .child(
                    div()
                        .h_full()
                        .rounded_full()
                        .w(relative((used.max(1) as f32) / 100.))
                        .bg(fill),
                ),
        )
        .child(
            div()
                .w(px(36.))
                .flex_none()
                .flex()
                .justify_end()
                .child(Label::new(format!("{}%", window.used_percent)).size(LabelSize::Small)),
        )
        .child(
            div().w(px(58.)).flex_none().flex().justify_end().children(
                reset
                    .clone()
                    .map(|(text, _)| Label::new(text).size(LabelSize::Small).color(Color::Muted)),
            ),
        )
        .when_some(reset, |this, (_, tooltip)| {
            this.tooltip(Tooltip::text(tooltip))
        })
}

fn login_command(account: &AgentAccount) -> String {
    let home = account.home_label.clone();
    match (account.provider, account.is_default) {
        (AccountProvider::Claude, true) => "claude auth login".into(),
        (AccountProvider::Claude, false) => format!("CLAUDE_CONFIG_DIR={home} claude auth login"),
        (AccountProvider::Codex, true) => "codex login".into(),
        (AccountProvider::Codex, false) => format!("CODEX_HOME={home} codex login"),
        (AccountProvider::Grok, true) => "grok login".into(),
        (AccountProvider::Grok, false) => format!("GROK_HOME={home} grok login"),
        (AccountProvider::Cursor, true) => "cursor-agent login".into(),
        (AccountProvider::Cursor, false) => {
            format!("HOME={home} AGENT_CLI_CREDENTIAL_STORE=file cursor-agent login")
        }
    }
}

fn provider_icon(provider: AccountProvider) -> IconName {
    match provider {
        AccountProvider::Claude => IconName::AiClaude,
        AccountProvider::Codex => IconName::AiOpenAi,
        AccountProvider::Grok => IconName::AiXAi,
        AccountProvider::Cursor => IconName::Sparkle,
    }
}

fn provider_color(provider: AccountProvider) -> Hsla {
    let color = match provider {
        AccountProvider::Claude => 0xd06a48,
        AccountProvider::Codex => 0x1596d6,
        AccountProvider::Grok => 0x2f9e63,
        AccountProvider::Cursor => 0x8a63d2,
    };
    rgb(color).into()
}

impl EventEmitter<()> for AgentUsageView {}

impl Focusable for AgentUsageView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for AgentUsageView {
    type Event = ();

    fn to_item_events(_: &Self::Event, _: &mut dyn FnMut(workspace::item::ItemEvent)) {}

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Agent Usage".into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::ZedAgent))
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }
}

impl Render for AgentUsageView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sections: Vec<AnyElement> = AccountProvider::ALL
            .iter()
            .filter_map(|provider| self.render_provider(*provider, cx))
            .collect();
        div()
            .id("agent-usage")
            .track_focus(&self.focus_handle)
            .size_full()
            .overflow_y_scroll()
            .bg(cx.theme().colors().editor_background)
            .child(
                v_flex()
                    .max_w(px(1000.))
                    .mx_auto()
                    .p_6()
                    .gap_6()
                    .child(self.render_header(cx))
                    .children(sections)
                    .child(Divider::horizontal())
                    .child(self.render_history(cx)),
            )
    }
}
