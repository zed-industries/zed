use std::sync::Arc;

use editor::{Editor, hover_markdown_style};
use gpui::{App, ClickEvent, Entity, EventEmitter, FocusHandle, Focusable, Task, WeakEntity};
use language::LanguageRegistry;
use markdown::{Markdown, MarkdownElement};
use ui::{Chip, KeyBinding, Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::{
    Toast, Workspace,
    item::{Item, ItemEvent},
    notifications::NotificationId,
};

use super::{
    OpenOnGitHub, PullRequestOverview, ReviewCopy, ReviewEvent, ReviewNotes, open_file_at_line,
    read_notes, refresh_review_views, set_review_body, submit_review, write_notes,
};

/// Shares the comment box keymap: cmd-enter submits, escape cancels.
const REVIEW_KEY_CONTEXT: &str = "PrReviewModal";

const MAX_LINE_COMMENT_LINES: usize = 4;
const MAX_BOT_COMMENT_LINES: usize = 6;

/// A tab with the pull request's description, checks, reviews and comments.
pub(crate) struct PullRequestOverviewView {
    copy: ReviewCopy,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    /// The last loaded data, kept on screen while a reload runs.
    loaded: Option<(PullRequestOverview, ReviewNotes)>,
    loading: bool,
    error: Option<SharedString>,
    description: Entity<Markdown>,
    conversation: Entity<Markdown>,
    review_editor: Entity<Editor>,
    submitting: bool,
    submit_error: Option<SharedString>,
    _load: Task<()>,
}

impl PullRequestOverviewView {
    fn new(
        copy: ReviewCopy,
        workspace: WeakEntity<Workspace>,
        language_registry: Arc<LanguageRegistry>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let review_editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(2, 8, window, cx);
            editor.set_placeholder_text("Leave an overall comment (optional)", window, cx);
            editor
        });
        let description = cx.new(|cx| {
            Markdown::new(
                SharedString::default(),
                Some(language_registry.clone()),
                None,
                cx,
            )
        });
        let conversation =
            cx.new(|cx| Markdown::new(SharedString::default(), Some(language_registry), None, cx));
        let mut view = Self {
            copy,
            workspace,
            focus_handle: cx.focus_handle(),
            loaded: None,
            loading: false,
            error: None,
            description,
            conversation,
            review_editor,
            submitting: false,
            submit_error: None,
            _load: Task::ready(()),
        };
        view.reload(cx);
        view
    }

    pub(crate) fn reload(&mut self, cx: &mut Context<Self>) {
        self.loading = true;
        let copy = self.copy.clone();
        let show_cached_first = self.loaded.is_none();
        self._load = cx.spawn(async move |this, cx| {
            if show_cached_first {
                let cached = cx
                    .background_spawn({
                        let copy = copy.clone();
                        async move {
                            let overview = super::read_cached_overview(&copy).await?;
                            let notes = super::read_notes(&copy)
                                .await
                                .log_err()
                                .flatten()
                                .map(|notes| ReviewNotes::parse(&notes))
                                .unwrap_or_default();
                            Some((overview, notes))
                        }
                    })
                    .await;
                if let Some((overview, notes)) = cached {
                    this.update(cx, |this, cx| {
                        if this.loaded.is_none() {
                            this.apply_loaded(overview, notes, cx);
                        }
                    })
                    .log_err();
                }
            }

            let result = cx
                .background_spawn(async move {
                    let overview = super::load_overview(&copy).await?;
                    let notes = super::read_notes(&copy)
                        .await?
                        .map(|notes| ReviewNotes::parse(&notes))
                        .unwrap_or_default();
                    anyhow::Ok((overview, notes))
                })
                .await;
            this.update(cx, |this, cx| {
                this.loading = false;
                match result {
                    Ok((overview, notes)) => {
                        this.error = None;
                        this.apply_loaded(overview, notes, cx);
                    }
                    Err(error) => {
                        this.error = Some(format!("{error:#}").into());
                    }
                }
                cx.notify();
            })
            .log_err();
        });
        cx.notify();
    }

    fn apply_loaded(
        &mut self,
        overview: PullRequestOverview,
        notes: ReviewNotes,
        cx: &mut Context<Self>,
    ) {
        let description = if overview.details.body.trim().is_empty() {
            "_No description._".to_string()
        } else {
            strip_html(&overview.details.body)
        };
        self.description
            .update(cx, |markdown, cx| markdown.reset(description.into(), cx));
        let conversation = conversation_markdown(&overview);
        self.conversation
            .update(cx, |markdown, cx| markdown.reset(conversation.into(), cx));
        self.prefill_review_body(&notes.body, cx);
        self.loaded = Some((overview, notes));
        cx.notify();
    }

    /// Shows the saved review summary, without touching anything typed since.
    fn prefill_review_body(&mut self, body: &str, cx: &mut Context<Self>) {
        if body.is_empty() || !self.review_editor.read(cx).text(cx).is_empty() {
            return;
        }
        let buffer = self.review_editor.read(cx).buffer().read(cx).as_singleton();
        if let Some(buffer) = buffer {
            buffer.update(cx, |buffer, cx| buffer.set_text(body, cx));
        }
    }

    pub(crate) fn focus_review_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.review_editor.focus_handle(cx), cx);
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.submit(ReviewEvent::Comment, window, cx);
    }

    fn cancel(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.focus_handle, cx);
    }

    fn submit(&mut self, event: ReviewEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.submitting {
            return;
        }
        self.submitting = true;
        self.submit_error = None;
        cx.notify();

        let body = self.review_editor.read(cx).text(cx);
        let copy = self.copy.clone();
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |this, cx| {
            let result = cx
                .background_spawn(async move {
                    let existing = read_notes(&copy).await?;
                    write_notes(&copy, &set_review_body(existing.as_deref(), &body)).await?;
                    submit_review(&copy, event).await
                })
                .await;
            match result {
                Ok(url) => {
                    this.update_in(cx, |this, window, cx| {
                        this.submitting = false;
                        this.review_editor
                            .update(cx, |editor, cx| editor.clear(window, cx));
                        cx.notify();
                    })
                    .log_err();
                    workspace
                        .update(cx, |workspace, cx| {
                            refresh_review_views(workspace, cx);
                            workspace.show_toast(
                                Toast::new(
                                    NotificationId::unique::<PullRequestOverviewView>(),
                                    format!("Review posted: {}", event.label()),
                                )
                                .on_click("Open", move |_window, cx| cx.open_url(&url)),
                                cx,
                            );
                        })
                        .log_err();
                }
                Err(error) => {
                    this.update(cx, |this, cx| {
                        this.submitting = false;
                        this.submit_error = Some(format!("{error:#}").into());
                        cx.notify();
                    })
                    .log_err();
                }
            }
        })
        .detach();
    }

    fn render_review_section(&self, notes: &ReviewNotes, cx: &mut Context<Self>) -> AnyElement {
        let pending_label = match notes.comments.len() {
            0 => "No pending comments".to_string(),
            1 => "1 pending comment".to_string(),
            count => format!("{count} pending comments"),
        };
        let editor_focus_handle = self.review_editor.focus_handle(cx);
        let submit_button = |id: &'static str, event: ReviewEvent, color: Color| {
            Button::new(id, event.label())
                .size(ButtonSize::Compact)
                .label_size(LabelSize::Small)
                .color(color)
                .disabled(self.submitting)
                .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                    this.submit(event, window, cx)
                }))
        };

        v_flex()
            .key_context(REVIEW_KEY_CONTEXT)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .gap_1p5()
            .p_2()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                h_flex()
                    .gap_1p5()
                    .items_baseline()
                    .child(
                        Label::new("Your review")
                            .size(LabelSize::Small)
                            .weight(gpui::FontWeight::SEMIBOLD),
                    )
                    .child(
                        Label::new(pending_label)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(
                div()
                    .px_1p5()
                    .py_1()
                    .rounded_sm()
                    .border_1()
                    .border_color(cx.theme().colors().border_variant)
                    .bg(cx.theme().colors().editor_background)
                    .child(self.review_editor.clone()),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        submit_button("review-comment", ReviewEvent::Comment, Color::Default)
                            .key_binding(
                                KeyBinding::for_action_in(&menu::Confirm, &editor_focus_handle, cx)
                                    .size(rems_from_px(11_f32)),
                            ),
                    )
                    .child(submit_button(
                        "review-approve",
                        ReviewEvent::Approve,
                        Color::Success,
                    ))
                    .child(submit_button(
                        "review-request-changes",
                        ReviewEvent::RequestChanges,
                        Color::Error,
                    ))
                    .when(self.submitting, |this| {
                        this.child(
                            Label::new("Submitting…")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    }),
            )
            .when_some(self.submit_error.clone(), |this, error| {
                this.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
            })
            .into_any_element()
    }

    fn render_header(&self, overview: &PullRequestOverview, cx: &Context<Self>) -> AnyElement {
        let details = &overview.details;
        let state = if details.is_draft {
            format!("{} (draft)", details.state.to_lowercase())
        } else {
            details.state.to_lowercase()
        };
        let summary = format!(
            "@{} · {} → {} · {} · {} files +{} −{}",
            details.author.login,
            details.head_ref_name,
            details.base_ref_name,
            state,
            details.changed_files,
            details.additions,
            details.deletions,
        );
        let review_decision = details
            .review_decision
            .as_deref()
            .filter(|decision| !decision.is_empty())
            .map(|decision| decision.replace('_', " ").to_lowercase());

        v_flex()
            .flex_1()
            .min_w_0()
            .gap_1()
            .child(
                h_flex()
                    .flex_wrap()
                    .gap_1p5()
                    .items_baseline()
                    .child(
                        div()
                            .min_w_0()
                            .child(Headline::new(details.title.clone()).size(HeadlineSize::Small)),
                    )
                    .child(
                        Label::new(format!("#{}", details.number))
                            .color(Color::Muted)
                            .buffer_font(cx),
                    ),
            )
            .child(
                Label::new(summary)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .when(
                !details.labels.is_empty() || review_decision.is_some(),
                |this| {
                    this.child(
                        h_flex()
                            .gap_1()
                            .flex_wrap()
                            .children(
                                details
                                    .labels
                                    .iter()
                                    .map(|label| Chip::new(label.name.clone())),
                            )
                            .when_some(review_decision, |this, decision| {
                                this.child(
                                    Label::new(format!("Review: {decision}"))
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                            }),
                    )
                },
            )
            .into_any_element()
    }

    fn render_actions(&self, cx: &Context<Self>) -> AnyElement {
        h_flex()
            .gap_1()
            .child(
                IconButton::new("pr-overview-refresh", IconName::ArrowCircle)
                    .icon_size(IconSize::Small)
                    .size(ButtonSize::Compact)
                    .disabled(self.loading)
                    .tooltip(Tooltip::text("Refresh"))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| this.reload(cx))),
            )
            .child(
                IconButton::new("pr-overview-github", IconName::Github)
                    .icon_size(IconSize::Small)
                    .size(ButtonSize::Compact)
                    .tooltip(Tooltip::text("Open on GitHub"))
                    .on_click(|_, window, cx| window.dispatch_action(Box::new(OpenOnGitHub), cx)),
            )
            .when(self.loading, |this| {
                this.child(
                    Label::new("Loading…")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
            })
            .into_any_element()
    }

    fn render_checks(&self, overview: &PullRequestOverview) -> AnyElement {
        let content = if overview.checks.is_empty() {
            empty_label("No checks.")
        } else {
            v_flex()
                .gap_0p5()
                .children(overview.checks.iter().map(|check| {
                    let (icon, color) = match check.state.to_uppercase().as_str() {
                        "SUCCESS" => (IconName::Check, Color::Success),
                        "FAILURE" | "ERROR" | "CANCELLED" | "TIMED_OUT" => {
                            (IconName::XCircle, Color::Error)
                        }
                        _ => (IconName::Circle, Color::Warning),
                    };
                    h_flex()
                        .gap_1p5()
                        .child(Icon::new(icon).size(IconSize::Small).color(color))
                        .child(Label::new(check.name.clone()).size(LabelSize::Small))
                }))
                .into_any_element()
        };
        section("Checks", content)
    }

    fn render_reviews(&self, overview: &PullRequestOverview) -> AnyElement {
        let reviews = &overview.details.latest_reviews;
        let content = if reviews.is_empty() {
            empty_label("No reviews yet.")
        } else {
            v_flex()
                .gap_0p5()
                .children(reviews.iter().map(|review| {
                    Label::new(format!(
                        "@{} {}",
                        review.author.login,
                        review.state.replace('_', " ").to_lowercase()
                    ))
                    .size(LabelSize::Small)
                }))
                .into_any_element()
        };
        section("Reviews", content)
    }

    fn render_pending_comments(&self, notes: &ReviewNotes, cx: &Context<Self>) -> AnyElement {
        let title = format!("Your pending comments ({})", notes.comments.len());
        let content = if notes.comments.is_empty() {
            empty_label("None. Add one with \"pr review: add comment\" on a line.")
        } else {
            v_flex()
                .gap_1()
                .child(
                    Label::new("These are posted when you submit the review.")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .children(notes.comments.iter().enumerate().map(|(ix, comment)| {
                    self.render_comment_row(
                        ElementId::NamedInteger("pending-comment".into(), ix as u64),
                        comment.path.clone(),
                        comment.line,
                        comment.text.clone(),
                        cx,
                    )
                }))
                .into_any_element()
        };
        section(title, content)
    }

    fn render_line_comments(
        &self,
        overview: &PullRequestOverview,
        cx: &Context<Self>,
    ) -> AnyElement {
        let comments = &overview.line_comments;
        let content = if comments.is_empty() {
            empty_label("No line comments.")
        } else {
            v_flex()
                .gap_1()
                .children(comments.iter().enumerate().map(|(ix, comment)| {
                    let body = comment
                        .body
                        .lines()
                        .take(MAX_LINE_COMMENT_LINES)
                        .collect::<Vec<_>>()
                        .join("\n");
                    let truncated = comment.body.lines().count() > MAX_LINE_COMMENT_LINES;
                    let text = format!(
                        "@{}: {}{}",
                        comment.user.login,
                        body,
                        if truncated { " …" } else { "" }
                    );
                    self.render_comment_row(
                        ElementId::NamedInteger("line-comment".into(), ix as u64),
                        comment.path.clone(),
                        comment.display_line().unwrap_or(1),
                        text,
                        cx,
                    )
                }))
                .into_any_element()
        };
        section("Line comments", content)
    }

    fn render_comment_row(
        &self,
        id: ElementId,
        path: String,
        line: u32,
        text: String,
        cx: &Context<Self>,
    ) -> AnyElement {
        let location = format!("{path}:{line}");
        v_flex()
            .id(id)
            .w_full()
            .px_1p5()
            .py_1()
            .gap_0p5()
            .rounded_sm()
            .cursor_pointer()
            .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
            .child(
                Label::new(location)
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .buffer_font(cx),
            )
            .child(Label::new(text).size(LabelSize::Small))
            .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                open_file_at_line(
                    this.workspace.clone(),
                    this.copy.dir.join(&path),
                    line,
                    window,
                    cx,
                );
            }))
            .into_any_element()
    }
}

