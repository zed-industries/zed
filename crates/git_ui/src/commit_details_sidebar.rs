use crate::commit_tooltip::CommitAvatar;
use crate::git_status_icon;
use git::status::FileStatus;
use git::{
    BuildCommitPermalinkParams, GitHostingProviderRegistry, GitRemote, ParsedGitRemote,
    parse_git_remote_url, repository::RepoPath,
};
use gpui::{
    AnyElement, App, ClickEvent, FontWeight, Hsla, InteractiveElement, IntoElement, ParentElement,
    SharedString, Styled, Window, px,
};
use project::git_store::Repository;
use std::sync::Arc;
use time::{OffsetDateTime, UtcOffset};
use ui::{
    ButtonLike, ButtonStyle, CopyButton, Icon, IconButton, IconName, IconSize, Label, LabelSize,
    Tooltip, prelude::*,
};

pub struct CommitDetailsSidebarData {
    pub sha: SharedString,
    pub author_name: SharedString,
    pub author_email: SharedString,
    pub commit_timestamp: i64,
    pub subject: SharedString,
    pub body: SharedString,
    pub ref_names: Vec<SharedString>,
    pub accent_color: Hsla,
}

impl CommitDetailsSidebarData {
    pub fn new(
        sha: SharedString,
        author_name: SharedString,
        author_email: SharedString,
        commit_timestamp: i64,
        subject: SharedString,
        body: SharedString,
    ) -> Self {
        Self {
            sha,
            author_name,
            author_email,
            commit_timestamp,
            subject,
            body,
            ref_names: Vec::new(),
            accent_color: gpui::hsla(0.0, 0.0, 0.5, 1.0),
        }
    }

    pub fn with_ref_names(mut self, ref_names: Vec<SharedString>) -> Self {
        self.ref_names = ref_names;
        self
    }

    pub fn with_accent_color(mut self, accent_color: Hsla) -> Self {
        self.accent_color = accent_color;
        self
    }
}

