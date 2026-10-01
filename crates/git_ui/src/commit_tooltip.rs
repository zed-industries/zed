use crate::commit_view::CommitView;
use anyhow::Result;
use askpass::AskPassDelegate;
use editor::hover_markdown_style;
use futures::Future;
use git::blame::BlameEntry;
use git::repository::CommitSummary;
use git::{GitRemote, commit::ParsedCommitMessage};
use git_ui_core::askpass_modal::AskPassModal;
use git_ui_core::notifications::show_error_toast;
use gpui::{
    AbsoluteLength, App, Asset, Element, Entity, MouseButton, ParentElement, Pixels, Render,
    ScrollHandle, StatefulInteractiveElement, Task, WeakEntity, Window, prelude::*,
};
use markdown::{Markdown, MarkdownElement};
use notifications::status_toast::StatusToast;
use project::git_store::{Repository, UnshallowState};
use settings::Settings;
use std::hash::Hash;
use theme_settings::ThemeSettings;
use time::{OffsetDateTime, UtcOffset};
use ui::{Avatar, Chip, CopyButton, Divider, Tooltip, prelude::*, tooltip_container};
use workspace::Workspace;

#[derive(Clone, Debug)]
pub struct CommitDetails {
    pub sha: SharedString,
    pub author_name: SharedString,
    pub author_email: SharedString,
    pub commit_time: OffsetDateTime,
    pub message: Option<ParsedCommitMessage>,
    pub tag_names: Vec<SharedString>,
    pub boundary: bool,
}

const MAX_COMMIT_TOOLTIP_TAG_CHIPS: usize = 2;

pub(crate) fn commit_tag_chips(tag_names: &[SharedString]) -> Option<impl IntoElement> {
    if tag_names.is_empty() {
        return None;
    }

    let (visible_tags, hidden_tags) =
        tag_names.split_at(tag_names.len().min(MAX_COMMIT_TOOLTIP_TAG_CHIPS));

    Some(
        h_flex().max_w(relative(0.6)).gap_1().child(
            h_flex()
                .gap_1()
                .min_w_0()
                .children(
                    visible_tags
                        .iter()
                        .map(|tag_name| Chip::new(tag_name.clone()).truncate()),
                )
                .when(!hidden_tags.is_empty(), |this| {
                    let hidden_tags = hidden_tags.to_vec();
                    this.child(Chip::new(format!("+{}", hidden_tags.len())).tooltip(
                        Tooltip::element(move |_window, cx| {
                            v_flex()
                                .gap_1()
                                .children(itertools::Itertools::intersperse_with(
                                    hidden_tags.iter().map(|tag_name| {
                                        Label::new(tag_name.clone())
                                            .size(LabelSize::Small)
                                            .buffer_font(cx)
                                            .into_any_element()
                                    }),
                                    || Divider::horizontal().into_any_element(),
                                ))
                                .into_any_element()
                        }),
                    ))
                })
                .child(Divider::vertical()),
        ),
    )
}

const COMMIT_AVATAR_BORDER_WIDTH: Pixels = px(1.);

pub struct CommitAvatar<'a> {
    sha: &'a SharedString,
    author_email: Option<SharedString>,
    remote: Option<&'a GitRemote>,
    size: Option<AbsoluteLength>,
}

impl<'a> CommitAvatar<'a> {
    pub fn new(
        sha: &'a SharedString,
        author_email: Option<SharedString>,
        remote: Option<&'a GitRemote>,
    ) -> Self {
        Self {
            sha,
            author_email,
            remote,
            size: None,
        }
    }

    pub fn from_commit_details(details: &'a CommitDetails) -> Self {
        Self {
            sha: &details.sha,
            author_email: Some(details.author_email.clone()),
            remote: details
                .message
                .as_ref()
                .and_then(|details| details.remote.as_ref()),
            size: None,
        }
    }