fn conversation_markdown(overview: &PullRequestOverview) -> String {
    let comments = &overview.details.comments;
    if comments.is_empty() {
        return "_No comments._".to_string();
    }
    comments
        .iter()
        .map(|comment| {
            let date = comment.created_at.get(..10).unwrap_or(&comment.created_at);
            let mut body = strip_html(&comment.body);
            if is_bot(&comment.author.login) {
                body = first_lines(&body, MAX_BOT_COMMENT_LINES);
            }
            format!("**@{}** ({date}):\n\n{body}", comment.author.login)
        })
        .collect::<Vec<_>>()
        .join("\n\n---\n\n")
}

/// Review bots post long reports; the full text stays on GitHub.
fn is_bot(login: &str) -> bool {
    login.ends_with("[bot]") || matches!(login, "coderabbitai" | "copilot-pull-request-reviewer")
}

/// The first `max_lines` non-empty lines, with "…" when anything was cut.
fn first_lines(text: &str, max_lines: usize) -> String {
    let mut kept = Vec::new();
    let mut non_empty = 0;
    let mut lines = text.lines();
    for line in lines.by_ref() {
        if !line.trim().is_empty() {
            if non_empty == max_lines {
                kept.push("_…_");
                return kept.join("\n");
            }
            non_empty += 1;
        }
        kept.push(line);
    }
    kept.join("\n").trim_end().to_string()
}

