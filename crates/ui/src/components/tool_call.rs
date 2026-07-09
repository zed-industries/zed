use std::sync::Arc;

use gpui::{AnyElement, ClickEvent, Hsla};

use crate::prelude::*;
use crate::traits::animation_ext::CommonAnimationExt;
use crate::{Disclosure, Divider, DividerColor, GradientFade, Tooltip};

/// Shared header background used by all tool-call cards.
pub fn tool_call_card_header_bg(cx: &App) -> Hsla {
    cx.theme()
        .colors()
        .element_background
        .blend(cx.theme().colors().editor_foreground.opacity(0.025))
}

/// Shared border color used by all tool-call cards.
pub fn tool_call_card_border_color(cx: &App) -> Hsla {
    cx.theme().colors().border.opacity(0.8)
}

/// The lifecycle status of a tool call, distilled down to what the
/// presentation layer needs to know in order to style itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCallStatusKind {
    Pending,
    InProgress,
    Completed,
    Failed,
    Canceled,
    Rejected,
    AwaitingConfirmation,
}

impl ToolCallStatusKind {
    /// Whether the status should be rendered with the "unsuccessful" treatment
    /// (dashed border, error affordances).
    pub fn is_unsuccessful(self) -> bool {
        matches!(self, Self::Failed | Self::Canceled | Self::Rejected)
    }
}

/// The visual style of a tool call. This is intentionally separate from
/// *where* the tool call is mounted in a conversation (inline list, floating
/// permission row, embedded subagent) — callers own those outer margins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCallStyle {
    /// Read-only tools (read, list, search, fetch). The header sits flush; when
    /// expanded, the output is contained in its own bordered card.
    ReadOnly,
    /// Thinking blocks. The header sits flush; when expanded, the output hangs
    /// off a thin left guideline.
    Thinking,
    /// Edits, terminals, and confirmations. Everything lives inside one bordered
    /// card with a tinted header; the output flows directly below the header.
    Card,
}

type ClickHandler = Arc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// A standardized, presentational tool-call card.
///
/// This component is deliberately decoupled from the agent thread: it receives
/// already-rendered slots (`icon`, `label`, `content`, `header_actions`) plus
/// plain status/style data, so it can be previewed in isolation and reused
/// across read-only, edit, and confirmation tool calls.
///
/// # Usage Example
///
/// ```
/// use ui::prelude::*;
/// use ui::{Icon, IconName, Label, ToolCall, ToolCallStatusKind, ToolCallStyle};
///
/// let tool_call = ToolCall::new("read-file")
///     .icon(Icon::new(IconName::ToolSearch).color(Color::Muted))
///     .label(Label::new("Read src/main.rs").color(Color::Muted))
///     .status(ToolCallStatusKind::Completed)
///     .style(ToolCallStyle::ReadOnly)
///     .collapsible(true)
///     .open(false);
/// ```
#[derive(IntoElement, RegisterComponent)]
pub struct ToolCall {
    id: ElementId,
    icon: Option<AnyElement>,
    label: Option<AnyElement>,
    status: ToolCallStatusKind,
    style: ToolCallStyle,
    is_open: bool,
    is_collapsible: bool,
    fade_label: bool,
    wrap_content: bool,
    framed: Option<bool>,
    on_toggle: Option<ClickHandler>,
    header_actions: Vec<AnyElement>,
    content: Option<AnyElement>,
}