    pub fn size(mut self, size: impl Into<AbsoluteLength>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn rendered_size(size: impl Into<AbsoluteLength>, window: &Window) -> Pixels {
        size.into().to_pixels(window.rem_size()) + COMMIT_AVATAR_BORDER_WIDTH * 2.
    }

    pub fn render(&'a self, window: &mut Window, cx: &mut App) -> AnyElement {
        let border_color = cx.theme().colors().border_variant;

        match self.avatar(window, cx) {
            None => {
                let container_size = self.size.map(|size| Self::rendered_size(size, window));

                h_flex()
                    .when_some(container_size, |this, size| this.size(size))
                    .justify_center()
                    .rounded_full()
                    .border(COMMIT_AVATAR_BORDER_WIDTH)
                    .border_color(border_color)
                    .bg(cx.theme().colors().element_disabled)
                    .child(
                        Icon::new(IconName::Person)
                            .color(Color::Muted)
                            .size(IconSize::XSmall),
                    )
                    .into_any_element()
            }
            Some(avatar) => avatar
                .when_some(self.size, |this, size| this.size(size))
                .border_color(border_color)
                .into_any_element(),
        }
    }

    pub fn avatar(&'a self, window: &mut Window, cx: &mut App) -> Option<Avatar> {
        // Bail early if the email isn't available yet. Without it,
        // the GitHub provider skips the fast CDN path and falls back
        // to an unauthenticated per-commit API call that is slow and
        // rate-limited. Worse, a failed lookup gets permanently
        // cached under the key (sha, host) — so even when the email
        // arrives on a later render, the cached None shadows the
        // fast path forever.
        self.author_email.as_ref()?;

        let remote = self
            .remote
            .filter(|remote| remote.host_supports_avatars())?;
        let avatar_url =
            CommitAvatarAsset::new(remote.clone(), self.sha.clone(), self.author_email.clone());

        let url = window.use_asset::<CommitAvatarAsset>(&avatar_url, cx)??;
        Some(Avatar::new(url.to_string()))
    }
}

#[derive(Clone, Debug)]
struct CommitAvatarAsset {
    sha: SharedString,
    author_email: Option<SharedString>,
    remote: GitRemote,
}

impl Hash for CommitAvatarAsset {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sha.hash(state);
        self.remote.host.name().hash(state);
    }
}

impl CommitAvatarAsset {
    fn new(remote: GitRemote, sha: SharedString, author_email: Option<SharedString>) -> Self {
        Self {
            remote,
            sha,
            author_email,
        }
    }
}

impl Asset for CommitAvatarAsset {
    type Source = Self;
    type Output = Option<SharedString>;

    fn load(
        source: Self::Source,
        cx: &mut App,
    ) -> impl Future<Output = Self::Output> + Send + 'static {
        let client = cx.http_client();

        async move {
            source
                .remote
                .avatar_url(source.sha, source.author_email, client)
                .await
                .map(|url| SharedString::from(url.to_string()))
        }
    }
}

pub struct CommitTooltip {
    commit: CommitDetails,
    scroll_handle: ScrollHandle,
    markdown: Entity<Markdown>,
    repository: Entity<Repository>,
    workspace: WeakEntity<Workspace>,
}

impl CommitTooltip {
    pub fn blame_entry(
        blame: &BlameEntry,
        details: Option<ParsedCommitMessage>,
        tag_names: Vec<SharedString>,
        repository: Entity<Repository>,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let commit_time = blame
            .committer_time
            .and_then(|t| OffsetDateTime::from_unix_timestamp(t).ok())
            .unwrap_or(OffsetDateTime::now_utc());

        Self::new(
            CommitDetails {
                sha: blame.sha.to_string().into(),
                commit_time,
                author_name: blame
                    .author
                    .clone()
                    .unwrap_or("<no name>".to_string())
                    .into(),
                author_email: blame.author_mail.clone().unwrap_or("".to_string()).into(),
                message: details,
                tag_names,
                boundary: blame.boundary,
            },
            repository,
            workspace,
            cx,
        )
    }

    pub fn new(
        commit: CommitDetails,
        repository: Entity<Repository>,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let markdown = cx.new(|cx| {
            Markdown::new(
                commit
                    .message
                    .as_ref()
                    .map(|message| message.message.clone())
                    .unwrap_or_default(),
                None,
                None,
                cx,
            )
        });
        Self {
            commit,
            repository,
            workspace,
            scroll_handle: ScrollHandle::new(),
            markdown,
        }
    }
}

