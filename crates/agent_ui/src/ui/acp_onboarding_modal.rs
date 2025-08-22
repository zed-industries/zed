use gpui::{
    ClickEvent, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, MouseDownEvent, Render,
};
use ui::{TintColor, Vector, VectorName, prelude::*};
use workspace::{ModalView, Workspace};

use crate::agent_panel::{AgentPanel, AgentType};

macro_rules! acp_onboarding_event {
    ($name:expr) => {
        telemetry::event!($name, source = "ACP Onboarding");
    };
    ($name:expr, $($key:ident $(= $value:expr)?),+ $(,)?) => {
        telemetry::event!($name, source = "ACP Onboarding", $($key $(= $value)?),+);
    };
}

pub struct AcpOnboardingModal {
    focus_handle: FocusHandle,
    workspace: Entity<Workspace>,
}

impl AcpOnboardingModal {
    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let workspace_entity = cx.entity();
        workspace.toggle_modal(window, cx, |_window, cx| Self {
            workspace: workspace_entity,
            focus_handle: cx.focus_handle(),
        });
    }

    fn open_panel(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.workspace.update(cx, |workspace, cx| {
            workspace.focus_panel::<AgentPanel>(window, cx);

            if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    panel.new_agent_thread(AgentType::Gemini, window, cx);
                });
            }
        });

        cx.emit(DismissEvent);

        acp_onboarding_event!("Open Panel Clicked");
    }

    fn view_docs(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        cx.open_url("https://zed.dev/blog/"); // TODO: Add link
        cx.notify();

        acp_onboarding_event!("Docs Link Clicked");
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for AcpOnboardingModal {}

impl Focusable for AcpOnboardingModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for AcpOnboardingModal {}

impl Render for AcpOnboardingModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let base = v_flex()
            .id("acp-onboarding")
            .key_context("AcpOnboardingModal")
            .relative()
            .w(rems(34.))
            .h_full()
            .p_4()
            .gap_2()
            .elevation_3(cx)
            .track_focus(&self.focus_handle(cx))
            .overflow_hidden()
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(|_, _: &menu::Cancel, _window, cx| {
                acp_onboarding_event!("Canceled", trigger = "Action");
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, _cx| {
                this.focus_handle.focus(window);
            }))
            .child(
                div()
                    .absolute()
                    .top(px(-8.0))
                    .right_0()
                    .w(px(400.))
                    .h(px(92.))
                    .child(
                        Vector::new(VectorName::AcpGrid, rems_from_px(400.), rems_from_px(92.))
                            .color(ui::Color::Custom(cx.theme().colors().text.alpha(0.32))),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .size_full()
                    .bg(gpui::linear_gradient(
                        175.,
                        gpui::linear_color_stop(
                            cx.theme().colors().elevated_surface_background,
                            0.,
                        ),
                        gpui::linear_color_stop(
                            cx.theme().colors().elevated_surface_background.opacity(0.),
                            0.8,
                        ),
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
                    .child(Headline::new("Bring Your Own Agent to Zed").size(HeadlineSize::Large))
                    .child(Headline::new("featuring Gemini CLI").size(HeadlineSize::Large)),
            )
            .child(h_flex().absolute().top_2().right_2().child(
                IconButton::new("cancel", IconName::Close).on_click(cx.listener(
                    |_, _: &ClickEvent, _window, cx| {
                        acp_onboarding_event!("Cancelled", trigger = "X click");
                        cx.emit(DismissEvent);
                    },
                )),
            ));

        let open_panel_button = Button::new("open-panel", "Start with Gemini CLI")
            .icon_size(IconSize::Indicator)
            .style(ButtonStyle::Tinted(TintColor::Accent))
            .full_width()
            .on_click(cx.listener(Self::open_panel));

        let docs_button = Button::new("view-post", "Read Blog Post")
            .icon(IconName::ArrowUpRight)
            .icon_size(IconSize::Indicator)
            .icon_color(Color::Muted)
            .full_width()
            .on_click(cx.listener(Self::view_docs));

        let copy = "Zed now lets you bring the agent of your choice through the new Agent Client Protocol, starting with Google's Gemini CLI integration.";

        base.child(Label::new(copy).color(Color::Muted)).child(
            v_flex()
                .w_full()
                .mt_2()
                .gap_2()
                .child(open_panel_button)
                .child(docs_button),
        )
    }
}