impl ToolCall {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            icon: None,
            label: None,
            status: ToolCallStatusKind::Completed,
            style: ToolCallStyle::ReadOnly,
            is_open: false,
            is_collapsible: false,
            fade_label: true,
            wrap_content: true,
            framed: None,
            on_toggle: None,
            header_actions: Vec::new(),
            content: None,
        }
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    pub fn label(mut self, label: impl IntoElement) -> Self {
        self.label = Some(label.into_any_element());
        self
    }

    pub fn status(mut self, status: ToolCallStatusKind) -> Self {
        self.status = status;
        self
    }

    pub fn style(mut self, style: ToolCallStyle) -> Self {
        self.style = style;
        self
    }

    pub fn open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn collapsible(mut self, is_collapsible: bool) -> Self {
        self.is_collapsible = is_collapsible;
        self
    }

    /// Whether to fade the trailing edge of the label with a gradient overlay,
    /// so a long, single-line label doesn't collide with the header actions.
    /// Defaults to `true`; edits typically disable it.
    pub fn fade_label(mut self, fade_label: bool) -> Self {
        self.fade_label = fade_label;
        self
    }

    /// Whether the component should frame the content slot itself (card top
    /// border / inline left guideline). Set to `false` when the caller's
    /// content already carries that styling. Defaults to `true`.
    pub fn wrap_content(mut self, wrap_content: bool) -> Self {
        self.wrap_content = wrap_content;
        self
    }

    /// Whether to draw the outer card frame (border, background, clipped
    /// corners). Defaults to following the style (`Card` is framed, `Inline`
    /// is not). Override this for a card-styled header that is mounted inside
    /// another container (e.g. a floating permission row) and shouldn't draw
    /// its own box.
    pub fn framed(mut self, framed: bool) -> Self {
        self.framed = Some(framed);
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Arc::new(handler));
        self
    }

    pub fn header_action(mut self, action: impl IntoElement) -> Self {
        self.header_actions.push(action.into_any_element());
        self
    }

    pub fn header_actions(mut self, actions: Vec<AnyElement>) -> Self {
        self.header_actions = actions;
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn when_some_content(self, content: Option<AnyElement>) -> Self {
        match content {
            Some(content) => self.content(content),
            None => self,
        }
    }
}

impl RenderOnce for ToolCall {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_card = self.style == ToolCallStyle::Card;
        let is_framed = self.framed.unwrap_or(is_card);
        let is_unsuccessful = self.status.is_unsuccessful();
        let header_bg = tool_call_card_header_bg(cx);
        let border_color = tool_call_card_border_color(cx);

        let header_group = SharedString::from(format!("{:?}-header", self.id));
        let has_actions = self.is_collapsible || !self.header_actions.is_empty();

        let disclosure = if self.is_collapsible {
            let on_toggle = self.on_toggle.clone();
            Some(
                Disclosure::new((self.id.clone(), "tool-call-disclosure"), self.is_open)
                    .opened_icon(IconName::ChevronUp)
                    .closed_icon(IconName::ChevronDown)
                    .visible_on_hover(header_group.clone())
                    .when_some(on_toggle, |this, on_toggle| {
                        this.on_toggle_expanded(Arc::clone(&on_toggle)
                            as Arc<dyn Fn(&ClickEvent, &mut Window, &mut App)>)
                    }),
            )
        } else {
            None
        };

        let header = h_flex()
            .group(header_group)
            .relative()
            .w_full()
            .justify_between()
            .when(is_card, |this| {
                this.p_0p5().rounded_t(rems_from_px(5.)).bg(header_bg)
            })
            .child(
                h_flex()
                    .relative()
                    .w_full()
                    .gap_1p5()
                    .when(is_card, |this| this.px_1())
                    .overflow_hidden()
                    .children(self.icon)
                    .children(self.label)
                    .when(self.fade_label, |this| {
                        let fade_bg = if is_card {
                            header_bg
                        } else {
                            cx.theme().colors().panel_background
                        };
                        this.child(GradientFade::new(fade_bg, fade_bg, fade_bg))
                    }),
            )
            .when(has_actions, |this| {
                this.child(
                    h_flex()
                        .flex_none()
                        .pr_0p5()
                        .gap_1()
                        .children(self.header_actions)
                        .children(disclosure),
                )
            });

        v_flex()
            .w_full()
            .when(!is_card, |this| this.gap_1())
            .when(is_framed, |this| {
                this.rounded_md()
                    .border_1()
                    .when(is_unsuccessful, |this| this.border_dashed())
                    .border_color(border_color)
                    .bg(cx.theme().colors().editor_background)
                    .overflow_hidden()
            })
            .child(header)
            .when_some(self.content, |this, content| {
                if !self.wrap_content {
                    this.child(content)
                } else if is_card {
                    this.child(
                        v_flex()
                            .w_full()
                            .border_t_1()
                            .when(is_unsuccessful, |this| this.border_dashed())
                            .border_color(border_color)
                            .child(content),
                    )
                } else if self.style == ToolCallStyle::ReadOnly {
                    // Read-only output is contained in its own bordered card.
                    this.child(
                        v_flex()
                            .w_full()
                            .rounded_md()
                            .border_1()
                            .border_color(border_color)
                            .bg(cx.theme().colors().editor_background)
                            .overflow_hidden()
                            .child(content),
                    )
                } else {
                    // Thinking output hangs off a thin left guideline so it
                    // reads as belonging to the header above it.
                    this.child(
                        v_flex()
                            .ml_1p5()
                            .pl_3p5()
                            .border_l_1()
                            .border_color(border_color)
                            .child(content),
                    )
                }
            })
    }
}