impl Render for CommitTooltip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let avatar = CommitAvatar::from_commit_details(&self.commit).render(window, cx);

        let author = self.commit.author_name.clone();

        let author_email = self.commit.author_email.clone();

        let short_commit_id = self
            .commit
            .sha
            .get(0..git::SHORT_SHA_LENGTH)
            .map(|sha| sha.to_string().into())
            .unwrap_or_else(|| self.commit.sha.clone());
        let full_sha = self.commit.sha.to_string();
        let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
        let absolute_timestamp = time_format::format_localized_timestamp(
            self.commit.commit_time,
            OffsetDateTime::now_utc(),
            local_offset,
            time_format::TimestampFormat::MediumAbsolute,
        );
        let markdown_style = {
            let style = hover_markdown_style(window, cx);
            style
        };

        let message = self
            .commit
            .message
            .as_ref()
            .map(|_| {
                MarkdownElement::new(self.markdown.clone(), markdown_style)
                    .scroll_handle(self.scroll_handle.clone())
                    .into_any()
            })
            .unwrap_or("<no commit message>".into_any());

        let pull_request = self
            .commit
            .message
            .as_ref()
            .and_then(|details| details.pull_request.clone());
        let tag_names = self.commit.tag_names.clone();

        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
        let message_max_height = window.line_height() * 12 + (ui_font_size / 0.4);
        let repo = self.repository.clone();
        let workspace = self.workspace.clone();
        let commit_summary = CommitSummary {
            sha: self.commit.sha.clone(),
            subject: self
                .commit
                .message
                .as_ref()
                .map_or(Default::default(), |message| {
                    message
                        .message
                        .split('\n')
                        .next()
                        .unwrap()
                        .trim_end()
                        .to_string()
                        .into()
                }),
            commit_timestamp: self.commit.commit_time.unix_timestamp(),
            author_name: self.commit.author_name.clone(),
            has_parent: false,
        };
        let boundary_notice = self
            .commit
            .boundary
            .then(|| {
                shallow_boundary_notice(self.repository.clone(), self.workspace.clone(), window, cx)
            })
            .flatten();

        tooltip_container(cx, move |this, cx| {
            this.occlude()
                .on_mouse_move(|_, _, cx| cx.stop_propagation())
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(
                    v_flex()
                        .w(gpui::rems(30.))
                        .child(
                            h_flex()
                                .pb_1()
                                .gap_2()
                                .overflow_x_hidden()
                                .flex_wrap()
                                .child(avatar)
                                .child(author)
                                .when(!author_email.is_empty(), |this| {
                                    this.child(
                                        div()
                                            .text_color(cx.theme().colors().text_muted)
                                            .child(author_email),
                                    )
                                })
                                .border_b_1()
                                .border_color(cx.theme().colors().border_variant),
                        )
                        .children(boundary_notice)
                        .child(
                            div()
                                .id("inline-blame-commit-message")
                                .track_scroll(&self.scroll_handle)
                                .py_1p5()
                                .max_h(message_max_height)
                                .overflow_y_scroll()
                                .child(message),
                        )
                        .child(
                            h_flex()
                                .text_color(cx.theme().colors().text_muted)
                                .w_full()
                                .justify_between()
                                .pt_1()
                                .gap_1()
                                .flex_wrap()
                                .border_t_1()
                                .border_color(cx.theme().colors().border_variant)
                                .child(absolute_timestamp)
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .min_w_0()
                                        .children(commit_tag_chips(&tag_names))
                                        .when_some(pull_request, |this, pr| {
                                            this.child(
                                                Button::new(
                                                    "pull-request-button",
                                                    format!("#{}", pr.number),
                                                )
                                                .color(Color::Muted)
                                                .start_icon(
                                                    Icon::new(IconName::PullRequest)
                                                        .size(IconSize::Small)
                                                        .color(Color::Muted),
                                                )
                                                .on_click(move |_, _, cx| {
                                                    cx.stop_propagation();
                                                    cx.open_url(pr.url.as_str())
                                                }),
                                            )
                                            .child(Divider::vertical())
                                        })
                                        .child(
                                            Button::new(
                                                "commit-sha-button",
                                                short_commit_id.clone(),
                                            )
                                            .color(Color::Muted)
                                            .start_icon(
                                                Icon::new(IconName::FileGit)
                                                    .size(IconSize::Small)
                                                    .color(Color::Muted),
                                            )
                                            .on_click(
                                                move |_, window, cx| {
                                                    CommitView::open(
                                                        commit_summary.sha.to_string(),
                                                        repo.downgrade(),
                                                        workspace.clone(),
                                                        None,
                                                        None,
                                                        window,
                                                        cx,
                                                    );
                                                    cx.stop_propagation();
                                                },
                                            ),
                                        )
                                        .child(Divider::vertical())
                                        .child(
                                            CopyButton::new("copy-commit-sha", full_sha)
                                                .tooltip_label("Copy SHA"),
                                        ),
                                ),
                        ),
                )
        })
    }
}

fn blame_entry_timestamp(blame_entry: &BlameEntry, format: time_format::TimestampFormat) -> String {
    match blame_entry.author_offset_date_time() {
        Ok(timestamp) => {
            let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
            time_format::format_localized_timestamp(
                timestamp,
                time::OffsetDateTime::now_utc(),
                local_offset,
                format,
            )
        }
        Err(_) => "Error parsing date".to_string(),
    }
}

