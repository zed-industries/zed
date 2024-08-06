use futures::Future;
use git::blame::BlameEntry;
use git::Oid;
use gpui::{
    AppContext, Asset, ClipboardItem, Element, ParentElement, Render, ScrollHandle,
    StatefulInteractiveElement, WeakView,
};
use settings::Settings;
use std::hash::Hash;
use theme::ThemeSettings;
use time::UtcOffset;
use ui::{prelude::*, tooltip_container, Avatar};
use workspace::Workspace;

use crate::git::blame::{CommitDetails, GitRemote};
use crate::EditorStyle;

struct CommitAvatar<'a> {
    details: Option<&'a CommitDetails>,
    sha: Oid,
}

impl<'a> CommitAvatar<'a> {
    fn new(details: Option<&'a CommitDetails>, sha: Oid) -> Self {
        Self { details, sha }
    }
}

impl<'a> CommitAvatar<'a> {
    fn render(&'a self, cx: &mut ViewContext<BlameEntryTooltip>) -> Option<impl IntoElement> {
        let remote = self
            .details
            .and_then(|details| details.remote.as_ref())
            .filter(|remote| remote.host_supports_avatars())?;

        let avatar_url = CommitAvatarAsset::new(remote.clone(), self.sha);

        let element = match cx.use_asset::<CommitAvatarAsset>(&avatar_url) {
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
    sha: Oid,
    remote: GitRemote,
}

impl Hash for CommitAvatarAsset {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sha.hash(state);
        self.remote.host.name().hash(state);
    }
}

impl CommitAvatarAsset {
    fn new(remote: GitRemote, sha: Oid) -> Self {
        Self { remote, sha }
    }
}

impl Asset for CommitAvatarAsset {
    type Source = Self;
    type Output = Option<SharedString>;

    fn load(
        source: Self::Source,
        cx: &mut AppContext,
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

pub(crate) struct BlameEntryTooltip {
    blame_entry: BlameEntry,
    details: Option<CommitDetails>,
    editor_style: EditorStyle,
    workspace: Option<WeakView<Workspace>>,
    scroll_handle: ScrollHandle,
}

impl BlameEntryTooltip {
    pub(crate) fn new(
        blame_entry: BlameEntry,
        details: Option<CommitDetails>,
        style: &EditorStyle,
        workspace: Option<WeakView<Workspace>>,
    ) -> Self {
        Self {
            editor_style: style.clone(),
            blame_entry,
            details,
            workspace,
            scroll_handle: ScrollHandle::new(),
        }
    }
}

impl Render for BlameEntryTooltip {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let avatar = CommitAvatar::new(self.details.as_ref(), self.blame_entry.sha).render(cx);

        let author = self
            .blame_entry
            .author
            .clone()
            .unwrap_or("<no name>".to_string());

        let author_email = self.blame_entry.author_mail.clone();

        let short_commit_id = self.blame_entry.sha.display_short();
        let full_sha = self.blame_entry.sha.to_string().clone();
        let absolute_timestamp = blame_entry_absolute_timestamp(&self.blame_entry);

        let message = self
            .details
            .as_ref()
            .map(|details| {
                crate::render_parsed_markdown(
                    "blame-message",
                    &details.parsed_message,
                    &self.editor_style,
                    self.workspace.clone(),
                    cx,
                )
                .into_any()
            })
            .unwrap_or("<no commit message>".into_any());

        let pull_request = self
            .details
            .as_ref()
            .and_then(|details| details.pull_request.clone());

        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size;
        let message_max_height = cx.line_height() * 12 + (ui_font_size / 0.4);

        tooltip_container(cx, move |this, cx| {
            this.occlude()
                .on_mouse_move(|_, cx| cx.stop_propagation())
                .child(
                    v_flex()
                        .w(gpui::rems(30.))
                        .gap_4()
                        .child(
                            h_flex()
                                .gap_x_2()
                                .overflow_x_hidden()
                                .flex_wrap()
                                .children(avatar)
                                .child(author)
                                .when_some(author_email, |this, author_email| {
                                    this.child(
                                        div()
                                            .text_color(cx.theme().colors().text_muted)
                                            .child(author_email),
                                    )
                                })
                                .border_b_1()
                                .border_color(cx.theme().colors().border),
                        )
                        .child(
                            div()
                                .id("inline-blame-commit-message")
                                .occlude()
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
                                .child(absolute_timestamp)
                                .child(
                                    h_flex()
                                        .gap_2()
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
                                                .style(ButtonStyle::Transparent)
                                                .on_click(move |_, cx| {
                                                    cx.stop_propagation();
                                                    cx.open_url(pr.url.as_str())
                                                }),
                                            )
                                        })
                                        .child(
                                            Button::new(
                                                "commit-sha-button",
                                                short_commit_id.clone(),
                                            )
                                            .style(ButtonStyle::Transparent)
                                            .color(Color::Muted)
                                            .icon(IconName::FileGit)
                                            .icon_color(Color::Muted)
                                            .icon_position(IconPosition::Start)
                                            .disabled(
                                                self.details.as_ref().map_or(true, |details| {
                                                    details.permalink.is_none()
                                                }),
                                            )
                                            .when_some(
                                                self.details
                                                    .as_ref()
                                                    .and_then(|details| details.permalink.clone()),
                                                |this, url| {
                                                    this.on_click(move |_, cx| {
                                                        cx.stop_propagation();
                                                        cx.open_url(url.as_str())
                                                    })
                                                },
                                            ),
                                        )
                                        .child(
                                            IconButton::new("copy-sha-button", IconName::Copy)
                                                .icon_color(Color::Muted)
                                                .on_click(move |_, cx| {
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

fn blame_entry_absolute_timestamp(blame_entry: &BlameEntry) -> String {
    blame_entry_timestamp(blame_entry, time_format::TimestampFormat::MediumAbsolute)
}
