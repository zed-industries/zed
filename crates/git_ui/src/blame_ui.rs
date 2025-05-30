use crate::{
    commit_tooltip::{CommitAvatar, CommitDetails, CommitTooltip},
    commit_view::CommitView,
};
use editor::{BlameRenderer, Editor, hover_markdown_style};
use git::{
    blame::{BlameEntry, ParsedCommitMessage},
    repository::CommitSummary,
};
use gpui::{
    ClipboardItem, Entity, Hsla, MouseButton, ScrollHandle, Subscription, TextStyle, WeakEntity,
    prelude::*,
};
use markdown::{Markdown, MarkdownElement};
use project::{git_store::Repository, project_settings::ProjectSettings};
use settings::Settings as _;
use theme::ThemeSettings;
use time::OffsetDateTime;
use time_format::format_local_timestamp;
use ui::{ContextMenu, Divider, IconButtonShape, prelude::*};
use workspace::Workspace;

const GIT_BLAME_MAX_AUTHOR_CHARS_DISPLAYED: usize = 20;

pub struct GitBlameRenderer;

impl BlameRenderer for GitBlameRenderer {
    fn max_author_length(&self) -> usize {
        GIT_BLAME_MAX_AUTHOR_CHARS_DISPLAYED
    }

    fn render_blame_entry(
        &self,
        style: &TextStyle,
        blame_entry: BlameEntry,
        details: Option<ParsedCommitMessage>,
        repository: Entity<Repository>,
        workspace: WeakEntity<Workspace>,
        editor: Entity<Editor>,
        ix: usize,
        sha_color: Hsla,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let relative_timestamp = blame_entry_relative_timestamp(&blame_entry);
        let short_commit_id = blame_entry.sha.display_short();
        let author_name = blame_entry.author.as_deref().unwrap_or("<no name>");
        let name = util::truncate_and_trailoff(author_name, GIT_BLAME_MAX_AUTHOR_CHARS_DISPLAYED);

        Some(
            h_flex()
                .w_full()
                .justify_between()
                .font_family(style.font().family)
                .line_height(style.line_height)
                .id(("blame", ix))
                .text_color(cx.theme().status().hint)
                .pr_2()
                .gap_2()
                .child(
                    h_flex()
                        .items_center()
                        .gap_2()
                        .child(div().text_color(sha_color).child(short_commit_id))
                        .child(name),
                )
                .child(relative_timestamp)
                .hover(|style| style.bg(cx.theme().colors().element_hover))
                .cursor_pointer()
                .on_mouse_down(MouseButton::Right, {
                    let blame_entry = blame_entry.clone();
                    let details = details.clone();
                    move |event, window, cx| {
                        deploy_blame_entry_context_menu(
                            &blame_entry,
                            details.as_ref(),
                            editor.clone(),
                            event.position,
                            window,
                            cx,
                        );
                    }
                })
                .on_click({
                    let blame_entry = blame_entry.clone();
                    let repository = repository.clone();
                    let workspace = workspace.clone();
                    move |_, window, cx| {
                        CommitView::open(
                            CommitSummary {
                                sha: blame_entry.sha.to_string().into(),
                                subject: blame_entry.summary.clone().unwrap_or_default().into(),
                                commit_timestamp: blame_entry.committer_time.unwrap_or_default(),
                                has_parent: true,
                            },
                            repository.downgrade(),
                            workspace.clone(),
                            window,
                            cx,
                        )
                    }
                })
                .hoverable_tooltip(move |_window, cx| {
                    cx.new(|cx| {
                        CommitTooltip::blame_entry(
                            &blame_entry,
                            details.clone(),
                            repository.clone(),
                            workspace.clone(),
                            cx,
                        )
                    })
                    .into()
                })
                .into_any(),
        )
    }

    fn render_inline_blame_entry(
        &self,
        style: &TextStyle,
        blame_entry: BlameEntry,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let relative_timestamp = blame_entry_relative_timestamp(&blame_entry);
        let author = blame_entry.author.as_deref().unwrap_or_default();
        let summary_enabled = ProjectSettings::get_global(cx)
            .git
            .show_inline_commit_summary();

        let text = match blame_entry.summary.as_ref() {
            Some(summary) if summary_enabled => {
                format!("{}, {} - {}", author, relative_timestamp, summary)
            }
            _ => format!("{}, {}", author, relative_timestamp),
        };

        Some(
            h_flex()
                .id("inline-blame")
                .w_full()
                .font_family(style.font().family)
                .text_color(cx.theme().status().hint)
                .line_height(style.line_height)
                .child(Icon::new(IconName::FileGit).color(Color::Hint))
                .child(text)
                .gap_2()
                .into_any(),
        )
    }

