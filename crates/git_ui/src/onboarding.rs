use gpui::{
    svg, ClickEvent, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Global,
    MouseDownEvent, Render,
};
use ui::{prelude::*, ButtonLike, TintColor, Tooltip};
use util::ResultExt;
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
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, _cx| {
                this.focus_handle.focus(window);
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
                IconButton::new("cancel", IconName::X).on_click(cx.listener(
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

/// Prompts the user to try Zed's git features
pub struct GitBanner {
    dismissed: bool,
}

#[derive(Clone)]
struct GitBannerGlobal(Entity<GitBanner>);
impl Global for GitBannerGlobal {}

impl GitBanner {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.set_global(GitBannerGlobal(cx.entity()));
        Self {
            dismissed: get_dismissed(),
        }
    }

    fn should_show(&self, _cx: &mut App) -> bool {
        !self.dismissed
    }

    fn dismiss(&mut self, cx: &mut Context<Self>) {
        git_onboarding_event!("Banner Dismissed");
        persist_dismissed(cx);
        self.dismissed = true;
        cx.notify();
    }
}

const DISMISSED_AT_KEY: &str = "zed_git_banner_dismissed_at";

fn get_dismissed() -> bool {
    db::kvp::KEY_VALUE_STORE
        .read_kvp(DISMISSED_AT_KEY)
        .log_err()
        .map_or(false, |dismissed| dismissed.is_some())
}

fn persist_dismissed(cx: &mut App) {
    cx.spawn(async |_| {
        let time = chrono::Utc::now().to_rfc3339();
        db::kvp::KEY_VALUE_STORE
            .write_kvp(DISMISSED_AT_KEY.into(), time)
            .await
    })
    .detach_and_log_err(cx);
}

pub(crate) fn clear_dismissed(cx: &mut App) {
    cx.defer(|cx| {
        cx.global::<GitBannerGlobal>()
            .clone()
            .0
            .update(cx, |this, cx| {
                this.dismissed = false;
                cx.notify();
            });
    });

    cx.spawn(async |_| {
        db::kvp::KEY_VALUE_STORE
            .delete_kvp(DISMISSED_AT_KEY.into())
            .await
    })
    .detach_and_log_err(cx);
}

impl Render for GitBanner {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.should_show(cx) {
            return div();
        }

        let border_color = cx.theme().colors().editor_foreground.opacity(0.3);
        let banner = h_flex()
            .rounded_sm()
            .border_1()
            .border_color(border_color)
            .child(
                ButtonLike::new("try-git")
                    .child(
                        h_flex()
                            .h_full()
                            .items_center()
                            .gap_1()
                            .child(Icon::new(IconName::GitBranchSmall).size(IconSize::Small))
                            .child(
                                h_flex()
                                    .gap_0p5()
                                    .child(
                                        Label::new("Introducing:")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .child(Label::new("Git Support").size(LabelSize::Small)),
                            ),
                    )
                    .on_click(cx.listener(|this, _, window, cx| {
                        git_onboarding_event!("Banner Clicked");
                        this.dismiss(cx);
                        window.dispatch_action(
                            Box::new(zed_actions::OpenGitIntegrationOnboarding),
                            cx,
                        )
                    })),
            )
            .child(
                div().border_l_1().border_color(border_color).child(
                    IconButton::new("close", IconName::Close)
                        .icon_size(IconSize::Indicator)
                        .on_click(cx.listener(|this, _, _window, cx| this.dismiss(cx)))
                        .tooltip(|window, cx| {
                            Tooltip::with_meta(
                                "Close Announcement Banner",
                                None,
                                "It won't show again for this feature",
                                window,
                                cx,
                            )
                        }),
                ),
            );

        div().pr_2().child(banner)
    }
}
