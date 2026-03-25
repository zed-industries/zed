use crate::{CommonAnimationExt, Indicator, Tooltip, prelude::*};
use gpui::SharedString;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AiSettingItemStatus {
    #[default]
    Stopped,
    Starting,
    Running,
    Error,
    AuthRequired,
    Authenticating,
}

impl AiSettingItemStatus {
    fn tooltip_text(&self) -> &'static str {
        match self {
            Self::Stopped => "Server is stopped.",
            Self::Starting => "Server is starting.",
            Self::Running => "Server is active.",
            Self::Error => "Server has an error.",
            Self::AuthRequired => "Authentication required.",
            Self::Authenticating => "Waiting for authorization…",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AiSettingItemSource {
    Extension,
    Custom,
    Registry,
}

impl AiSettingItemSource {
    fn icon_name(&self) -> IconName {
        match self {
            Self::Extension => IconName::ZedSrcExtension,
            Self::Custom => IconName::ZedSrcCustom,
            Self::Registry => IconName::AcpRegistry,
        }
    }

    fn tooltip_text(&self, label: &str) -> String {
        match self {
            Self::Extension => format!("{label} was installed from an extension."),
            Self::Registry => format!("{label} was installed from the ACP registry."),
            Self::Custom => format!("{label} was configured manually."),
        }
    }
}

/// A reusable setting item row for AI-related configuration lists.
#[derive(IntoElement, RegisterComponent)]
pub struct AiSettingItem {
    id: ElementId,
    status: AiSettingItemStatus,
    source: AiSettingItemSource,
    icon: Option<AnyElement>,
    label: SharedString,
    detail_label: Option<SharedString>,
    actions: Vec<AnyElement>,
    details: Option<AnyElement>,
}

impl AiSettingItem {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        status: AiSettingItemStatus,
        source: AiSettingItemSource,
    ) -> Self {
        Self {
            id: id.into(),
            status,
            source,
            icon: None,
            label: label.into(),
            detail_label: None,
            actions: Vec::new(),
            details: None,
        }
    }

    pub fn icon(mut self, element: impl IntoElement) -> Self {
        self.icon = Some(element.into_any_element());
        self
    }

    pub fn detail_label(mut self, detail: impl Into<SharedString>) -> Self {
        self.detail_label = Some(detail.into());
        self
    }

    pub fn action(mut self, element: impl IntoElement) -> Self {
        self.actions.push(element.into_any_element());
        self
    }

    pub fn details(mut self, element: impl IntoElement) -> Self {
        self.details = Some(element.into_any_element());
        self
    }
}

impl RenderOnce for AiSettingItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            status,
            source,
            icon,
            label,
            detail_label,
            actions,
            details,
        } = self;

        let status_id = format!("status-{}", id);
        let source_id = format!("source-{}", id);
        let status_tooltip = status.tooltip_text();
        let source_tooltip = source.tooltip_text(&label);

        let status_indicator = match status {
            AiSettingItemStatus::Stopped => Indicator::dot().color(Color::Muted).into_any_element(),
            AiSettingItemStatus::Running => {
                Indicator::dot().color(Color::Success).into_any_element()
            }
            AiSettingItemStatus::Error => Indicator::dot().color(Color::Error).into_any_element(),
            AiSettingItemStatus::AuthRequired => {
                Indicator::dot().color(Color::Warning).into_any_element()
            }
            AiSettingItemStatus::Starting | AiSettingItemStatus::Authenticating => {
                Icon::new(IconName::LoadCircle)
                    .size(IconSize::XSmall)
                    .color(Color::Muted)
                    .with_keyed_rotate_animation(format!("{}-starting", id), 3)
                    .into_any_element()
            }
        };

        v_flex()
            .id(id)
            .min_w_0()
            .child(
                h_flex()
                    .min_w_0()
                    .w_full()
                    .justify_between()
                    .child(
                        h_flex()
                            .flex_1()
                            .min_w_0()
                            .gap_1()
                            .child(
                                h_flex()
                                    .id(status_id)
                                    .h_full()
                                    .w_3()
                                    .flex_none()
                                    .justify_center()
                                    .child(status_indicator)
                                    .tooltip(Tooltip::text(status_tooltip)),
                            )
                            .children(icon)
                            .child(Label::new(label).flex_shrink_0().truncate())
                            .child(
                                div()
                                    .id(source_id)
                                    .min_w_0()
                                    .flex_none()
                                    .tooltip(Tooltip::text(source_tooltip))
                                    .child(
                                        Icon::new(source.icon_name())
                                            .size(IconSize::Small)
                                            .color(Color::Muted),
                                    ),
                            )
                            .when_some(detail_label, |this, detail| {
                                this.child(
                                    Label::new(detail)
                                        .color(Color::Muted)
                                        .size(LabelSize::Small),
                                )
                            }),
                    )
                    .when(!actions.is_empty(), |this| {
                        this.child(h_flex().gap_0p5().flex_none().children(actions))
                    }),
            )
            .children(details)
    }
}

