use client::zed_urls;
use gpui::{
    ClickEvent, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, MouseDownEvent, Render,
    linear_color_stop, linear_gradient,
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
        cx.open_url(&zed_urls::external_agents_docs(cx));
        cx.notify();

        acp_onboarding_event!("Documentation Link Clicked");
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
        let illustration_element = |label: bool, opacity: f32| {
            h_flex()
                .px_1()
                .py_0p5()
                .gap_1()
                .rounded_sm()
                .bg(cx.theme().colors().element_active.opacity(0.05))
                .border_1()
                .border_color(cx.theme().colors().border)
                .border_dashed()
                .child(
                    Icon::new(IconName::Stop)
                        .size(IconSize::Small)
                        .color(Color::Custom(cx.theme().colors().text_muted.opacity(0.15))),
                )
                .map(|this| {
                    if label {
                        this.child(
                            Label::new("Your Agent Here")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    } else {
                        this.child(
                            div().w_16().h_1().rounded_full().bg(cx
                                .theme()
                                .colors()
                                .element_active
                                .opacity(0.6)),
                        )
                    }
                })
                .opacity(opacity)
        };

        let illustration = h_flex()
            .relative()
            .h(rems_from_px(126.))
            .bg(cx.theme().colors().editor_background)
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .justify_center()
            .gap_8()
            .rounded_t_md()
            .overflow_hidden()
            .child(
                div().absolute().inset_0().w(px(515.)).h(px(126.)).child(
                    Vector::new(VectorName::AcpGrid, rems_from_px(515.), rems_from_px(126.))
                        .color(ui::Color::Custom(cx.theme().colors().text.opacity(0.02))),
                ),
            )
            .child(div().absolute().inset_0().size_full().bg(linear_gradient(
                0.,
                linear_color_stop(
                    cx.theme().colors().elevated_surface_background.opacity(0.1),
                    0.9,
                ),
                linear_color_stop(
                    cx.theme().colors().elevated_surface_background.opacity(0.),
                    0.,
                ),
            )))
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .size_full()
                    .bg(gpui::black().opacity(0.15)),
            )
            .child(
                Vector::new(
                    VectorName::AcpLogoSerif,
                    rems_from_px(257.),
                    rems_from_px(47.),
                )
                .color(ui::Color::Custom(cx.theme().colors().text.opacity(0.8))),
            )
            .child(
                v_flex()
                    .gap_1p5()
                    .child(illustration_element(false, 0.15))
                    .child(illustration_element(true, 0.3))
                    .child(
                        h_flex()
                            .pl_1()
                            .pr_2()
                            .py_0p5()
                            .gap_1()
                            .rounded_sm()
                            .bg(cx.theme().colors().element_active.opacity(0.2))
                            .border_1()
                            .border_color(cx.theme().colors().border)
                            .child(
                                Icon::new(IconName::AiGemini)
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(Label::new("New Gemini CLI Thread").size(LabelSize::Small)),
                    )
                    .child(illustration_element(true, 0.3))
                    .child(illustration_element(false, 0.15)),
            );

        let heading = v_flex()
            .w_full()
            .gap_1()
            .child(
                Label::new("Now Available")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(Headline::new("Bring Your Own Agent to Zed").size(HeadlineSize::Large));

        let copy = "Bring the agent of your choice to Zed via our new Agent Client Protocol (ACP), starting with Google's Gemini CLI integration.";

        let open_panel_button = Button::new("open-panel", "Start with Gemini CLI")
            .icon_size(IconSize::Indicator)
            .style(ButtonStyle::Tinted(TintColor::Accent))
            .full_width()
            .on_click(cx.listener(Self::open_panel));

        let docs_button = Button::new("add-other-agents", "Add Other Agents")
            .icon(IconName::ArrowUpRight)
            .icon_size(IconSize::Indicator)
            .icon_color(Color::Muted)
            .full_width()
            .on_click(cx.listener(Self::view_docs));

        let close_button = h_flex().absolute().top_2().right_2().child(
            IconButton::new("cancel", IconName::Close).on_click(cx.listener(
                |_, _: &ClickEvent, _window, cx| {
                    acp_onboarding_event!("Canceled", trigger = "X click");
                    cx.emit(DismissEvent);
                },
            )),
        );

        v_flex()
            .id("acp-onboarding")
            .key_context("AcpOnboardingModal")
            .relative()
            .w(rems(34.))
            .h_full()
            .elevation_3(cx)
            .track_focus(&self.focus_handle(cx))
            .overflow_hidden()
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(|_, _: &menu::Cancel, _window, cx| {
                acp_onboarding_event!("Canceled", trigger = "Action");
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, cx| {
                this.focus_handle.focus(window, cx);
            }))
            .child(illustration)
            .child(
                v_flex()
                    .p_4()
                    .gap_2()
                    .child(heading)
                    .child(Label::new(copy).color(Color::Muted))
                    .child(
                        v_flex()
                            .w_full()
                            .mt_2()
                            .gap_1()
                            .child(open_panel_button)
                            .child(docs_button),
                    ),
            )
            .child(close_button)
    }
}
