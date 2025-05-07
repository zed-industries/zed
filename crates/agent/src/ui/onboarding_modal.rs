use gpui::{
    ClickEvent, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, MouseDownEvent, Render,
    linear_color_stop, linear_gradient,
};
use ui::{TintColor, Vector, VectorName, prelude::*};
use workspace::{ModalView, Workspace};

use crate::assistant_panel::AssistantPanel;

macro_rules! agent_onboarding_event {
    ($name:expr) => {
        telemetry::event!($name, source = "Agent Onboarding");
    };
    ($name:expr, $($key:ident $(= $value:expr)?),+ $(,)?) => {
        telemetry::event!($name, source = "Agent Onboarding", $($key $(= $value)?),+);
    };
}

pub struct AgentOnboardingModal {
    focus_handle: FocusHandle,
    workspace: Entity<Workspace>,
}

impl AgentOnboardingModal {
    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let workspace_entity = cx.entity();
        workspace.toggle_modal(window, cx, |_window, cx| Self {
            workspace: workspace_entity,
            focus_handle: cx.focus_handle(),
        });
    }

    fn open_panel(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.workspace.update(cx, |workspace, cx| {
            workspace.focus_panel::<AssistantPanel>(window, cx);
        });

        cx.emit(DismissEvent);

        agent_onboarding_event!("Open Panel Clicked");
    }

    fn view_blog(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        cx.open_url("http://zed.dev/blog/fastest-ai-code-editor");
        cx.notify();

        agent_onboarding_event!("Blog Link Clicked");
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for AgentOnboardingModal {}

impl Focusable for AgentOnboardingModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for AgentOnboardingModal {}

impl Render for AgentOnboardingModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window_height = window.viewport_size().height;
        let max_height = window_height - px(200.);

        let base = v_flex()
            .id("agent-onboarding")
            .key_context("AgentOnboardingModal")
            .relative()
            .w(px(450.))
            .h_full()
            .max_h(max_height)
            .p_4()
            .gap_2()
            .elevation_3(cx)
            .track_focus(&self.focus_handle(cx))
            .overflow_hidden()
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(|_, _: &menu::Cancel, _window, cx| {
                agent_onboarding_event!("Canceled", trigger = "Action");
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, _cx| {
                this.focus_handle.focus(window);
            }))
            .child(
                div()
                    .absolute()
                    .top_0()
                    .right(px(-1.0))
                    .w(px(441.))
                    .h(px(167.))
                    .child(
                        Vector::new(VectorName::Grid, rems_from_px(441.), rems_from_px(167.))
                            .color(ui::Color::Custom(cx.theme().colors().text.alpha(0.1))),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .top(px(-8.0))
                    .right_0()
                    .w(px(400.))
                    .h(px(92.))
                    .child(
                        Vector::new(VectorName::AiGrid, rems_from_px(400.), rems_from_px(92.))
                            .color(ui::Color::Custom(cx.theme().colors().text.alpha(0.32))),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .right_0()
                    .w(px(660.))
                    .h(px(801.))
                    .overflow_hidden()
                    .bg(linear_gradient(
                        75.,
                        linear_color_stop(cx.theme().colors().panel_background.alpha(0.01), 1.0),
                        linear_color_stop(cx.theme().colors().panel_background, 0.45),
                    )),
            )
            .child(
                div()
                    .absolute()
                    .bottom_0()
                    .right_0()
                    .w(px(660.))
                    .h(px(301.))
                    .overflow_hidden()
                    .bg(linear_gradient(
                        0.,
                        linear_color_stop(cx.theme().colors().panel_background.alpha(0.01), 1.0),
                        linear_color_stop(cx.theme().colors().panel_background, 0.),
                    )),
            )
            .child(
                v_flex()
                    .w_full()
                    .gap_1()
                    .child(
                        Label::new("Introducing")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(Headline::new("Agentic Editing in Zed").size(HeadlineSize::Large)),
            )
            .child(h_flex().absolute().top_2().right_2().child(
                IconButton::new("cancel", IconName::X).on_click(cx.listener(
                    |_, _: &ClickEvent, _window, cx| {
                        agent_onboarding_event!("Cancelled", trigger = "X click");
                        cx.emit(DismissEvent);
                    },
                )),
            ));

        let open_panel_button = Button::new("open-panel", "Get Started with the Agent Panel")
            .icon_size(IconSize::Indicator)
            .style(ButtonStyle::Tinted(TintColor::Accent))
            .full_width()
            .on_click(cx.listener(Self::open_panel));

        let blog_post_button = Button::new("view-blog", "Check out the Blog Post")
            .icon(IconName::ArrowUpRight)
            .icon_size(IconSize::Indicator)
            .icon_color(Color::Muted)
            .full_width()
            .on_click(cx.listener(Self::view_blog));

        let copy = "Zed now natively supports agentic editing, enabling fluid collaboration between humans and AI.";

        base.child(Label::new(copy).color(Color::Muted)).child(
            v_flex()
                .w_full()
                .mt_2()
                .gap_2()
                .child(open_panel_button)
                .child(blog_post_button),
        )
    }
}