    fn render_blame_entry_popover(
        &self,
        blame: BlameEntry,
        scroll_handle: ScrollHandle,
        details: Option<ParsedCommitMessage>,
        markdown: Entity<Markdown>,
        repository: Entity<Repository>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let commit_time = blame
            .committer_time
            .and_then(|t| OffsetDateTime::from_unix_timestamp(t).ok())
            .unwrap_or(OffsetDateTime::now_utc());

        let commit_details = CommitDetails {
            sha: blame.sha.to_string().into(),
            commit_time,
            author_name: blame
                .author
                .clone()
                .unwrap_or("<no name>".to_string())
                .into(),
            author_email: blame.author_mail.clone().unwrap_or("".to_string()).into(),
            message: details,
        };

        let avatar = CommitAvatar::new(&commit_details).render(window, cx);

        let author = commit_details.author_name.clone();
        let author_email = commit_details.author_email.clone();

        let short_commit_id = commit_details
            .sha
            .get(0..8)
            .map(|sha| sha.to_string().into())
            .unwrap_or_else(|| commit_details.sha.clone());
        let full_sha = commit_details.sha.to_string().clone();
        let absolute_timestamp = format_local_timestamp(
            commit_details.commit_time,
            OffsetDateTime::now_utc(),
            time_format::TimestampFormat::MediumAbsolute,
        );
        let markdown_style = {
            let mut style = hover_markdown_style(window, cx);
            if let Some(code_block) = &style.code_block.text {
                style.base_text_style.refine(code_block);
            }
            style
        };

        let message = commit_details
            .message
            .as_ref()
            .map(|_| MarkdownElement::new(markdown.clone(), markdown_style).into_any())
            .unwrap_or("<no commit message>".into_any());

        let pull_request = commit_details
            .message
            .as_ref()
            .and_then(|details| details.pull_request.clone());

        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
        let message_max_height = window.line_height() * 12 + (ui_font_size / 0.4);
        let commit_summary = CommitSummary {
            sha: commit_details.sha.clone(),
            subject: commit_details
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
            commit_timestamp: commit_details.commit_time.unix_timestamp(),
            has_parent: false,
        };

        let ui_font = ThemeSettings::get_global(cx).ui_font.clone();

        // padding to avoid tooltip appearing right below the mouse cursor
        // TODO: use tooltip_container here
        Some(
            div()
                .pl_2()
                .pt_2p5()
                .child(
                    v_flex()
                        .elevation_2(cx)
                        .font(ui_font)
                        .text_ui(cx)
                        .text_color(cx.theme().colors().text)
                        .py_1()
                        .px_2()
                        .map(|el| {
                            el.occlude()
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
                                                            .text_color(
                                                                cx.theme().colors().text_muted,
                                                            )
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
                                                .track_scroll(&scroll_handle),
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
                                                            .on_click(move |_, window, cx| {
                                                                CommitView::open(
                                                                    commit_summary.clone(),
                                                                    repository.downgrade(),
                                                                    workspace.clone(),
                                                                    window,
                                                                    cx,
                                                                );
                                                                cx.stop_propagation();
                                                            }),
                                                        )
                                                        .child(
                                                            IconButton::new(
                                                                "copy-sha-button",
                                                                IconName::Copy,
                                                            )
                                                            .shape(IconButtonShape::Square)
                                                            .icon_size(IconSize::Small)
                                                            .icon_color(Color::Muted)
                                                            .on_click(move |_, _, cx| {
                                                                cx.stop_propagation();
                                                                cx.write_to_clipboard(
                                                                    ClipboardItem::new_string(
                                                                        full_sha.clone(),
                                                                    ),
                                                                )
                                                            }),
                                                        ),
                                                ),
                                        ),
                                )
                        }),
                )
                .into_any_element(),
        )
    }

    fn open_blame_commit(
        &self,
        blame_entry: BlameEntry,
        repository: Entity<Repository>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        CommitView::open(
            CommitSummary {
                sha: blame_entry.sha.to_string().into(),
                subject: blame_entry.summary.clone().unwrap_or_default().into(),
                commit_timestamp: blame_entry.committer_time.unwrap_or_default(),
                has_parent: true,
            },
            repository.downgrade(),
            workspace.clone(),
            window,
            cx,
        )
    }
}

fn deploy_blame_entry_context_menu(
    blame_entry: &BlameEntry,
    details: Option<&ParsedCommitMessage>,
    editor: Entity<Editor>,
    position: gpui::Point<Pixels>,
    window: &mut Window,
    cx: &mut App,
) {
    let context_menu = ContextMenu::build(window, cx, move |menu, _, _| {
        let sha = format!("{}", blame_entry.sha);
        menu.on_blur_subscription(Subscription::new(|| {}))
            .entry("Copy commit SHA", None, move |_, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(sha.clone()));
            })
            .when_some(
                details.and_then(|details| details.permalink.clone()),
                |this, url| {
                    this.entry("Open permalink", None, move |_, cx| {
                        cx.open_url(url.as_str())
                    })
                },
            )
    });

    editor.update(cx, move |editor, cx| {
        editor.deploy_mouse_context_menu(position, context_menu, window, cx);
        cx.notify();
    });
}

fn blame_entry_relative_timestamp(blame_entry: &BlameEntry) -> String {
    match blame_entry.author_offset_date_time() {
        Ok(timestamp) => {
            let local = chrono::Local::now().offset().local_minus_utc();
            time_format::format_localized_timestamp(
                timestamp,
                time::OffsetDateTime::now_utc(),
                time::UtcOffset::from_whole_seconds(local).unwrap(),
                time_format::TimestampFormat::Relative,
            )
        }
        Err(_) => "Error parsing date".to_string(),
    }
}