/// GitHub bodies mix Markdown with HTML that the Markdown renderer shows as raw
/// tags. Keeps the text, turns `<br>` into a line break, and decodes entities.
fn strip_html(text: &str) -> String {
    let mut without_comments = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("<!--") {
        without_comments.push_str(&rest[..start]);
        rest = match rest[start..].find("-->") {
            Some(end) => &rest[start + end + 3..],
            None => "",
        };
    }
    without_comments.push_str(rest);

    let mut stripped = String::with_capacity(without_comments.len());
    let mut rest = without_comments.as_str();
    while let Some(start) = rest.find('<') {
        stripped.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        let looks_like_tag = after
            .chars()
            .next()
            .is_some_and(|next| next.is_ascii_alphabetic() || next == '/');
        match after.find('>').filter(|_| looks_like_tag) {
            Some(end) => {
                let tag_name = after[..end]
                    .trim_start_matches('/')
                    .split(|character: char| !character.is_ascii_alphanumeric())
                    .next()
                    .unwrap_or_default();
                if tag_name.eq_ignore_ascii_case("br") {
                    stripped.push('\n');
                }
                rest = &after[end + 1..];
            }
            None => {
                stripped.push('<');
                rest = after;
            }
        }
    }
    stripped.push_str(rest);

    let decoded = stripped
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&");

    let mut collapsed = String::with_capacity(decoded.len());
    let mut blank_lines = 0;
    for line in decoded.lines() {
        if line.trim().is_empty() {
            blank_lines += 1;
            if blank_lines > 1 {
                continue;
            }
        } else {
            blank_lines = 0;
        }
        collapsed.push_str(line);
        collapsed.push('\n');
    }
    collapsed.trim().to_string()
}

