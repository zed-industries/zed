use std::sync::Arc;

use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, ScrollHandle,
    SharedString, Task, WeakEntity, Window,
};
use language::LanguageRegistry;
use markdown::{Markdown, MarkdownElement, MarkdownFont, MarkdownOptions, MarkdownStyle};
use time::OffsetDateTime;
use ui::{Chip, Divider, Headline, HeadlineSize, WithScrollbar, prelude::*};
use util::ResultExt as _;
use workspace::{Workspace, item::Item};

use crate::{
    github_issues::{GithubIssuesClient, IssueDetails},
    github_issues_panel::{github_author_avatar, issue_label_chip, relative_timestamp},
};

/// A read-only view of one GitHub issue, opened in the main pane by the
/// GitHub issues panel.
pub struct GithubIssueView {
    number: u64,
    title: SharedString,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    language_registry: Arc<LanguageRegistry>,
    state: ViewState,
}

enum ViewState {
    /// The task is held so that closing the tab cancels the load.
    Loading {
        _task: Task<()>,
    },
    Loaded {
        details: IssueDetails,
        /// `None` when the issue has no description.
        body: Option<Entity<Markdown>>,
        comments: Vec<CommentEntry>,
    },
    Failed {
        message: SharedString,
    },
}

struct CommentEntry {
    author_login: Option<SharedString>,
    created_at: String,
    body: Entity<Markdown>,
}

/// GitHub issue bodies routinely embed raw HTML (`<details>`, `<img>`), which
/// the parser drops unless asked to handle it.
fn markdown_options() -> MarkdownOptions {
    MarkdownOptions {
        parse_html: true,
        ..MarkdownOptions::default()
    }
}

impl GithubIssueView {
    pub fn open(
        number: u64,
        title: SharedString,
        client: GithubIssuesClient,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        workspace
            .update(cx, |workspace, cx| {
                let language_registry = workspace.project().read(cx).languages().clone();
                let pane = workspace.active_pane().clone();
                pane.update(cx, |pane, cx| {
                    let existing_index = pane.items().position(|item| {
                        item.downcast::<Self>()
                            .is_some_and(|view| view.read(cx).number == number)
                    });
                    if let Some(existing_index) = existing_index {
                        pane.activate_item(existing_index, true, true, window, cx);
                    } else {
                        let view =
                            cx.new(|cx| Self::new(number, title, client, language_registry, cx));
                        pane.add_item(Box::new(view), true, true, None, window, cx);
                    }
                });
            })
            .log_err();
    }

    fn new(
        number: u64,
        title: SharedString,
        client: GithubIssuesClient,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut Context<Self>,
    ) -> Self {
        let task = cx.spawn(async move |this, cx| {
            let details = match this.update(cx, |_, cx| client.load_issue(number, cx)) {
                Ok(load_task) => load_task.await,
                Err(error) => Err(error),
            };
            this.update(cx, |this, cx| {
                match details {
                    Ok(details) => this.finish_loading(details, cx),
                    Err(error) => {
                        this.state = ViewState::Failed {
                            message: format!("{error:#}").into(),
                        }
                    }
                }
                cx.notify();
            })
            .log_err();
        });
        Self {
            number,
            title,
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
            language_registry,
            state: ViewState::Loading { _task: task },
        }
    }

    fn finish_loading(&mut self, details: IssueDetails, cx: &mut Context<Self>) {
        self.title = details.title.clone().into();
        let language_registry = self.language_registry.clone();
        let body_source = details.body.trim().to_string();
        let body = (!body_source.is_empty()).then(|| {
            cx.new(|cx| {
                Markdown::new_with_options(
                    body_source.into(),
                    Some(language_registry.clone()),
                    None,
                    markdown_options(),
                    cx,
                )
            })
        });
        let comments = details
            .comments
            .iter()
            .map(|comment| CommentEntry {
                author_login: comment
                    .author
                    .as_ref()
                    .map(|author| SharedString::from(author.login.clone())),
                created_at: comment.created_at.clone(),
                body: cx.new(|cx| {
                    Markdown::new_with_options(
                        comment.body.clone().into(),
                        Some(language_registry.clone()),
                        None,
                        markdown_options(),
                        cx,
                    )
                }),
            })
            .collect();
        self.state = ViewState::Loaded {
            details,
            body,
            comments,
        };
    }

