use crate::{IconDecoration, IconDecorationKind, Tooltip, prelude::*};
use gpui::{Animation, AnimationExt, SharedString, pulsating_between};
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AiSettingItemStatus {
    #[default]
    Stopped,
    Starting,
    Running,
    Error,
    AuthRequired,
    ClientSecretRequired,
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
            Self::ClientSecretRequired => "Client secret required.",
            Self::Authenticating => "Waiting for authorization…",
        }
    }

    fn indicator_color(&self) -> Option<Color> {
        match self {
            Self::Stopped => None,
            Self::Starting | Self::Authenticating => Some(Color::Muted),
            Self::Running => Some(Color::Success),
            Self::Error => Some(Color::Error),
            Self::AuthRequired | Self::ClientSecretRequired => Some(Color::Warning),
        }
    }

    fn is_animated(&self) -> bool {
        matches!(self, Self::Starting | Self::Authenticating)
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
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

        let source_id = format!("source-{}", id);
        let icon_id = format!("icon-{}", id);
        let status_tooltip = status.tooltip_text();
        let source_tooltip = source.tooltip_text(&label);

        let icon_element = icon.unwrap_or_else(|| {
            let letter = label.chars().next().unwrap_or('?').to_ascii_uppercase();

            h_flex()
                .size_5()
                .flex_none()
                .justify_center()
                .rounded_sm()
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .bg(cx.theme().colors().element_active.opacity(0.2))
                .child(
                    Label::new(SharedString::from(letter.to_string()))
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .buffer_font(cx),
                )
                .into_any_element()
        });

        let icon_child = if status.is_animated() {
            div()
                .child(icon_element)
                .with_animation(
                    format!("icon-pulse-{}", id),
                    Animation::new(Duration::from_secs(2))
                        .repeat()
                        .with_easing(pulsating_between(0.4, 0.8)),
                    |element, delta| element.opacity(delta),
                )
                .into_any_element()
        } else {
            icon_element.into_any_element()
        };

        let icon_container = div()
            .id(icon_id)
            .relative()
            .flex_none()
            .tooltip(Tooltip::text(status_tooltip))
            .child(icon_child)
            .when_some(status.indicator_color(), |this, color| {
                this.child(
                    IconDecoration::new(
                        IconDecorationKind::Dot,
                        cx.theme().colors().panel_background,
                        cx,
                    )
                    .size(px(12.))
                    .color(color.color(cx))
                    .position(gpui::Point {
                        x: px(-3.),
                        y: px(-3.),
                    }),
                )
            });

        v_flex()
            .id(id)
            .min_w_0()
            .child(
                h_flex()
                    .min_w_0()
                    .w_full()
                    .gap_1p5()
                    .justify_between()
                    .child(
                        h_flex()
                            .flex_1()
                            .min_w_0()
                            .gap_1p5()
                            .child(icon_container)
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
                "MCP server with letter avatar (running)",
                container()
                    .child(
                        AiSettingItem::new(
                            "ext-mcp",
                            "Postgres",
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
                "MCP server (stopped)",
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
                "MCP server (starting, animated)",
                container()
                    .child(AiSettingItem::new(
                        "starting-mcp",
                        "Context7",
                        AiSettingItemStatus::Starting,
                        AiSettingItemSource::Extension,
                    ))
                    .into_any_element(),
            ),
            single_example(
                "Agent with icon (running)",
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
                "Registry agent (starting, animated)",
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
                "Error with details",
                container()
                    .child(
                        AiSettingItem::new(
                            "error-mcp",
                            "Amplitude",
                            AiSettingItemStatus::Error,
                            AiSettingItemSource::Extension,
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
