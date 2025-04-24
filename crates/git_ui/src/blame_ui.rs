use crate::{commit_tooltip::CommitTooltip, commit_view::CommitView};
use editor::{BlameRenderer, Editor};
use git::{
    blame::{BlameEntry, ParsedCommitMessage},
    repository::CommitSummary,
};
use gpui::{
    AnyElement, App, AppContext as _, ClipboardItem, Element as _, Entity, Hsla,
    InteractiveElement as _, MouseButton, Pixels, StatefulInteractiveElement as _, Styled as _,
    Subscription, TextStyle, WeakEntity, Window, div,
};
use project::{git_store::Repository, project_settings::ProjectSettings};
use settings::Settings as _;
use ui::{
    ActiveTheme, Color, ContextMenu, FluentBuilder as _, Icon, IconName, IntoElement,
    ParentElement as _, h_flex,
};
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
        blame_entry: BlameEntry,
        details: Option<ParsedCommitMessage>,
        repository: Entity<Repository>,
        workspace: WeakEntity<Workspace>,
        cx: &mut App,
    ) -> Option<AnyElement> {
        Some(
            cx.new(|cx| {
                CommitTooltip::blame_entry(
                    &blame_entry,
                    details.clone(),
                    repository.clone(),
                    workspace.clone(),
                    cx,
                )
            })
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