    fn render_placeholder(&self, message: SharedString) -> AnyElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .p_4()
            .child(Label::new(message).color(Color::Muted))
            .into_any_element()
    }

    fn render_issue(
        &self,
        details: &IssueDetails,
        body: &Option<Entity<Markdown>>,
        comments: &[CommentEntry],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let local_offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
        let now = OffsetDateTime::now_utc();
        let author_login: Option<SharedString> = details
            .author
            .as_ref()
            .map(|author| SharedString::from(author.login.clone()));
        let opened_time = relative_timestamp(&details.created_at, now, local_offset);
        let updated_time = relative_timestamp(&details.updated_at, now, local_offset);
        let (state_name, state_color) = if details.state.eq_ignore_ascii_case("open") {
            ("Open", Color::Success)
        } else {
            ("Closed", Color::Error)
        };

        let dot_separator = || {
            Label::new("•")
                .size(LabelSize::Small)
                .color(Color::Muted)
                .alpha(0.5)
                .flex_none()
        };

        let header = v_flex()
            .gap_2()
            .child(
                h_flex()
                    .gap_2()
                    .flex_wrap()
                    .child(Headline::new(details.title.clone()).size(HeadlineSize::Medium))
                    .child(
                        Headline::new(format!("#{}", details.number))
                            .size(HeadlineSize::Medium)
                            .color(Color::Muted),
                    ),
            )
            .child(
                h_flex()
                    .gap_1p5()
                    .flex_wrap()
                    .child(
                        Chip::new(state_name)
                            .label_color(state_color)
                            .bg_color(state_color.color(cx).opacity(0.15))
                            .border_color(state_color.color(cx).opacity(0.5)),
                    )
                    .child(div().flex_none().child(github_author_avatar(
                        author_login.as_deref(),
                        px(14.),
                        cx,
                    )))
                    .when_some(author_login, |this, author_login| {
                        this.child(
                            Label::new(author_login)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    })
                    .when_some(opened_time, |this, opened_time| {
                        this.child(dot_separator()).child(
                            Label::new(format!("opened {opened_time}"))
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    })
                    .when_some(updated_time, |this, updated_time| {
                        this.child(dot_separator()).child(
                            Label::new(format!("updated {updated_time}"))
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    })
                    .children(
                        details
                            .labels
                            .iter()
                            .map(|label| issue_label_chip(label, cx)),
                    ),
            );

        let body_element: AnyElement = match body {
            Some(body) => MarkdownElement::new(
                body.clone(),
                MarkdownStyle::themed(MarkdownFont::Preview, window, cx),
            )
            .into_any_element(),
            None => Label::new("No description provided.")
                .color(Color::Muted)
                .into_any_element(),
        };

        let comment_cards = comments
            .iter()
            .enumerate()
            .map(|(index, comment)| {
                let comment_time = relative_timestamp(&comment.created_at, now, local_offset);
                v_flex()
                    .id(("github-issue-comment", index))
                    .gap_2()
                    .p_3()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded_md()
                    .child(
                        h_flex()
                            .gap_1p5()
                            .child(div().flex_none().child(github_author_avatar(
                                comment.author_login.as_deref(),
                                px(14.),
                                cx,
                            )))
                            .when_some(comment.author_login.clone(), |this, author_login| {
                                this.child(Label::new(author_login).size(LabelSize::Small))
                            })
                            .when_some(comment_time, |this, comment_time| {
                                this.child(dot_separator()).child(
                                    Label::new(comment_time)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                            }),
                    )
                    .child(MarkdownElement::new(
                        comment.body.clone(),
                        MarkdownStyle::themed(MarkdownFont::Preview, window, cx),
                    ))
            })
            .collect::<Vec<_>>();

        div()
            .relative()
            .size_full()
            .child(
                div()
                    .id("github-issue-view-scroll")
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .p_4()
                    .child(
                        v_flex()
                            .gap_3()
                            .child(header)
                            .child(Divider::horizontal())
                            .child(body_element)
                            .children(comment_cards),
                    ),
            )
            .vertical_scrollbar_for(&self.scroll_handle, window, cx)
            .into_any_element()
    }
}

impl Render for GithubIssueView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match &self.state {
            ViewState::Loading { .. } => self.render_placeholder("Loading issue…".into()),
            ViewState::Failed { message } => self.render_placeholder(message.clone()),
            ViewState::Loaded {
                details,
                body,
                comments,
            } => self.render_issue(details, body, comments, window, cx),
        };

        v_flex()
            .key_context("GithubIssueView")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(content)
    }
}

impl Focusable for GithubIssueView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for GithubIssueView {}

impl Item for GithubIssueView {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("#{} {}", self.number, self.title).into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<ui::Icon> {
        Some(ui::Icon::new(IconName::Github))
    }
}
