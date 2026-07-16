//! Webhooks settings page — view and manage event-driven agent triggers.

use gpui::{ScrollHandle, prelude::*};
use ui::{prelude::*, *};

use crate::SettingsWindow;
use crate::page_data::SubPageLink;

pub(crate) fn render_webhooks_page(
    _settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let store = agent::webhook::global_store();
    let subs = store.all();

    v_flex()
        .id("webhooks-page")
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .child(Label::new("Webhooks"))
        .child(
            Label::new("Event-driven agent triggers that run automatically when files change, HTTP requests arrive, or git hooks fire.")
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
        .child(div().h_4())
        .child(render_event_types(cx))
        .child(div().h_6())
        .child(if subs.is_empty() {
            render_empty_state(cx)
        } else {
            render_sub_list(&subs, window, cx)
        })
        .into_any_element()
}

fn render_event_types(cx: &mut Context<SettingsWindow>) -> impl IntoElement {
    div()
        .p_4()
        .rounded_md()
        .border_1()
        .border_color(cx.theme().colors().border)
        .child(
            v_flex()
                .gap_3()
                .child(Label::new("Event Types").size(LabelSize::Large))
                .child(event_type_row("HTTP", "Listen for POST requests on a localhost port. Agent fires when a request arrives.", "8080", cx))
                .child(event_type_row("File Change", "Watch file changes matching a glob pattern. Agent fires when files are modified.", "**/*.rs", cx))
                .child(event_type_row("Git Hook", "Run on git events like pre-commit or post-merge.", "pre-commit", cx)),
        )
}

fn event_type_row(name: &str, desc: &str, example: &str, cx: &mut Context<SettingsWindow>) -> impl IntoElement {
    div()
        .child(
            h_flex()
                .gap_2()
                .child(Icon::new(IconName::Zap).size(IconSize::Small).color(Color::Accent))
                .child(
                    v_flex()
                        .child(Label::new(name).size(LabelSize::Small))
                        .child(Label::new(desc).size(LabelSize::XSmall).color(Color::Muted)),
                ),
        )
}

fn render_empty_state(cx: &mut Context<SettingsWindow>) -> impl IntoElement {
    div()
        .p_8()
        .w_full()
        .child(
            v_flex()
                .gap_2()
                .child(Icon::new(IconName::Zap).size(IconSize::XLarge).color(Color::Muted))
                .child(Label::new("No webhooks configured").size(LabelSize::Large).color(Color::Muted))
                .child(
                    Label::new("Tell the agent to subscribe to events. Try: 'Watch for changes to .rs files and run tests' or 'Listen for HTTP requests on port 8080'.")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
        .into_any_element()
}

fn render_sub_list(
    subs: &[agent::webhook::WebhookSubscription],
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    v_flex()
        .gap_3()
        .child(
            v_flex()
                .gap_2()
                .children(subs.iter().map(|sub| render_sub_card(sub, window, cx))),
        )
}

fn render_sub_card(
    sub: &agent::webhook::WebhookSubscription,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    let id = sub.id.clone();
    let active = sub.active;
    let et = match sub.event_type {
        agent::webhook::WebhookEventType::Http => "HTTP",
        agent::webhook::WebhookEventType::FileChange => "File Change",
        agent::webhook::WebhookEventType::GitHook => "Git Hook",
    };
    let status_text = if active { "Active" } else { "Paused" };
    let status_color = if active { Color::Success } else { Color::Warning };
    let status_icon = if active { IconName::Check } else { IconName::CircleSlash };

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
                                .child(Label::new(&sub.id).size(LabelSize::Large))
                                .child(Label::new(et).size(LabelSize::XSmall).color(Color::Muted))
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
                                .child(
                                    Button::new(format!("toggle-{id}"), if active { "Pause" } else { "Resume" })
                                        .style(ButtonStyle::OutlinedGhost)
                                        .on_click(cx.listener({
                                            let id = id.clone();
                                            move |_, _, _, cx| {
                                                let store = agent::webhook::global_store();
                                                store.toggle(&id);
                                                cx.notify();
                                            }
                                        })),
                                )
                                .child(
                                    Button::new(format!("del-{id}"), "Delete")
                                        .style(ButtonStyle::OutlinedGhost)
                                        .color(Color::Error)
                                        .on_click(cx.listener({
                                            let id = id.clone();
                                            move |_, _, _, cx| {
                                                let store = agent::webhook::global_store();
                                                store.remove(&id);
                                                cx.notify();
                                            }
                                        })),
                                ),
                        ),
                )
                .child(Label::new(&sub.prompt).size(LabelSize::Small).color(Color::Muted))
                .child(
                    h_flex()
                        .gap_4()
                        .child(
                            h_flex()
                                .gap_1()
                                .child(Label::new("Filter:").size(LabelSize::XSmall).color(Color::Muted))
                                .child(
                                    Label::new(if sub.filter.is_empty() { "(none)" } else { &sub.filter })
                                        .size(LabelSize::XSmall),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .child(Label::new("Fired:").size(LabelSize::XSmall).color(Color::Muted))
                                .child(Label::new(format!("{}", sub.fire_count)).size(LabelSize::XSmall)),
                        ),
                ),
        )
}

pub(crate) fn webhooks_sub_page_link() -> SubPageLink {
    SubPageLink {
        title: "Webhooks".into(),
        r#type: Default::default(),
        json_path: Some("webhooks"),
        description: Some(
            "Event-driven agent triggers that run automatically."
                .into(),
        ),
        search_aliases: &[
            "webhook", "trigger", "event", "file watch", "http listener",
            "git hook", "automation",
        ],
        in_json: false,
        files: settings::SettingsFile::User,
        render: render_webhooks_page,
    }
}