/// A presentational terminal tool-call card.
///
/// Like [`ToolCall`], this is decoupled from the agent thread — the command
/// block, terminal output, and any footer (permission buttons, sandbox
/// warnings) are passed in as already-rendered slots.
#[derive(IntoElement, RegisterComponent)]
pub struct ToolCallTerminal {
    id: ElementId,
    working_dir: SharedString,
    command: Option<AnyElement>,
    status: ToolCallStatusKind,
    is_open: bool,
    elapsed: Option<SharedString>,
    truncated: bool,
    truncation_tooltip: Option<SharedString>,
    exit_code: Option<i32>,
    command_failed: bool,
    stop_tooltip: Option<SharedString>,
    on_toggle: Option<ClickHandler>,
    on_stop: Option<ClickHandler>,
    notice: Option<AnyElement>,
    output: Option<AnyElement>,
    footer: Option<AnyElement>,
}

impl ToolCallTerminal {
    pub fn new(id: impl Into<ElementId>, working_dir: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            working_dir: working_dir.into(),
            command: None,
            status: ToolCallStatusKind::InProgress,
            is_open: false,
            elapsed: None,
            truncated: false,
            truncation_tooltip: None,
            exit_code: None,
            command_failed: false,
            stop_tooltip: None,
            on_toggle: None,
            on_stop: None,
            notice: None,
            output: None,
            footer: None,
        }
    }

    pub fn command(mut self, command: impl IntoElement) -> Self {
        self.command = Some(command.into_any_element());
        self
    }

    pub fn status(mut self, status: ToolCallStatusKind) -> Self {
        self.status = status;
        self
    }

    pub fn open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn elapsed(mut self, elapsed: impl Into<SharedString>) -> Self {
        self.elapsed = Some(elapsed.into());
        self
    }

    pub fn truncated(mut self, truncated: bool) -> Self {
        self.truncated = truncated;
        self
    }

    pub fn truncation_tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.truncation_tooltip = Some(tooltip.into());
        self
    }

    pub fn exit_code(mut self, exit_code: Option<i32>) -> Self {
        self.exit_code = exit_code;
        self
    }

    pub fn command_failed(mut self, command_failed: bool) -> Self {
        self.command_failed = command_failed;
        self
    }

    pub fn stop_tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.stop_tooltip = Some(tooltip.into());
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Arc::new(handler));
        self
    }

    pub fn on_stop(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_stop = Some(Arc::new(handler));
        self
    }

    /// A notice rendered between the command block and the output (e.g. a
    /// "command ran without the sandbox" warning).
    pub fn notice(mut self, notice: impl IntoElement) -> Self {
        self.notice = Some(notice.into_any_element());
        self
    }

    pub fn output(mut self, output: impl IntoElement) -> Self {
        self.output = Some(output.into_any_element());
        self
    }

    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
}

impl RenderOnce for ToolCallTerminal {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let header_bg = tool_call_card_header_bg(cx);
        let border_color = cx.theme().colors().border.opacity(0.6);
        let is_unsuccessful = self.status.is_unsuccessful() || self.command_failed;

        let is_running = matches!(
            self.status,
            ToolCallStatusKind::Pending | ToolCallStatusKind::InProgress
        );
        let needs_confirmation = self.status == ToolCallStatusKind::AwaitingConfirmation;

        let header_group = SharedString::from(format!("{:?}-terminal-header", self.id));

        let disclosure = {
            let on_toggle = self.on_toggle.clone();
            Disclosure::new((self.id.clone(), "terminal-disclosure"), self.is_open)
                .opened_icon(IconName::ChevronUp)
                .closed_icon(IconName::ChevronDown)
                .visible_on_hover(header_group.clone())
                .when_some(on_toggle, |this, on_toggle| {
                    this.on_toggle_expanded(
                        Arc::clone(&on_toggle) as Arc<dyn Fn(&ClickEvent, &mut Window, &mut App)>
                    )
                })
        };

