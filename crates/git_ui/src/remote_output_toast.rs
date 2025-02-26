use std::{ops::Range, time::Duration};

use git::repository::{Remote, RemoteCommandOutput};
use gpui::{
    DismissEvent, EventEmitter, HighlightStyle, InteractiveText, StyledText, Task, UnderlineStyle,
    WeakEntity,
};
use itertools::Itertools;
use linkify::{LinkFinder, LinkKind};
use ui::{
    div, h_flex, px, rems_from_px, v_flex, vh, Color, Context, FluentBuilder, Icon,
    InteractiveElement, IntoElement, Label, LabelCommon, LabelSize, ParentElement, Render,
    SharedString, Styled, StyledExt, Window,
};
use workspace::{notifications::NotificationId, Workspace};

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
    workspace: WeakEntity<Workspace>,
    id: NotificationId,
    message: SharedString,
    remote_info: Option<InfoFromRemote>,
    dismiss_task: Task<()>,
}

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
                    .timer(Duration::from_millis(5000))
                    .await;
                workspace
                    .update(&mut cx, |workspace, cx| {
                        workspace.dismiss_notification(&id, cx);
                    })
                    .ok();
            }
        });

        let message;
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
                message = output.stdout.into();
                let remote_message = get_remote_lines(&output.stderr);
                let finder = LinkFinder::new();
                let links = finder
                    .links(&remote_message)
                    .filter(|link| *link.kind() != LinkKind::Url)
                    .map(|link| link.start()..link.end())
                    .collect_vec();

                remote = Some(InfoFromRemote {
                    name: remote_ref.name.into(),
                    remote_text: remote_message.into(),
                    links,
                });
            }
        }

        Self {
            workspace,
            id,
            message,
            remote_info: remote,
            dismiss_task: task,
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
                    .child(Label::new(self.message.clone()).size(LabelSize::Default))
                    .when_some(self.remote_info.as_ref(), |this, remote_info| {
                        this.child(
                            h_flex()
                                .gap_2()
                                .child(Icon::new(ui::IconName::Cloud).color(Color::Default))
                                .child(
                                    Label::new(format!("From {}:", remote_info.name))
                                        .size(LabelSize::Default),
                                ),
                        )
                        .map(|div| {
                            let mut text_style = window.text_style();
                            text_style.font_size = rems_from_px(12.0).into();
                            let styled_text = StyledText::new(remote_info.remote_text.clone())
                                .with_highlights(
                                    &window.text_style(),
                                    remote_info.links.iter().map(|link| {
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
                                    }),
                                );
                            let this = cx.weak_entity();
                            let text = InteractiveText::new("remote-message", styled_text)
                                .on_click(remote_info.links.clone(), move |ix, _window, cx| {
                                    this.update(cx, |this, cx| {
                                        if let Some(remote_info) = &this.remote_info {
                                            cx.open_url(
                                                &remote_info.remote_text
                                                    [remote_info.links[ix].clone()],
                                            )
                                        }
                                    })
                                    .ok();
                                });

                            div.child(text)
                        })
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