impl Component for AiSettingItem {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let container = || {
            v_flex()
                .w_80()
                .p_2()
                .gap_2()
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .bg(cx.theme().colors().panel_background)
        };

        let details_row = |icon_name: IconName, icon_color: Color, message: &str| {
            h_flex()
                .py_1()
                .min_w_0()
                .w_full()
                .gap_2()
                .justify_between()
                .child(
                    h_flex()
                        .pr_4()
                        .min_w_0()
                        .w_full()
                        .gap_2()
                        .child(
                            Icon::new(icon_name)
                                .size(IconSize::XSmall)
                                .color(icon_color),
                        )
                        .child(
                            div().min_w_0().flex_1().child(
                                Label::new(SharedString::from(message.to_string()))
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            ),
                        ),
                )
        };

        let examples = vec![
            single_example(
                "Extension MCP server (running, with tools)",
                container()
                    .child(
                        AiSettingItem::new(
                            "ext-mcp",
                            "postgres-mcp",
                            AiSettingItemStatus::Running,
                            AiSettingItemSource::Extension,
                        )
                        .detail_label("3 tools")
                        .action(
                            IconButton::new("menu", IconName::Settings)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted),
                        )
                        .action(
                            IconButton::new("toggle", IconName::Check)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted),
                        ),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Custom MCP server (stopped)",
                container()
                    .child(AiSettingItem::new(
                        "custom-mcp",
                        "my-local-server",
                        AiSettingItemStatus::Stopped,
                        AiSettingItemSource::Custom,
                    ))
                    .into_any_element(),
            ),
            single_example(
                "Extension agent (running)",
                container()
                    .child(
                        AiSettingItem::new(
                            "ext-agent",
                            "Claude Agent",
                            AiSettingItemStatus::Running,
                            AiSettingItemSource::Extension,
                        )
                        .icon(
                            Icon::new(IconName::AiClaude)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
                        .action(
                            IconButton::new("restart", IconName::RotateCw)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted),
                        )
                        .action(
                            IconButton::new("delete", IconName::Trash)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted),
                        ),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Registry agent (starting)",
                container()
                    .child(
                        AiSettingItem::new(
                            "reg-agent",
                            "Devin Agent",
                            AiSettingItemStatus::Starting,
                            AiSettingItemSource::Registry,
                        )
                        .icon(
                            Icon::new(IconName::ZedAssistant)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .into_any_element(),
            ),
            single_example(
                "With details",
                container()
                    .child(
                        AiSettingItem::new(
                            "error-mcp",
                            "failing-server",
                            AiSettingItemStatus::Error,
                            AiSettingItemSource::Extension,
                        )
                        .action(
                            IconButton::new("toggle", IconName::Check)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted),
                        )
                        .details(
                            details_row(
                                IconName::XCircle,
                                Color::Error,
                                "Failed to connect: connection refused",
                            )
                            .child(
                                Button::new("logout", "Log Out")
                                    .style(ButtonStyle::Outlined)
                                    .label_size(LabelSize::Small),
                            ),
                        ),
                    )
                    .into_any_element(),
            ),
        ];

        Some(example_group(examples).vertical().into_any_element())
    }
}
