use gpui::{
    ClickEvent, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, MouseDownEvent, Render,
    svg,
};
use ui::{TintColor, prelude::*};
use workspace::{ModalView, Workspace};

use crate::git_panel::GitPanel;

macro_rules! git_onboarding_event {
    ($name:expr) => {
        telemetry::event!($name, source = "Git Onboarding");
    };
    ($name:expr, $($key:ident $(= $value:expr)?),+ $(,)?) => {
        telemetry::event!($name, source = "Git Onboarding", $($key $(= $value)?),+);
    };
}

/// Introduces user to the Git Panel and overall improved Git support
pub struct GitOnboardingModal {
    focus_handle: FocusHandle,
    workspace: Entity<Workspace>,
}

impl GitOnboardingModal {
    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let workspace_entity = cx.entity();
        workspace.toggle_modal(window, cx, |_window, cx| Self {
            workspace: workspace_entity,
            focus_handle: cx.focus_handle(),
        });
    }

    fn open_panel(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.workspace.update(cx, |workspace, cx| {
            workspace.focus_panel::<GitPanel>(window, cx);
        });

        cx.emit(DismissEvent);

        git_onboarding_event!("Open Panel Clicked");
    }

    fn view_blog(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        cx.open_url("https://zed.dev/blog/git");
        cx.notify();

        git_onboarding_event!("Blog Link Clicked");
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for GitOnboardingModal {}

impl Focusable for GitOnboardingModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for GitOnboardingModal {}

impl Render for GitOnboardingModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window_height = window.viewport_size().height;
        let max_height = window_height - px(200.);

        let base = v_flex()
            .id("git-onboarding")
            .key_context("GitOnboardingModal")
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
                git_onboarding_event!("Cancelled", trigger = "Action");
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, cx| {
                this.focus_handle.focus(window, cx);
            }))
            .child(
                div().p_1p5().absolute().inset_0().h(px(160.)).child(
                    svg()
                        .path("icons/git_onboarding_bg.svg")
                        .text_color(cx.theme().colors().icon_disabled)
                        .w(px(420.))
                        .h(px(128.))
                        .overflow_hidden(),
                ),
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
                    .child(Headline::new("Native Git Support").size(HeadlineSize::Large)),
            )
            .child(h_flex().absolute().top_2().right_2().child(
                IconButton::new("cancel", IconName::Close).on_click(cx.listener(
                    |_, _: &ClickEvent, _window, cx| {
                        git_onboarding_event!("Cancelled", trigger = "X click");
                        cx.emit(DismissEvent);
                    },
                )),
            ));

        let open_panel_button = Button::new("open-panel", "Get Started with the Git Panel")
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

        let copy = "First-class support for staging, committing, pulling, pushing, viewing diffs, and more. All without leaving Zed.";

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
