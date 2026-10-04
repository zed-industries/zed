//! The account picker shown next to the mode and model selectors.

use gpui::{AnyElement, App, Entity};
use project::Project;
use ui::{
    Button, Callout, CalloutBorderPosition, ContextMenu, ContextMenuEntry, PopoverMenu, Severity,
    Tooltip, prelude::*,
};

use crate::account_registry::{
    AccountRegistry, AgentAccountsSettings, QuotaRegistry, account_label_with_quota,
};
use crate::thread_accounts;
use crate::thread_metadata_store::ThreadId;
use crate::{AddAgentAccount, Agent, ContinueThreadWith, NewExternalAgentThread};
use settings::Settings as _;

/// Renders the picker when the thread's agent has more than one account.
///
/// Picking another account starts an empty thread over with it, or, once the
/// conversation has messages, continues it with that account.
pub(crate) fn render_account_selector(
    agent: &Agent,
    has_messages: bool,
    is_local_project: bool,
    cx: &App,
) -> Option<AnyElement> {
    // Account homes are local paths; remote agents can't use them.
    if !is_local_project {
        return None;
    }
    let agent_id = agent.id();
    let accounts = AccountRegistry::accounts_for_agent(agent_id.as_ref(), cx);
    if accounts.is_empty() {
        return None;
    }
    let current = agent.account().cloned();
    let current_label = AccountRegistry::label(agent_id.as_ref(), current.as_ref(), cx);

    let trigger = Button::new("account-selector-trigger", current_label)
        .label_size(LabelSize::Small)
        .color(Color::Muted)
        .end_icon(
            Icon::new(IconName::ChevronDown)
                .size(IconSize::XSmall)
                .color(Color::Muted),
        );
    let tooltip = if has_messages {
        "Continue this conversation with another account"
    } else {
        "Choose the account for this thread"
    };

    Some(
        PopoverMenu::new("account-selector")
            .trigger_with_tooltip(trigger, Tooltip::text(tooltip))
            .anchor(gpui::Anchor::BottomRight)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            })
            .menu(move |window, cx| {
                QuotaRegistry::refresh_if_stale(&accounts, cx);
                let accounts = accounts.clone();
                let current = current.clone();
                let agent_id = agent_id.clone();
                Some(ContextMenu::build(window, cx, move |mut menu, _, cx| {
                    menu = menu.header(if has_messages {
                        "Continue with account"
                    } else {
                        "Account"
                    });
                    for account in &accounts {
                        let account_id = account.id();
                        let is_current = account_id == current;
                        let label = account_label_with_quota(account, cx);
                        let action: Box<dyn gpui::Action> = if has_messages {
                            Box::new(ContinueThreadWith {
                                agent: agent_id.clone(),
                                account: account_id.clone(),
                            })
                        } else {
                            Box::new(NewExternalAgentThread {
                                agent: agent_id.clone(),
                                account: account_id.clone(),
                            })
                        };
                        menu.push_item(
                            ContextMenuEntry::new(label)
                                .toggleable(IconPosition::End, is_current)
                                .disabled(is_current)
                                .handler(move |window, cx| {
                                    window.dispatch_action(action.boxed_clone(), cx)
                                }),
                        );
                    }
                    let add_account = AddAgentAccount { agent: agent_id };
                    menu.separator().item(
                        ContextMenuEntry::new("Add Account…")
                            .icon(IconName::Plus)
                            .icon_color(Color::Muted)
                            .handler(move |window, cx| {
                                window.dispatch_action(Box::new(add_account.clone()), cx)
                            }),
                    )
                }))
            })
            .into_any_element(),
    )
}

/// Tells the user that a thread continues a conversation started elsewhere.
pub(crate) fn render_handoff_notice(thread_id: ThreadId, cx: &App) -> Option<AnyElement> {
    let source = thread_accounts::read(thread_id, cx)?.handoff_from?;
    let source_agent = Agent::with_account(source.agent_id, source.account);
    let label = crate::agent_panel::handoff_target_label(&source_agent, cx);
    Some(
        Callout::new()
            .border_position(CalloutBorderPosition::Bottom)
            .severity(Severity::Info)
            .icon(IconName::ArrowRight)
            .title(format!("Continued from {label}"))
            .description(
                "The earlier conversation was converted into this agent's own session, \
                 so it has the full history. The original thread is unchanged.",
            )
            .into_any_element(),
    )
}

