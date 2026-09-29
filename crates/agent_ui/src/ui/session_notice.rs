use agent_client_protocol::schema::v1 as acp;
use component::{Component, ComponentScope, example_group_with_title, single_example};
use gpui::{AnyElement, App, ClickEvent, ElementId, Stateful, Window, px};
use ui::{Callout, Color, IconButton, IconName, IconSize, Severity, Tooltip, prelude::*};

#[derive(IntoElement)]
pub struct SessionNotice {
    id: ElementId,
    callout: Callout,
}

impl SessionNotice {
    pub fn new(
        id: impl Into<ElementId>,
        notice: &acp::Notice,
        on_dismiss: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        let id = id.into();
        let dismiss_id = id.clone();
        let (severity, icon) = match &notice.severity {
            acp::NoticeSeverity::Info => (Severity::Info, Some(IconName::Info)),
            acp::NoticeSeverity::Warning => (Severity::Warning, Some(IconName::Warning)),
            acp::NoticeSeverity::Error => (Severity::Error, Some(IconName::XCircle)),
            _ => (Severity::Info, None),
        };

        Self {
            id,
            callout: Callout::new()
                .severity(severity)
                .title(notice.title.clone())
                .when_some(icon, Callout::icon)
                .when_some(notice.description.clone(), Callout::description)
                .scrollable_description(false)
                .dismiss_action(
                    div()
                        .debug_selector(move || format!("dismiss-{dismiss_id}"))
                        .child(
                            IconButton::new("dismiss", IconName::Close)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .aria_label(format!("Dismiss notice: {}", notice.title))
                                .tab_index(0_isize)
                                .tooltip(Tooltip::text("Dismiss Notice"))
                                .on_click(on_dismiss),
                        ),
                ),
        }
    }
}

impl RenderOnce for SessionNotice {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().id(self.id).w_full().flex_none().child(self.callout)
    }
}

pub fn session_notice_list(id: impl Into<ElementId>) -> Stateful<Div> {
    v_flex()
        .id(id)
        .w_full()
        .flex_shrink_0()
        .max_h(rems_from_px(192_f32))
        .overflow_y_scroll()
}

fn preview_notice(id: impl Into<ElementId>, notice: acp::Notice) -> AnyElement {
    SessionNotice::new(id, &notice, |_, _, _| {}).into_any_element()
}

#[derive(RegisterComponent)]
pub struct SessionNoticePreview;

impl Component for SessionNoticePreview {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn name() -> &'static str {
        "Session Notice"
    }

    fn description() -> &'static str {
        "Live ACP session notices as rendered above the agent composer. Dismiss buttons are illustrative in these previews."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
        let detailed = acp::Notice::new(acp::NoticeSeverity::Warning, "MCP server unavailable")
            .description("Continuing without it. Other integrations are still available.");
        let long = acp::Notice::new(
            acp::NoticeSeverity::Warning,
            "The optional documentation integration is unavailable in this remote environment",
        )
        .description(
            "Work continues using the files in your project.\nThe agent could not connect to the documentation service after several attempts.\n**This is plain text**, not Markdown.",
        );

        v_flex()
            .gap_6()
            .children([
                example_group_with_title(
                    "Severities",
                    vec![
                        single_example(
                            "Info",
                            preview_notice(
                                "preview-info-notice",
                                acp::Notice::new(acp::NoticeSeverity::Info, "Indexing resumed"),
                            ),
                        )
                        .width(px(640.)),
                        single_example(
                            "Warning with detail",
                            preview_notice("preview-warning-notice", detailed),
                        )
                        .width(px(640.)),
                        single_example(
                            "Error",
                            preview_notice(
                                "preview-error-notice",
                                acp::Notice::new(
                                    acp::NoticeSeverity::Error,
                                    "Optional integration failed",
                                )
                                .description("Work will continue without this integration."),
                            ),
                        )
                        .width(px(640.)),
                        single_example(
                            "Future severity",
                            preview_notice(
                                "preview-custom-notice",
                                acp::Notice::new(
                                    acp::NoticeSeverity::Other("maintenance".into()),
                                    "Scheduled maintenance",
                                ),
                            ),
                        )
                        .width(px(640.)),
                        single_example(
                            "Custom severity",
                            preview_notice(
                                "preview-extension-notice",
                                acp::Notice::new(
                                    acp::NoticeSeverity::Other("_agent_advisory".into()),
                                    "Using the workspace configuration",
                                )
                                .description("Agent-specific and future severities use the same generic presentation."),
                            ),
                        )
                        .width(px(640.)),
                    ],
                )
                .vertical()
                .into_any_element(),
                example_group_with_title(
                    "Layout",
                    vec![
                        single_example("Narrow", preview_notice("preview-narrow-notice", long))
                            .width(px(320.)),
                        single_example(
                            "Notice stack",
                            session_notice_list("preview-notice-stack")
                                .children([
                                    preview_notice(
                                        "preview-stack-info",
                                        acp::Notice::new(
                                            acp::NoticeSeverity::Info,
                                            "Connected to the remote environment",
                                        ),
                                    ),
                                    preview_notice(
                                        "preview-stack-warning",
                                        acp::Notice::new(
                                            acp::NoticeSeverity::Warning,
                                            "Network performance is degraded",
                                        )
                                        .description("Responses may take longer than usual."),
                                    ),
                                ])
                                .into_any_element(),
                        )
                        .width(px(640.)),
                        single_example(
                            "Overflowing notice stack",
                            session_notice_list("preview-overflow-notice-stack")
                                .children((0..6_usize).map(|index| {
                                    preview_notice(
                                        ("preview-repeated-notice", index),
                                        acp::Notice::new(
                                            acp::NoticeSeverity::Warning,
                                            "MCP server unavailable",
                                        )
                                        .description("Continuing without it. Repeated notices are shown as separate events."),
                                    )
                                }))
                                .into_any_element(),
                        )
                        .width(px(320.)),
                    ],
                )
                .vertical()
                .into_any_element(),
            ])
            .into_any_element()
    }
}