pub fn blame_entry_relative_timestamp(blame_entry: &BlameEntry) -> String {
    blame_entry_timestamp(blame_entry, time_format::TimestampFormat::Relative)
}

pub(crate) fn shallow_boundary_notice(
    repository: Entity<Repository>,
    workspace: WeakEntity<Workspace>,
    window: &Window,
    cx: &App,
) -> Option<impl IntoElement + use<>> {
    let unshallow_state = repository.read(cx).unshallow_state();
    if unshallow_state == UnshallowState::Unshallowed {
        return None;
    }
    let in_flight = unshallow_state == UnshallowState::InProgress;
    let avatar_width = CommitAvatar::rendered_size(rems(1.), window);
    let can_fetch = workspace
        .read_with(cx, |workspace, cx| {
            !workspace.project().read(cx).is_via_collab()
        })
        .unwrap_or(false);
    Some(
        v_flex()
            .py_1()
            .gap_2()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                h_flex()
                    .gap_2()
                    .items_start()
                    .child(
                        h_flex().w(avatar_width).justify_center().child(
                            Icon::new(IconName::Warning)
                                .size(IconSize::Small)
                                .color(Color::Warning),
                        ),
                    )
                    .child(
                        div().flex_1().min_w_0().child(
                            Label::new(
                                "Shallow clone boundary: earlier history is missing, so these lines may come from an older commit.",
                            )
                            .size(LabelSize::Small)
                            .line_height_style(LineHeightStyle::UiLabel),
                        ),
                    ),
            )
            .when(can_fetch, |this| {
                this.child(
                    h_flex()
                        .gap_2()
                        .child(div().w(avatar_width))
                        .child(
                            Button::new(
                                "fetch-unshallow",
                                if in_flight {
                                    "Fetching…"
                                } else {
                                    "Fetch Missing History"
                                },
                            )
                            .style(ButtonStyle::Outlined)
                            .label_size(LabelSize::Small)
                            .disabled(in_flight)
                            .tooltip(Tooltip::text(
                                "Run `git fetch --unshallow` to download the full history",
                            ))
                            .on_click(move |_, window, cx| {
                                cx.stop_propagation();
                                fetch_unshallow(
                                    repository.clone(),
                                    workspace.clone(),
                                    window,
                                    cx,
                                )
                                .detach_and_log_err(cx);
                            }),
                        ),
                )
            }),
    )
}

pub(crate) fn fetch_unshallow(
    repository: Entity<Repository>,
    workspace: WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) -> Task<Result<()>> {
    if repository.read(cx).unshallow_state() != UnshallowState::Idle {
        return Task::ready(Ok(()));
    }
    let askpass = {
        let workspace = workspace.clone();
        let window_handle = window.window_handle();
        AskPassDelegate::new_with_cancellation(
            &mut cx.to_async(),
            move |prompt, tx, cancellation, cx| {
                window_handle
                    .update(cx, |_, window, cx| {
                        workspace
                            .update(cx, |workspace, cx| {
                                workspace.toggle_modal(window, cx, |window, cx| {
                                    AskPassModal::new(
                                        "git fetch --unshallow".into(),
                                        prompt.into(),
                                        tx,
                                        cancellation,
                                        window,
                                        cx,
                                    )
                                });
                            })
                            .ok();
                    })
                    .ok();
            },
        )
    };
    let fetch = repository.update(cx, |repository, cx| repository.fetch_unshallow(askpass, cx));
    window.refresh();
    window.spawn(cx, async move |cx| {
        let result = match fetch.await {
            Ok(result) => result,
            Err(canceled) => Err(anyhow::Error::from(canceled)),
        };
        cx.update(|window, cx| {
            window.refresh();
            let Some(workspace) = workspace.upgrade() else {
                return Ok(());
            };
            match result {
                Ok(_) => {
                    workspace.update(cx, |workspace, cx| {
                        let toast = StatusToast::new(
                            "Fetched the missing commit history",
                            cx,
                            |this, _| {
                                this.icon(
                                    Icon::new(IconName::GitBranch)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                )
                            },
                        );
                        workspace.toggle_status_toast(toast, cx);
                    });
                    Ok(())
                }
                Err(error) => {
                    show_error_toast(workspace, "fetch --unshallow", error, cx);
                    Err(anyhow::anyhow!("git fetch --unshallow failed"))
                }
            }
        })?
    })
}
