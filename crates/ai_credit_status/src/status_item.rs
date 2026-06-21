use std::sync::Arc;
use std::time::Duration;

use client::{Client, UserStore};
use gpui::{Action, App, Context, Empty, Entity, FocusHandle, Subscription, Task, Window, px};
use language_model::{Event as LanguageModelEvent, LanguageModelRegistry};
use project::DisableAiSettings;
use settings::Settings;
use ui::{Label, LabelSize, ProgressBar, Tooltip, prelude::*};
use workspace::{HideStatusItem, ItemHandle, StatusItemView};
use zed_actions::OpenSettingsAt;

use crate::fetch::{
    CreditSnapshot, active_provider_id, fetch_credit_snapshot, usage_color,
};
use crate::ai_credit_settings::AiCreditStatusSettings;

pub struct AiCreditStatusItem {
    snapshot: Option<CreditSnapshot>,
    error_message: Option<String>,
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    _subscriptions: Vec<Subscription>,
    refresh_task: Option<Task<()>>,
    pane_item_focus_handle: Option<FocusHandle>,
}

impl AiCreditStatusItem {
    pub fn new(user_store: Entity<UserStore>, client: Arc<Client>, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            snapshot: None,
            error_message: None,
            user_store,
            client,
            _subscriptions: Vec::new(),
            refresh_task: None,
            pane_item_focus_handle: None,
        };

        this._subscriptions.push(cx.observe_global::<settings::SettingsStore>(|_, cx| {
            cx.notify();
        }));
        this._subscriptions
            .push(cx.subscribe(&LanguageModelRegistry::global(cx), |_, _, event, cx| {
                if matches!(event, LanguageModelEvent::DefaultModelChanged) {
                    cx.notify();
                }
            }));
        this._subscriptions.push(cx.observe(&this.user_store, |_, _, cx| {
            cx.notify();
        }));

        this.schedule_refresh(cx);
        this
    }

    fn schedule_refresh(&mut self, cx: &mut Context<Self>) {
        let settings = AiCreditStatusSettings::get_global(cx);
        if !settings.enabled || DisableAiSettings::get_global(cx).disable_ai {
            self.snapshot = None;
            self.error_message = None;
            self.refresh_task = None;
            return;
        }

        if active_provider_id(cx).is_none() {
            self.snapshot = None;
            self.error_message = Some("No active AI provider".into());
            return;
        }

        let user_store = self.user_store.clone();
        let client = self.client.clone();

        self.refresh_task = Some(cx.spawn(async move |this, cx| {
            loop {
                let (enabled, monthly_budget_usd, refresh_seconds) = cx.update(|cx| {
                    let settings = AiCreditStatusSettings::get_global(cx);
                    (
                        settings.enabled && !DisableAiSettings::get_global(cx).disable_ai,
                        settings.monthly_budget_usd,
                        settings.refresh_seconds.max(15),
                    )
                });

                if !enabled {
                    if this
                        .update(cx, |this, cx| {
                            this.snapshot = None;
                            this.error_message = None;
                            cx.notify();
                        })
                        .is_err()
                    {
                        break;
                    }
                    break;
                }

                let provider_id = cx.update(|cx| active_provider_id(cx));
                if let Some(provider_id) = provider_id {
                    match fetch_credit_snapshot(
                        provider_id,
                        user_store.clone(),
                        client.clone(),
                        monthly_budget_usd,
                        cx,
                    )
                    .await
                    {
                        Ok(snapshot) => {
                            if this
                                .update(cx, |this, cx| {
                                    this.snapshot = Some(snapshot);
                                    this.error_message = None;
                                    cx.notify();
                                })
                                .is_err()
                            {
                                break;
                            }
                        }
                        Err(error) => {
                            if this
                                .update(cx, |this, cx| {
                                    this.snapshot = None;
                                    this.error_message = Some(error.to_string());
                                    cx.notify();
                                })
                                .is_err()
                            {
                                break;
                            }
                        }
                    }
                } else if this
                    .update(cx, |this, cx| {
                        this.snapshot = None;
                        this.error_message = Some("No active AI provider".into());
                        cx.notify();
                    })
                    .is_err()
                {
                    break;
                }

                cx.background_executor()
                    .timer(Duration::from_secs(refresh_seconds))
                    .await;
            }

        }));
    }
}

impl Render for AiCreditStatusItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = AiCreditStatusSettings::get_global(cx);
        if !settings.enabled || DisableAiSettings::get_global(cx).disable_ai {
            return Empty.into_any_element();
        }

        let Some(snapshot) = self.snapshot.clone() else {
            let message = self
                .error_message
                .clone()
                .unwrap_or_else(|| "Loading AI credit usage…".into());

            return div()
                .id("ai-credit-status-loading")
                .child(
                    Label::new(message)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element();
        };

        let percent = (snapshot.used_ratio * 100.0).round() as u32;
        let bar_color = usage_color(snapshot.used_ratio, cx);
        let tooltip = snapshot.tooltip.clone();
        let account_url = snapshot.account_url.clone();

        h_flex()
            .id("ai-credit-status")
            .gap_1()
            .items_center()
            .max_w(px(180.))
            .child(
                div().w(px(96.)).child(
                    ProgressBar::new(
                        "ai-credit-usage",
                        snapshot.used_ratio * 100.0,
                        100.0,
                        cx,
                    )
                    .fg_color(bar_color),
                ),
            )
            .child(
                Label::new(format!("{percent}%"))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                Label::new(snapshot.label)
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .truncate(),
            )
            .tooltip(Tooltip::text(tooltip))
            .on_click(cx.listener(move |_, _, window, cx| {
                if let Some(url) = account_url.clone() {
                    cx.open_url(&url);
                } else {
                    window.dispatch_action(
                        OpenSettingsAt {
                            path: "agent".into(),
                            target: None,
                        }
                        .boxed_clone(),
                        cx,
                    );
                }
            }))
            .into_any_element()
    }
}

impl StatusItemView for AiCreditStatusItem {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pane_item_focus_handle = active_pane_item.map(|item| item.item_focus_handle(cx));
    }

    fn hide_setting(&self, _cx: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|settings| {
            settings
                .ai_credit_status
                .get_or_insert_with(Default::default)
                .enabled = Some(false);
        }))
    }
}