        let header = h_flex()
            .id((self.id.clone(), "terminal-header"))
            .pt_1()
            .pl_1p5()
            .pr_1()
            .flex_none()
            .gap_1()
            .justify_between()
            .rounded_t_md()
            .child(
                div()
                    .id((self.id.clone(), "terminal-working-dir"))
                    .w_full()
                    .max_w_full()
                    .overflow_x_scroll()
                    .child(
                        Label::new(self.working_dir)
                            .buffer_font(cx)
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    ),
            )
            .child(disclosure)
            .when_some(self.elapsed, |header, elapsed| {
                header.child(
                    Label::new(format!("({elapsed})"))
                        .buffer_font(cx)
                        .color(Color::Muted)
                        .size(LabelSize::XSmall),
                )
            })
            .when(is_running && !needs_confirmation, |header| {
                let on_stop = self.on_stop.clone();
                let stop_tooltip = self.stop_tooltip.clone();
                header
                    .gap_1p5()
                    .child(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::XSmall)
                            .color(Color::Muted)
                            .with_rotate_animation(2),
                    )
                    .child(
                        div()
                            .h(relative(0.6))
                            .ml_1p5()
                            .child(Divider::vertical().color(DividerColor::Border)),
                    )
                    .child(
                        IconButton::new((self.id.clone(), "stop-terminal"), IconName::Stop)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Error)
                            .when_some(stop_tooltip, |this, tooltip| {
                                this.tooltip(Tooltip::text(tooltip))
                            })
                            .when_some(on_stop, |this, on_stop| {
                                this.on_click(move |event, window, cx| on_stop(event, window, cx))
                            }),
                    )
            })
            .when(self.truncated, |header| {
                let truncation_tooltip = self.truncation_tooltip.clone();
                header.child(
                    h_flex()
                        .id((self.id.clone(), "terminal-truncated"))
                        .gap_1()
                        .child(
                            Icon::new(IconName::Info)
                                .size(IconSize::XSmall)
                                .color(Color::Ignored),
                        )
                        .child(
                            Label::new("Truncated")
                                .color(Color::Muted)
                                .size(LabelSize::XSmall),
                        )
                        .when_some(truncation_tooltip, |this, tooltip| {
                            this.tooltip(Tooltip::text(tooltip))
                        }),
                )
            })
            .when(is_unsuccessful, |header| {
                let exit_code = self.exit_code;
                header.child(
                    div()
                        .id((self.id.clone(), "terminal-error"))
                        .child(
                            Icon::new(IconName::Close)
                                .size(IconSize::Small)
                                .color(Color::Error),
                        )
                        .when_some(exit_code, |this, code| {
                            this.tooltip(Tooltip::text(format!("Exited with code {code}")))
                        }),
                )
            });

        v_flex()
            .w_full()
            .overflow_hidden()
            .child(
                v_flex()
                    .group(header_group)
                    .bg(header_bg)
                    .text_xs()
                    .child(header)
                    .children(self.command),
            )
            .when_some(self.notice, |this, notice| this.child(notice))
            .when(self.is_open, |this| {
                this.when_some(self.output, |this, output| {
                    this.child(
                        div()
                            .pt_2()
                            .border_t_1()
                            .when(is_unsuccessful, |this| this.border_dashed())
                            .border_color(border_color)
                            .bg(cx.theme().colors().editor_background)
                            .rounded_b_md()
                            .child(output),
                    )
                })
            })
            .when_some(self.footer, |this, footer| this.child(footer))
    }
}