pub struct CommitDetailsSidebar {
    data: CommitDetailsSidebarData,
    remote: Option<GitRemote>,
    changed_files: Vec<(RepoPath, FileStatus)>,
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_file_click: Option<Arc<dyn Fn(&RepoPath, &ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl CommitDetailsSidebar {
    pub fn new(data: CommitDetailsSidebarData) -> Self {
        Self {
            data,
            remote: None,
            changed_files: Vec::new(),
            on_close: None,
            on_file_click: None,
        }
    }

    pub fn remote(mut self, remote: Option<GitRemote>) -> Self {
        self.remote = remote;
        self
    }

    pub fn changed_files(mut self, files: Vec<(RepoPath, FileStatus)>) -> Self {
        self.changed_files = files;
        self
    }

    pub fn on_close(mut self, callback: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(callback));
        self
    }

    pub fn on_file_click(
        mut self,
        callback: impl Fn(&RepoPath, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_file_click = Some(Arc::new(callback));
        self
    }

    pub fn render(self, window: &mut Window, cx: &mut App) -> AnyElement {
        let full_sha = self.data.sha.clone();

        let date_string = OffsetDateTime::from_unix_timestamp(self.data.commit_timestamp)
            .ok()
            .map(|datetime| {
                let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
                let local_datetime = datetime.to_offset(local_offset);
                let format =
                    time::format_description::parse("[month repr:short] [day], [year]").ok();
                format
                    .and_then(|f| local_datetime.format(&f).ok())
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let author_email_for_avatar = if self.data.author_email.is_empty() {
            None
        } else {
            Some(self.data.author_email.clone())
        };
        let avatar = CommitAvatar::new(&full_sha, author_email_for_avatar, self.remote.as_ref());

        let avatar_element = v_flex()
            .w(px(32.))
            .h(px(32.))
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_full()
            .justify_center()
            .items_center()
            .child(
                avatar
                    .avatar(window, cx)
                    .map(|a| a.size(px(32.)).into_any_element())
                    .unwrap_or_else(|| {
                        Icon::new(IconName::Person)
                            .color(Color::Muted)
                            .size(IconSize::XLarge)
                            .into_any_element()
                    }),
            );

        let changed_files_count = self.changed_files.len();
        let author_name = self.data.author_name.clone();
        let author_email = self.data.author_email.clone();
        let subject = self.data.subject.clone();
        let body = self.data.body.clone();
        let ref_names = self.data.ref_names.clone();
        let accent_color = self.data.accent_color;
        let remote = self.remote.clone();
        let on_close = self.on_close;
        let on_file_click = self.on_file_click;
        let changed_files = self.changed_files;

        v_flex()
            .w(px(300.))
            .h_full()
            .border_l_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().surface_background)
            // Commit Details
            .child(
                div()
                    .id("commit-details")
                    .w_full()
                    .child(
                        v_flex()
                            .p_3()
                            .gap_3()
                            .child(
                                h_flex()
                                    .w_full()
                                    .px_1()
                                    .items_start()
                                    .justify_between()
                                    .child(avatar_element)
                                    .when_some(on_close, |this, on_close| {
                                        this.child(
                                            IconButton::new("close-detail", IconName::Close)
                                                .icon_size(IconSize::Small)
                                                .on_click(move |_, window, cx| {
                                                    on_close(window, cx);
                                                }),
                                        )
                                    }),
                            )
                            .child(
                                v_flex()
                                    .px_1()
                                    .gap_0p5()
                                    .child(
                                        Label::new(author_name.clone())
                                            .weight(FontWeight::SEMIBOLD),
                                    )
                                    .child(
                                        Label::new(date_string)
                                            .color(Color::Muted)
                                            .size(LabelSize::Small),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .gap_1p5()
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .w_5()
                                                    .h_5()
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        Icon::new(IconName::Person)
                                                            .size(IconSize::Small)
                                                            .color(Color::Muted),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .id("author-name")
                                                    .child(
                                                        Label::new(author_name)
                                                            .size(LabelSize::Small)
                                                            .color(Color::Muted),
                                                    )
                                                    .when(!author_email.is_empty(), |this| {
                                                        this.tooltip(Tooltip::text(author_email))
                                                    }),
                                            ),
                                    )
                                    .child({
                                        let short_sha: SharedString =
                                            full_sha.chars().take(7).collect::<String>().into();
                                        h_flex()
                                            .gap_1()
                                            .child(
                                                CopyButton::new("copy-sha", full_sha.to_string())
                                                    .tooltip_label("Copy SHA"),
                                            )
                                            .child(
                                                Label::new(short_sha)
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted),
                                            )
                                    })
                                    .when_some(remote, |this, remote| {
                                        let provider_name = remote.host.name();
                                        let icon = match provider_name.as_str() {
                                            "GitHub" => IconName::Github,
                                            _ => IconName::Link,
                                        };
                                        let parsed_remote = ParsedGitRemote {
                                            owner: remote.owner.as_ref().into(),
                                            repo: remote.repo.as_ref().into(),
                                        };
                                        let params = BuildCommitPermalinkParams {
                                            sha: full_sha.as_ref(),
                                        };
                                        let url = remote
                                            .host
                                            .build_commit_permalink(&parsed_remote, params)
                                            .to_string();
                                        this.child(
                                            h_flex().child(
                                                ButtonLike::new("view-on-provider")
                                                    .style(ButtonStyle::Subtle)
                                                    .child(
                                                        h_flex()
                                                            .gap_1()
                                                            .child(
                                                                Icon::new(icon)
                                                                    .size(IconSize::Small)
                                                                    .color(Color::Muted),
                                                            )
                                                            .child(
                                                                Label::new(format!(
                                                                    "View on {}",
                                                                    provider_name
                                                                ))
                                                                .size(LabelSize::Small)
                                                                .color(Color::Muted),
                                                            ),
                                                    )
                                                    .on_click(move |_, _, cx| {
                                                        cx.open_url(&url);
                                                    }),
                                            ),
                                        )
                                    }),
                            )
                            .when(!ref_names.is_empty(), |this| {
                                this.child(h_flex().pt_1().gap_1().flex_wrap().children(
                                    ref_names.iter().map(|name| {
                                        render_badge(name, accent_color).into_any_element()
                                    }),
                                ))
                            })
                            .child(
                                v_flex()
                                    .px_1()
                                    .gap_2()
                                    .child(Label::new(subject).weight(FontWeight::MEDIUM))
                                    .when(!body.is_empty(), |this| {
                                        this.child(
                                            Label::new(body)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                    }),
                            ),
                    )
                    .h_1_2()
                    .overflow_y_scroll(),
            )
            // Changes list
            .child(
                div()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .id("changed-files")
                    .w_full()
                    .child(
                        v_flex()
                            .p_3()
                            .gap_2()
                            .child(
                                Label::new(format!("{} Changed Files", changed_files_count))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .when(!changed_files.is_empty(), |this| {
                                this.child(v_flex().gap_1().children(changed_files.iter().map(
                                    |(path, status)| {
                                        let file_name: String = path
                                            .file_name()
                                            .map(|n| n.to_string())
                                            .unwrap_or_default();
                                        let dir_path: String = path
                                            .parent()
                                            .map(|p| p.as_unix_str().to_string())
                                            .unwrap_or_default();

                                        let on_file_click = on_file_click.clone();
                                        let path_for_click = path.clone();

                                        h_flex()
                                            .id(SharedString::from(path.as_unix_str().to_string()))
                                            .gap_1()
                                            .overflow_hidden()
                                            .child(git_status_icon(*status))
                                            .child(
                                                Label::new(file_name)
                                                    .size(LabelSize::Small)
                                                    .single_line(),
                                            )
                                            .when(!dir_path.is_empty(), |this| {
                                                this.child(
                                                    Label::new(dir_path)
                                                        .size(LabelSize::Small)
                                                        .color(Color::Muted)
                                                        .single_line(),
                                                )
                                            })
                                            .when_some(on_file_click, |this, callback| {
                                                this.cursor_pointer().on_click(
                                                    move |event, window, cx| {
                                                        callback(
                                                            &path_for_click,
                                                            event,
                                                            window,
                                                            cx,
                                                        );
                                                    },
                                                )
                                            })
                                    },
                                )))
                            }),
                    )
                    .h_1_2()
                    .overflow_y_scroll(),
            )
            .into_any_element()
    }
}

fn render_badge(name: &SharedString, accent_color: Hsla) -> impl IntoElement {
    div()
        .px_1p5()
        .py_0p5()
        .flex()
        .items_center()
        .justify_center()
        .rounded_md()
        .bg(accent_color.opacity(0.18))
        .border_1()
        .border_color(accent_color.opacity(0.55))
        .child(
            Label::new(name.clone())
                .size(LabelSize::Small)
                .color(Color::Default)
                .single_line(),
        )
}

pub fn get_remote_from_repository(repository: &Repository, cx: &mut App) -> Option<GitRemote> {
    let remote_url = repository.default_remote_url()?;
    let provider_registry = GitHostingProviderRegistry::default_global(cx);
    let (provider, parsed) = parse_git_remote_url(provider_registry, &remote_url)?;
    Some(GitRemote {
        host: provider,
        owner: parsed.owner.into(),
        repo: parsed.repo.into(),
    })
}
