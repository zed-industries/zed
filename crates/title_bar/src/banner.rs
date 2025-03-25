use gpui::{Action, Entity, Global, Render};
use ui::{prelude::*, ButtonLike, Tooltip};
use util::ResultExt;

/// Prompts the user to try Zed's features
pub struct Banner {
    dismissed: bool,
    source: String,
    details: BannerDetails,
}

#[derive(Clone)]
struct BannerGlobal {
    entity: Entity<Banner>,
}
impl Global for BannerGlobal {}

pub struct BannerDetails {
    pub action: Box<dyn Action>,
    pub banner_label: Box<dyn Fn(&Window, &mut Context<Banner>) -> AnyElement>,
}

impl Banner {
    pub fn new(source: &str, details: BannerDetails, cx: &mut Context<Self>) -> Self {
        cx.set_global(BannerGlobal {
            entity: cx.entity(),
        });
        Self {
            source: source.to_string(),
            details,
            dismissed: get_dismissed(source),
        }
    }

    fn should_show(&self, _cx: &mut App) -> bool {
        !self.dismissed
    }

    fn dismiss(&mut self, cx: &mut Context<Self>) {
        telemetry::event!("Banner Dismissed", source = self.source);
        persist_dismissed(&self.source, cx);
        self.dismissed = true;
        cx.notify();
    }
}

fn dismissed_at_key(source: &str) -> String {
    format!(
        "{}_{}",
        "_banner_dismissed_at",
        source.to_lowercase().trim().replace(" ", "_")
    )
}

fn get_dismissed(source: &str) -> bool {
    let dismissed_at = if source == "Git Onboarding" {
        "zed_git_banner_dismissed_at".to_string()
    } else {
        dismissed_at_key(source)
    };
    db::kvp::KEY_VALUE_STORE
        .read_kvp(&dismissed_at)
        .log_err()
        .map_or(false, |dismissed| dismissed.is_some())
}

fn persist_dismissed(source: &str, cx: &mut App) {
    let dismissed_at = dismissed_at_key(source);
    cx.spawn(async |_| {
        let time = chrono::Utc::now().to_rfc3339();
        db::kvp::KEY_VALUE_STORE.write_kvp(dismissed_at, time).await
    })
    .detach_and_log_err(cx);
}

pub fn restore_banner(cx: &mut App) {
    cx.defer(|cx| {
        cx.global::<BannerGlobal>()
            .entity
            .clone()
            .update(cx, |this, cx| {
                this.dismissed = false;
                cx.notify();
            });
    });

    let source = &cx.global::<BannerGlobal>().entity.read(cx).source;
    let dismissed_at = dismissed_at_key(source);
    cx.spawn(async |_| db::kvp::KEY_VALUE_STORE.delete_kvp(dismissed_at).await)
        .detach_and_log_err(cx);
}

impl Render for Banner {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.should_show(cx) {
            return div();
        }

        let border_color = cx.theme().colors().editor_foreground.opacity(0.3);
        let banner = h_flex()
            .rounded_sm()
            .border_1()
            .border_color(border_color)
            .child(
                ButtonLike::new("try-a-feature")
                    .child((self.details.banner_label)(window, cx))
                    .on_click(cx.listener(|this, _, window, cx| {
                        telemetry::event!("Banner Clicked", source = this.source);
                        this.dismiss(cx);
                        window.dispatch_action(this.details.action.boxed_clone(), cx)
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