impl Component for ToolCall {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn description() -> &'static str {
        "A standardized presentation for a tool call in an agent conversation, \
        with a header (icon + label) and optional collapsible output. The style \
        controls how the output reads: read-only tools contain it in a card, \
        thinking blocks hang it off a left guideline, and edits/terminals wrap \
        the whole thing in a card."
    }

    fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
        let sample_label = |text: &'static str| Label::new(text).color(Color::Muted);
        let sample_icon =
            |icon: IconName| Icon::new(icon).size(IconSize::Small).color(Color::Muted);
        let sample_output = || {
            v_flex().p_2().gap_1().children(
                ["fn main() {", "    println!(\"hello\");", "}"]
                    .into_iter()
                    .map(|line| {
                        Label::new(line)
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .buffer_font(cx)
                    }),
            )
        };

        let read_only_examples = vec![
            single_example(
                "Collapsed",
                ToolCall::new("read-collapsed")
                    .icon(sample_icon(IconName::ToolSearch))
                    .label(sample_label("Read src/main.rs"))
                    .status(ToolCallStatusKind::Completed)
                    .style(ToolCallStyle::ReadOnly)
                    .collapsible(true)
                    .open(false)
                    .into_any_element(),
            )
            .width(px(560.)),
            single_example(
                "Expanded With Carded Output",
                ToolCall::new("read-expanded")
                    .icon(sample_icon(IconName::ToolSearch))
                    .label(sample_label("Read src/main.rs"))
                    .status(ToolCallStatusKind::Completed)
                    .style(ToolCallStyle::ReadOnly)
                    .collapsible(true)
                    .open(true)
                    .content(sample_output())
                    .into_any_element(),
            )
            .width(px(560.)),
            single_example(
                "Failed",
                ToolCall::new("read-failed")
                    .icon(sample_icon(IconName::ToolSearch))
                    .label(sample_label("Read missing.rs"))
                    .status(ToolCallStatusKind::Failed)
                    .style(ToolCallStyle::ReadOnly)
                    .header_action(
                        Icon::new(IconName::Close)
                            .color(Color::Error)
                            .size(IconSize::Small),
                    )
                    .into_any_element(),
            )
            .width(px(560.)),
        ];

        let thinking_examples = vec![
            single_example(
                "Expanded With Guideline Output",
                ToolCall::new("thinking-expanded")
                    .icon(sample_icon(IconName::ToolThink))
                    .label(sample_label("Thinking"))
                    .status(ToolCallStatusKind::Completed)
                    .style(ToolCallStyle::Thinking)
                    .collapsible(true)
                    .open(true)
                    .content(sample_output())
                    .into_any_element(),
            )
            .width(px(560.)),
        ];

        let card_examples = vec![
            single_example(
                "Edit Card",
                ToolCall::new("card-edit")
                    .icon(sample_icon(IconName::ToolPencil))
                    .label(sample_label("Edit src/main.rs"))
                    .status(ToolCallStatusKind::Completed)
                    .style(ToolCallStyle::Card)
                    .fade_label(false)
                    .collapsible(true)
                    .open(true)
                    .content(sample_output())
                    .into_any_element(),
            )
            .width(px(560.)),
            single_example(
                "Interrupted Edit",
                ToolCall::new("card-edit-canceled")
                    .icon(sample_icon(IconName::ToolPencil))
                    .label(sample_label("Edit src/main.rs"))
                    .status(ToolCallStatusKind::Canceled)
                    .style(ToolCallStyle::Card)
                    .fade_label(false)
                    .header_action(
                        IconButton::new("discard", IconName::Undo).icon_size(IconSize::Small),
                    )
                    .into_any_element(),
            )
            .width(px(560.)),
        ];

        v_flex()
            .gap_4()
            .child(example_group_with_title("Read-only", read_only_examples).vertical())
            .child(example_group_with_title("Thinking", thinking_examples).vertical())
            .child(example_group_with_title("Card", card_examples).vertical())
            .into_any_element()
    }
}

impl Component for ToolCallTerminal {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn description() -> &'static str {
        "A standardized card for displaying a terminal (execute) tool call, \
        with a working-directory header, the command, collapsible output, and \
        running/finished/failed states."
    }

    fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
        let command = |text: &'static str| {
            div().p_1p5().bg(tool_call_card_header_bg(cx)).child(
                Label::new(text)
                    .buffer_font(cx)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
        };
        let output = || {
            div().p_2().child(
                Label::new("hello from the terminal")
                    .buffer_font(cx)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
        };

        let examples = vec![
            single_example(
                "Running",
                ToolCallTerminal::new("terminal-running", "~/project")
                    .command(command("cargo build"))
                    .status(ToolCallStatusKind::InProgress)
                    .into_any_element(),
            )
            .width(px(560.)),
            single_example(
                "Finished",
                ToolCallTerminal::new("terminal-finished", "~/project")
                    .command(command("cargo build"))
                    .status(ToolCallStatusKind::Completed)
                    .elapsed("3s")
                    .open(true)
                    .output(output())
                    .into_any_element(),
            )
            .width(px(560.)),
            single_example(
                "Failed Exit Code",
                ToolCallTerminal::new("terminal-failed", "~/project")
                    .command(command("cargo test"))
                    .status(ToolCallStatusKind::Completed)
                    .command_failed(true)
                    .exit_code(Some(1))
                    .open(true)
                    .output(output())
                    .into_any_element(),
            )
            .width(px(560.)),
            single_example(
                "Truncated",
                ToolCallTerminal::new("terminal-truncated", "~/project")
                    .command(command("cat big_file.txt"))
                    .status(ToolCallStatusKind::Completed)
                    .truncated(true)
                    .into_any_element(),
            )
            .width(px(560.)),
        ];

        v_flex()
            .gap_4()
            .child(example_group_with_title("Terminal", examples).vertical())
            .into_any_element()
    }
}
