use std::{ops::Range, time::Duration};

use git::repository::{Remote, RemoteCommandOutput};
use gpui::{
    DismissEvent, EventEmitter, FocusHandle, Focusable, HighlightStyle, InteractiveText,
    StyledText, Task, UnderlineStyle, WeakEntity,
};
use itertools::Itertools;
use linkify::{LinkFinder, LinkKind};
use ui::{
    div, h_flex, px, v_flex, vh, Clickable, Color, Context, FluentBuilder, Icon, IconButton,
    IconName, InteractiveElement, IntoElement, Label, LabelCommon, LabelSize, ParentElement,
    Render, SharedString, Styled, StyledExt, Window,
};
use workspace::{
    notifications::{Notification, NotificationId},
    Workspace,
};

pub enum RemoteAction {
    Fetch,
    Pull,
    Push(Remote),
}

struct InfoFromRemote {
    name: SharedString,
    remote_text: SharedString,
    links: Vec<Range<usize>>,
}

pub struct RemoteOutputToast {
    _workspace: WeakEntity<Workspace>,
    _id: NotificationId,
    message: SharedString,
    remote_info: Option<InfoFromRemote>,
    _dismiss_task: Task<()>,
    focus_handle: FocusHandle,
}

impl Focusable for RemoteOutputToast {
    fn focus_handle(&self, _cx: &ui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Notification for RemoteOutputToast {}

const REMOTE_OUTPUT_TOAST_SECONDS: u64 = 5;

impl RemoteOutputToast {
    pub fn new(
        action: RemoteAction,
        output: RemoteCommandOutput,
        id: NotificationId,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let task = cx.spawn({
            let workspace = workspace.clone();
            let id = id.clone();
            |_, mut cx| async move {
                cx.background_executor()
                    .timer(Duration::from_secs(REMOTE_OUTPUT_TOAST_SECONDS))
                    .await;
                workspace
                    .update(&mut cx, |workspace, cx| {
                        workspace.dismiss_notification(&id, cx);
                    })
                    .ok();
            }
        });

        let mut message: SharedString;
        let remote;

        match action {
            RemoteAction::Fetch | RemoteAction::Pull => {
                if output.is_empty() {
                    message = "Up to date".into();
                } else {
                    message = output.stderr.into();
                }
                remote = None;
            }

            RemoteAction::Push(remote_ref) => {
                message = output.stdout.trim().to_string().into();
                if message.is_empty() {
                    message = output.stderr.trim().to_string().into();
                    if message.is_empty() {
                        message = "Push Successful".into();
                    }
                    remote = None;
                } else {
                    let remote_message = get_remote_lines(&output.stderr);

                    remote = if remote_message.is_empty() {
                        None
                    } else {
                        let finder = LinkFinder::new();
                        let links = finder
                            .links(&remote_message)
                            .filter(|link| *link.kind() == LinkKind::Url)
                            .map(|link| link.start()..link.end())
                            .collect_vec();

                        Some(InfoFromRemote {
                            name: remote_ref.name,
                            remote_text: remote_message.into(),
                            links,
                        })
                    }
                }
            }
        }

        Self {
            _workspace: workspace,
            _id: id,
            message,
            remote_info: remote,
            _dismiss_task: task,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for RemoteOutputToast {
    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        div()
            .occlude()
            .w_full()
            .max_h(vh(0.8, window))
            .elevation_3(cx)
            .child(
                v_flex()
                    .p_3()
                    .overflow_hidden()
                    .child(
                        h_flex()
                            .justify_between()
                            .items_start()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Icon::new(IconName::GitBranch).color(Color::Default))
                                    .child(Label::new("Git")),
                            )
                            .child(h_flex().child(
                                IconButton::new("close", IconName::Close).on_click(
                                    cx.listener(|_, _, _, cx| cx.emit(gpui::DismissEvent)),
                                ),
                            )),
                    )
                    .child(Label::new(self.message.clone()).size(LabelSize::Default))
                    .when_some(self.remote_info.as_ref(), |this, remote_info| {
                        this.child(
                            div()
                                .border_1()
                                .border_color(Color::Muted.color(cx))
                                .rounded_lg()
                                .text_sm()
                                .mt_1()
                                .p_1()
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(Icon::new(IconName::Cloud).color(Color::Default))
                                        .child(
                                            Label::new(remote_info.name.clone())
                                                .size(LabelSize::Default),
                                        ),
                                )
                                .map(|div| {
                                    let styled_text =
                                        StyledText::new(remote_info.remote_text.clone())
                                            .with_highlights(remote_info.links.iter().map(
                                                |link| {
                                                    (
                                                        link.clone(),
                                                        HighlightStyle {
                                                            underline: Some(UnderlineStyle {
                                                                thickness: px(1.0),
                                                                ..Default::default()
                                                            }),
                                                            ..Default::default()
                                                        },
                                                    )
                                                },
                                            ));
                                    let this = cx.weak_entity();
                                    let text = InteractiveText::new("remote-message", styled_text)
                                        .on_click(
                                            remote_info.links.clone(),
                                            move |ix, _window, cx| {
                                                this.update(cx, |this, cx| {
                                                    if let Some(remote_info) = &this.remote_info {
                                                        cx.open_url(
                                                            &remote_info.remote_text
                                                                [remote_info.links[ix].clone()],
                                                        )
                                                    }
                                                })
                                                .ok();
                                            },
                                        );

                                    div.child(text)
                                }),
                        )
                    }),
            )
    }
}

impl EventEmitter<DismissEvent> for RemoteOutputToast {}

fn get_remote_lines(output: &str) -> String {
    output
        .lines()
        .filter_map(|line| line.strip_prefix("remote:"))
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