/// With auto-switch on, warns when the thread's account is almost out of
/// quota and offers the account with the most quota left.
pub(crate) fn render_quota_notice(
    agent: &Agent,
    has_messages: bool,
    is_local_project: bool,
    cx: &mut App,
) -> Option<AnyElement> {
    if !is_local_project || !AgentAccountsSettings::get_global(cx).auto_switch {
        return None;
    }
    let agent_id = agent.id();
    let accounts = AccountRegistry::accounts_for_agent(agent_id.as_ref(), cx);
    if accounts.len() < 2 {
        return None;
    }
    // Readings are cached for five minutes, so this rarely fetches.
    let to_refresh = accounts.clone();
    cx.defer(move |cx| QuotaRegistry::refresh_if_stale(&to_refresh, cx));

    let current = accounts
        .iter()
        .find(|account| account.id().as_ref() == agent.account())?;
    if !QuotaRegistry::is_exhausted(current, cx) {
        return None;
    }
    let alternative = QuotaRegistry::best_alternative(agent_id.as_ref(), agent.account(), cx)?;
    let used = QuotaRegistry::quota(current, cx)
        .and_then(|quota| quota.max_used_percent())
        .unwrap_or(100);
    let action: Box<dyn gpui::Action> = if has_messages {
        Box::new(ContinueThreadWith {
            agent: agent_id,
            account: alternative.id(),
        })
    } else {
        Box::new(NewExternalAgentThread {
            agent: agent_id,
            account: alternative.id(),
        })
    };
    let button_label = format!("Continue with {}", alternative.label());
    Some(
        Callout::new()
            .border_position(CalloutBorderPosition::Bottom)
            .severity(Severity::Warning)
            .icon(IconName::Warning)
            .title(format!("{} has used {used}% of its quota", current.label()))
            .description(format!(
                "{} has the most quota left. The conversation moves there with its history.",
                account_label_with_quota(&alternative, cx)
            ))
            .actions_slot(
                Button::new("quota-switch-account", button_label)
                    .label_size(LabelSize::Small)
                    .on_click(move |_, window, cx| {
                        window.dispatch_action(action.boxed_clone(), cx)
                    }),
            )
            .into_any_element(),
    )
}

/// After the agent reported the thread's account out of quota or credits,
/// offers to continue the conversation with another account or agent.
pub(crate) fn render_usage_limit_notice(
    agent: &Agent,
    project: &Entity<Project>,
    cx: &App,
) -> AnyElement {
    let agent_id = agent.id();
    let current_label = AccountRegistry::label(agent_id.as_ref(), agent.account(), cx);
    let alternative = project
        .read(cx)
        .is_local()
        .then(|| QuotaRegistry::any_alternative(agent_id.as_ref(), agent.account(), cx))
        .flatten();
    let targets = crate::agent_panel::handoff_targets(agent, project, cx);

    let mut actions = h_flex().gap_1();
    if let Some(alternative) = &alternative {
        let action = ContinueThreadWith {
            agent: agent_id,
            account: alternative.id(),
        };
        actions = actions.child(
            Button::new(
                "usage-limit-switch-account",
                format!("Continue with {}", alternative.label()),
            )
            .label_size(LabelSize::Small)
            .on_click(move |_, window, cx| window.dispatch_action(Box::new(action.clone()), cx)),
        );
    }
    if !targets.is_empty() {
        actions = actions.child(
            PopoverMenu::new("usage-limit-continue-with")
                .trigger(
                    Button::new("usage-limit-continue-with-trigger", "Continue with…")
                        .label_size(LabelSize::Small)
                        .end_icon(Icon::new(IconName::ChevronDown).size(IconSize::XSmall)),
                )
                .anchor(gpui::Anchor::BottomRight)
                .menu(move |window, cx| {
                    let targets = targets.clone();
                    Some(ContextMenu::build(window, cx, move |mut menu, _, _| {
                        for (target, label) in &targets {
                            let Agent::Custom { id, account } = target else {
                                continue;
                            };
                            menu = menu.action(
                                label.clone(),
                                Box::new(ContinueThreadWith {
                                    agent: id.clone(),
                                    account: account.clone(),
                                }),
                            );
                        }
                        menu
                    }))
                }),
        );
    }

    Callout::new()
        .border_position(CalloutBorderPosition::Bottom)
        .severity(Severity::Warning)
        .icon(IconName::Warning)
        .title(format!("{current_label} is out of quota"))
        .description(
            "Continue this conversation with another account or agent; it moves there \
             with its history and this thread stays as it is.",
        )
        .actions_slot(actions)
        .into_any_element()
}