fn section(title: impl Into<SharedString>, content: AnyElement) -> AnyElement {
    v_flex()
        .gap_1()
        .child(
            Label::new(title)
                .size(LabelSize::Small)
                .weight(gpui::FontWeight::SEMIBOLD),
        )
        .child(content)
        .into_any_element()
}

fn empty_label(text: &'static str) -> AnyElement {
    Label::new(text)
        .size(LabelSize::Small)
        .color(Color::Muted)
        .into_any_element()
}

pub(crate) fn open_overview(
    workspace: &mut Workspace,
    copy: ReviewCopy,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Entity<PullRequestOverviewView> {
    let existing = workspace
        .items_of_type::<PullRequestOverviewView>(cx)
        .find(|view| view.read(cx).copy == copy);
    if let Some(view) = existing {
        workspace.activate_item(&view, true, true, window, cx);
        view.update(cx, |view, cx| view.reload(cx));
        return view;
    }

    let language_registry = workspace.project().read(cx).languages().clone();
    let workspace_handle = cx.weak_entity();
    let view = cx.new(|cx| {
        PullRequestOverviewView::new(copy, workspace_handle, language_registry, window, cx)
    });
    workspace.add_item_to_active_pane(Box::new(view.clone()), None, true, window, cx);
    view
}

impl EventEmitter<ItemEvent> for PullRequestOverviewView {}

impl Focusable for PullRequestOverviewView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for PullRequestOverviewView {
    type Event = ItemEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::PullRequest))
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("PR #{}", self.copy.pr_number).into()
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<SharedString> {
        self.loaded
            .as_ref()
            .map(|(overview, _)| overview.details.title.clone().into())
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        f(*event)
    }
}

