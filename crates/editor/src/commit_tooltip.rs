use futures::Future;
use git::blame::BlameEntry;
use git::PullRequest;
use gpui::{
    App, Asset, ClipboardItem, Element, Entity, MouseButton, ParentElement, Render, ScrollHandle,
    StatefulInteractiveElement,
};
use markdown::Markdown;
use settings::Settings;
use std::hash::Hash;
use theme::ThemeSettings;
use time::{OffsetDateTime, UtcOffset};
use time_format::format_local_timestamp;
use ui::{prelude::*, tooltip_container, Avatar, Divider, IconButtonShape};
use url::Url;

use crate::git::blame::GitRemote;
use crate::hover_popover::hover_markdown_style;

#[derive(Clone, Debug)]
pub struct CommitDetails {
    pub sha: SharedString,
    pub committer_name: SharedString,
    pub committer_email: SharedString,
    pub commit_time: OffsetDateTime,
    pub message: Option<ParsedCommitMessage>,
}

#[derive(Clone, Debug, Default)]
pub struct ParsedCommitMessage {
    pub message: SharedString,
    pub permalink: Option<Url>,
    pub pull_request: Option<PullRequest>,
    pub remote: Option<GitRemote>,
}

struct CommitAvatar<'a> {
    commit: &'a CommitDetails,
}

impl<'a> CommitAvatar<'a> {
    fn new(details: &'a CommitDetails) -> Self {
        Self { commit: details }
    }
}

impl<'a> CommitAvatar<'a> {
    fn render(
        &'a self,
        window: &mut Window,
        cx: &mut Context<CommitTooltip>,
    ) -> Option<impl IntoElement> {
        let remote = self
            .commit
            .message
            .as_ref()
            .and_then(|details| details.remote.as_ref())
            .filter(|remote| remote.host_supports_avatars())?;

        let avatar_url = CommitAvatarAsset::new(remote.clone(), self.commit.sha.clone());

        let element = match window.use_asset::<CommitAvatarAsset>(&avatar_url, cx) {
            // Loading or no avatar found
            None | Some(None) => Icon::new(IconName::Person)
                .color(Color::Muted)
                .into_element()
                .into_any(),
            // Found
            Some(Some(url)) => Avatar::new(url.to_string()).into_element().into_any(),
        };
        Some(element)
    }
}

#[derive(Clone, Debug)]
struct CommitAvatarAsset {
    sha: SharedString,
    remote: GitRemote,
}

impl Hash for CommitAvatarAsset {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sha.hash(state);
        self.remote.host.name().hash(state);
    }
}

impl CommitAvatarAsset {
    fn new(remote: GitRemote, sha: SharedString) -> Self {
        Self { remote, sha }
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
                .avatar_url(source.sha, client)
                .await
                .map(|url| SharedString::from(url.to_string()))
        }
    }
}

pub struct CommitTooltip {
    commit: CommitDetails,
    scroll_handle: ScrollHandle,
    markdown: Entity<Markdown>,
}

impl CommitTooltip {
    pub fn blame_entry(
        blame: &BlameEntry,
        details: Option<ParsedCommitMessage>,
        window: &mut Window,
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
                committer_name: blame
                    .committer_name
                    .clone()
                    .unwrap_or("<no name>".to_string())
                    .into(),
                committer_email: blame
                    .committer_email
                    .clone()
                    .unwrap_or("".to_string())
                    .into(),
                message: details,
            },
            window,
            cx,
        )
    }

    pub fn new(commit: CommitDetails, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut style = hover_markdown_style(window, cx);
        if let Some(code_block) = &style.code_block.text {
            style.base_text_style.refine(code_block);
        }
        let markdown = cx.new(|cx| {
            Markdown::new(
                commit
                    .message
                    .as_ref()
                    .map(|message| message.message.clone())
                    .unwrap_or_default(),
                style,
                None,
                None,
                cx,
            )
        });
        Self {
            commit,
            scroll_handle: ScrollHandle::new(),
            markdown,
        }
    }
}