impl Render for PullRequestOverviewView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let markdown_style = hover_markdown_style(window, cx);
        let content = match &self.loaded {
            Some((overview, notes)) => v_flex()
                .gap_4()
                .child(
                    h_flex()
                        .gap_2()
                        .items_start()
                        .justify_between()
                        .child(self.render_header(overview, cx))
                        .child(div().flex_none().child(self.render_actions(cx))),
                )
                .child(self.render_review_section(notes, cx))
                .when_some(self.error.clone(), |this, error| {
                    this.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
                })
                .child(section(
                    "Description",
                    div()
                        .child(MarkdownElement::new(
                            self.description.clone(),
                            markdown_style.clone(),
                        ))
                        .into_any_element(),
                ))
                .child(self.render_checks(overview))
                .child(self.render_reviews(overview))
                .child(self.render_pending_comments(notes, cx))
                .child(self.render_line_comments(overview, cx))
                .child(section(
                    "Conversation",
                    div()
                        .child(MarkdownElement::new(
                            self.conversation.clone(),
                            markdown_style,
                        ))
                        .into_any_element(),
                ))
                .into_any_element(),
            None => v_flex()
                .gap_2()
                .child(
                    Headline::new(format!("Pull request #{}", self.copy.pr_number))
                        .size(HeadlineSize::Small),
                )
                .child(self.render_actions(cx))
                .map(|this| match &self.error {
                    Some(error) => this.child(
                        Label::new(error.clone())
                            .size(LabelSize::Small)
                            .color(Color::Error),
                    ),
                    None => this.child(
                        Label::new("Loading the pull request from GitHub…")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
                })
                .into_any_element(),
        };

        div()
            .id("pr-overview")
            .track_focus(&self.focus_handle)
            .size_full()
            .overflow_y_scroll()
            .bg(cx.theme().colors().editor_background)
            .child(div().max_w(rems(48.)).p_4().child(content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_html() {
        let body = "<!-- walkthrough -->\n<details>\n<summary>Summary &amp; notes</summary>\n\n\n\nLine one<br>Line two <a href=\"https://x\">link</a>\n<dl><dd><sub>small</sub></dd></dl>\n\nif a &lt; b &gt; c\n</details>";
        assert_eq!(
            strip_html(body),
            "Summary & notes\n\nLine one\nLine two link\nsmall\n\nif a < b > c"
        );
        assert_eq!(strip_html("a < b and 1<2"), "a < b and 1<2");
    }

    #[test]
    fn test_first_lines_and_bots() {
        let text = "1\n\n2\n3\n4\n5\n6\n7";
        assert_eq!(first_lines(text, 6), "1\n\n2\n3\n4\n5\n6\n_…_");
        assert_eq!(first_lines("a\nb", 6), "a\nb");
        assert!(is_bot("github-actions[bot]"));
        assert!(is_bot("coderabbitai"));
        assert!(!is_bot("omkar273"));
    }
}