impl Render for CommitTooltip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let avatar = CommitAvatar::new(&self.commit).render(window, cx);

        let author = self.commit.committer_name.clone();

        let author_email = self.commit.committer_email.clone();

        let short_commit_id = self
            .commit
            .sha
            .get(0..8)
            .map(|sha| sha.to_string().into())
            .unwrap_or_else(|| self.commit.sha.clone());
        let full_sha = self.commit.sha.to_string().clone();
        let absolute_timestamp = format_local_timestamp(
            self.commit.commit_time,
            OffsetDateTime::now_utc(),
            time_format::TimestampFormat::MediumAbsolute,
        );

        let message = self
            .commit
            .message
            .as_ref()
            .map(|_| self.markdown.clone().into_any_element())
            .unwrap_or("<no commit message>".into_any());

        let pull_request = self
            .commit
            .message
            .as_ref()
            .and_then(|details| details.pull_request.clone());

        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size;
        let message_max_height = window.line_height() * 12 + (ui_font_size / 0.4);

        tooltip_container(window, cx, move |this, _, cx| {
            this.occlude()
                .on_mouse_move(|_, _, cx| cx.stop_propagation())
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(
                    v_flex()
                        .w(gpui::rems(30.))
                        .gap_4()
                        .child(
                            h_flex()
                                .pb_1p5()
                                .gap_x_2()
                                .overflow_x_hidden()
                                .flex_wrap()
                                .children(avatar)
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
                        .child(
                            div()
                                .id("inline-blame-commit-message")
                                .child(message)
                                .max_h(message_max_height)
                                .overflow_y_scroll()
                                .track_scroll(&self.scroll_handle),
                        )
                        .child(
                            h_flex()
                                .text_color(cx.theme().colors().text_muted)
                                .w_full()
                                .justify_between()
                                .pt_1p5()
                                .border_t_1()
                                .border_color(cx.theme().colors().border_variant)
                                .child(absolute_timestamp)
                                .child(
                                    h_flex()
                                        .gap_1p5()
                                        .when_some(pull_request, |this, pr| {
                                            this.child(
                                                Button::new(
                                                    "pull-request-button",
                                                    format!("#{}", pr.number),
                                                )
                                                .color(Color::Muted)
                                                .icon(IconName::PullRequest)
                                                .icon_color(Color::Muted)
                                                .icon_position(IconPosition::Start)
                                                .style(ButtonStyle::Subtle)
                                                .on_click(move |_, _, cx| {
                                                    cx.stop_propagation();
                                                    cx.open_url(pr.url.as_str())
                                                }),
                                            )
                                        })
                                        .child(Divider::vertical())
                                        .child(
                                            Button::new(
                                                "commit-sha-button",
                                                short_commit_id.clone(),
                                            )
                                            .style(ButtonStyle::Subtle)
                                            .color(Color::Muted)
                                            .icon(IconName::FileGit)
                                            .icon_color(Color::Muted)
                                            .icon_position(IconPosition::Start)
                                            .disabled(
                                                self.commit
                                                    .message
                                                    .as_ref()
                                                    .map_or(true, |details| {
                                                        details.permalink.is_none()
                                                    }),
                                            )
                                            .when_some(
                                                self.commit
                                                    .message
                                                    .as_ref()
                                                    .and_then(|details| details.permalink.clone()),
                                                |this, url| {
                                                    this.on_click(move |_, _, cx| {
                                                        cx.stop_propagation();
                                                        cx.open_url(url.as_str())
                                                    })
                                                },
                                            ),
                                        )
                                        .child(
                                            IconButton::new("copy-sha-button", IconName::Copy)
                                                .shape(IconButtonShape::Square)
                                                .icon_size(IconSize::Small)
                                                .icon_color(Color::Muted)
                                                .on_click(move |_, _, cx| {
                                                    cx.stop_propagation();
                                                    cx.write_to_clipboard(
                                                        ClipboardItem::new_string(full_sha.clone()),
                                                    )
                                                }),
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
            let local = chrono::Local::now().offset().local_minus_utc();
            time_format::format_localized_timestamp(
                timestamp,
                time::OffsetDateTime::now_utc(),
                UtcOffset::from_whole_seconds(local).unwrap(),
                format,
            )
        }
        Err(_) => "Error parsing date".to_string(),
    }
}

pub fn blame_entry_relative_timestamp(blame_entry: &BlameEntry) -> String {
    blame_entry_timestamp(blame_entry, time_format::TimestampFormat::Relative)
}
