use crate::{
    commit_tooltip::{CommitAvatar, CommitTooltip, commit_tag_chips, shallow_boundary_notice},
    commit_view::{CommitView, GitBlob, build_buffer, worktree_id_for_repo_path},
};
use anyhow::Context as _;
use editor::{BlameRenderer, Editor, GitBlame, MultiBuffer, hover_markdown_style};
use git::{
    Oid,
    blame::BlameEntry,
    commit::ParsedCommitMessage,
    repository::{CommitSummary, RepoPath},
};
use gpui::{
    ClipboardItem, Entity, Hsla, MouseButton, Pixels, Rems, ScrollHandle, Subscription, TextStyle,
    TextStyleRefinement, UnderlineStyle, WeakEntity, prelude::*,
};
use markdown::{Markdown, MarkdownElement};
use project::{
    git_store::Repository,
    project_settings::{InlineBlameLocation, ProjectSettings},
};
use settings::Settings as _;
use std::sync::Arc;
use theme_settings::ThemeSettings;
use time::OffsetDateTime;
use ui::{ContextMenu, CopyButton, Divider, prelude::*, tooltip_container};
use util::paths::PathStyle;
use workspace::{Workspace, notifications::NotifyTaskExt as _};

const GIT_BLAME_MAX_AUTHOR_CHARS_DISPLAYED: usize = 20;
const GIT_BLAME_GUTTER_MARGIN: Rems = rems(0.5);
const GIT_BLAME_GUTTER_GAP: Rems = rems(0.5);
const GIT_BLAME_AVATAR_SIZE: Rems = rems(1.);

pub struct GitBlameRenderer;

fn format_blame_text(blame_entry: &BlameEntry, cx: &App) -> String {
    let relative_timestamp = blame_entry_relative_timestamp(blame_entry);
    let author = blame_entry.author.as_deref().unwrap_or_default();
    let summary_enabled = ProjectSettings::get_global(cx)
        .git
        .inline_blame
        .show_commit_summary;

    match blame_entry.summary.as_ref() {
        Some(summary) if summary_enabled => {
            format!("{author}, {relative_timestamp} - {summary}")
        }
        _ => format!("{author}, {relative_timestamp}"),
    }
}

#[derive(Default)]
pub struct GitBlameStatus {
    text: Option<SharedString>,
    active_editor: Option<Entity<Editor>>,
    _subscriptions: Vec<Subscription>,
}

impl GitBlameStatus {
    fn update(&mut self, editor: Entity<Editor>, _window: &mut Window, cx: &mut Context<Self>) {
        let inline_blame = ProjectSettings::get_global(cx).git.inline_blame;
        let text =
            if inline_blame.enabled && inline_blame.location == InlineBlameLocation::StatusBar {
                editor
                    .update(cx, |editor, cx| editor.active_git_blame_entry(cx))
                    .map(|blame_entry| SharedString::from(format_blame_text(&blame_entry, cx)))
            } else {
                None
            };

        if text != self.text {
            self.text = text;
            cx.notify();
        }
    }
}

impl Render for GitBlameStatus {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let inline_blame = ProjectSettings::get_global(cx).git.inline_blame;
        if !inline_blame.enabled || inline_blame.location != InlineBlameLocation::StatusBar {
            return div();
        }

        div().when_some(self.text.clone(), |el, text| {
            el.child(
                Button::new("git-blame-status", text.clone())
                    .label_size(LabelSize::Small)
                    .start_icon(
                        Icon::new(IconName::FileGit)
                            .size(IconSize::Small)
                            .color(Color::Hint),
                    )
                    .on_click(cx.listener(|this, _, window, cx| {
                        if let Some(editor) = this.active_editor.clone() {
                            let focus_handle = gpui::Focusable::focus_handle(editor.read(cx), cx);
                            focus_handle.dispatch_action(
                                &editor::actions::OpenGitBlameCommit,
                                window,
                                cx,
                            );
                        }
                    }))
                    .tooltip(ui::Tooltip::text(text)),
            )
        })
    }
}

impl workspace::StatusItemView for GitBlameStatus {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn workspace::item::ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            self.active_editor = Some(editor.clone());
            self._subscriptions = vec![cx.observe_in(&editor, window, Self::update)];
            self.update(editor, window, cx);
        } else {
            self.text = None;
            self.active_editor = None;
            self._subscriptions.clear();
            cx.notify();
        }
    }

    fn hide_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        None
    }
}

impl BlameRenderer for GitBlameRenderer {
    fn max_author_length(&self) -> usize {
        GIT_BLAME_MAX_AUTHOR_CHARS_DISPLAYED
    }

    fn blame_entry_non_text_width(&self, window: &Window, cx: &App) -> Pixels {
        let show_avatar = ProjectSettings::get_global(cx).git.blame.show_avatar;
        let gap_count = if show_avatar { 3. } else { 2. };
        let width = GIT_BLAME_GUTTER_MARGIN.to_pixels(window.rem_size())
            + GIT_BLAME_GUTTER_GAP.to_pixels(window.rem_size()) * gap_count;

        if show_avatar {
            width + CommitAvatar::rendered_size(GIT_BLAME_AVATAR_SIZE, window)
        } else {
            width
        }
    }

    fn render_blame_entry(
        &self,
        style: &TextStyle,
        blame_entry: BlameEntry,
        details: Option<ParsedCommitMessage>,
        tag_names: Vec<SharedString>,
        repository: Entity<Repository>,
        workspace: WeakEntity<Workspace>,
        editor: Entity<Editor>,
        ix: usize,
        sha_color: Hsla,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let relative_timestamp = blame_entry_relative_timestamp(&blame_entry);
        let short_commit_id = blame_entry.sha.display_short();
        let author_name = blame_entry.author.as_deref().unwrap_or("<no name>");
        let name = util::truncate_and_trailoff(author_name, GIT_BLAME_MAX_AUTHOR_CHARS_DISPLAYED);
        let is_highlighted = editor
            .read(cx)
            .blame()
            .is_some_and(|blame| blame.read(cx).highlighted_sha() == Some(blame_entry.sha));

        let avatar = if ProjectSettings::get_global(cx).git.blame.show_avatar {
            let author_email = blame_entry.author_mail.as_ref().map(|email| {
                SharedString::from(
                    email
                        .trim_start_matches('<')
                        .trim_end_matches('>')
                        .to_string(),
                )
            });
            Some(
                CommitAvatar::new(
                    &blame_entry.sha.to_string().into(),
                    author_email,
                    details.as_ref().and_then(|it| it.remote.as_ref()),
                )
                .size(GIT_BLAME_AVATAR_SIZE)
                .render(window, cx),
            )
        } else {
            None
        };

        Some(
            div()
                .mr_2()
                .child(
                    h_flex()
                        .id(("blame", ix))
                        .w_full()
                        .gap_2()
                        .justify_between()
                        .font(style.font())
                        .line_height(style.line_height)
                        .text_color(cx.theme().status().hint)
                        .when(is_highlighted, |this| {
                            this.bg(cx.theme().colors().element_selected)
                        })
                        .child(
                            h_flex()
                                .gap_2()
                                .child(div().text_color(sha_color).child(short_commit_id))
                                .children(avatar)
                                .child(name),
                        )
                        .child(relative_timestamp)
                        .hover(|style| style.bg(cx.theme().colors().element_hover))
                        .cursor_pointer()
                        .on_mouse_down(MouseButton::Right, {
                            let blame_entry = blame_entry.clone();
                            let details = details.clone();
                            let editor = editor.clone();
                            let repository = repository.clone();
                            let workspace = workspace.clone();
                            move |event, window, cx| {
                                cx.stop_propagation();

                                deploy_blame_entry_context_menu(
                                    &blame_entry,
                                    details.as_ref(),
                                    editor.clone(),
                                    repository.clone(),
                                    workspace.clone(),
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
                                    blame_entry.sha.to_string(),
                                    repository.downgrade(),
                                    workspace.clone(),
                                    None,
                                    None,
                                    window,
                                    cx,
                                )
                            }
                        })
                        .when(!editor.read(cx).has_mouse_context_menu(), |el| {
                            el.hoverable_tooltip(move |_window, cx| {
                                cx.new(|cx| {
                                    CommitTooltip::blame_entry(
                                        &blame_entry,
                                        details.clone(),
                                        tag_names.clone(),
                                        repository.clone(),
                                        workspace.clone(),
                                        cx,
                                    )
                                })
                                .into()
                            })
                        }),
                )
                .into_any(),
        )
    }

    fn render_inline_blame_entry(
        &self,
        style: &TextStyle,
        blame_entry: BlameEntry,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let text = format_blame_text(&blame_entry, cx);

        Some(
            h_flex()
                .id("inline-blame")
                .w_full()
                .font(style.font())
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
        tag_names: Vec<SharedString>,
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

        let sha = blame.sha.to_string().into();
        let author: SharedString = blame
            .author
            .clone()
            .unwrap_or("<no name>".to_string())
            .into();
        let author_email = blame.author_mail.as_deref().unwrap_or_default();
        let author_email_for_avatar = blame.author_mail.as_ref().map(|email| {
            SharedString::from(
                email
                    .trim_start_matches('<')
                    .trim_end_matches('>')
                    .to_string(),
            )
        });
        let avatar = CommitAvatar::new(
            &sha,
            author_email_for_avatar,
            details.as_ref().and_then(|it| it.remote.as_ref()),
        )
        .render(window, cx);

        let short_commit_id = sha
            .get(..git::SHORT_SHA_LENGTH)
            .map(|sha| sha.to_string().into())
            .unwrap_or_else(|| sha.clone());
        let local_offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
        let absolute_timestamp = time_format::format_localized_timestamp(
            commit_time,
            OffsetDateTime::now_utc(),
            local_offset,
            time_format::TimestampFormat::MediumAbsolute,
        );
        let link_color = cx.theme().colors().text_accent;
        let markdown_style = {
            let mut style = hover_markdown_style(window, cx);
            style.link.refine(&TextStyleRefinement {
                color: Some(link_color),
                underline: Some(UnderlineStyle {
                    color: Some(link_color.opacity(0.4)),
                    thickness: px(1.0),
                    ..Default::default()
                }),
                ..Default::default()
            });
            style
        };

        let message = details
            .as_ref()
            .map(|_| {
                MarkdownElement::new(markdown.clone(), markdown_style)
                    .scroll_handle(scroll_handle.clone())
                    .into_any()
            })
            .unwrap_or("<no commit message>".into_any());

        let pull_request = details
            .as_ref()
            .and_then(|details| details.pull_request.clone());

        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
        let message_max_height = window.line_height() * 12 + (ui_font_size / 0.4);
        let commit_summary = CommitSummary {
            sha: sha.clone(),
            subject: details
                .as_ref()
                .and_then(|details| {
                    Some(
                        details
                            .message
                            .split('\n')
                            .next()?
                            .trim_end()
                            .to_string()
                            .into(),
                    )
                })
                .unwrap_or_default(),
            commit_timestamp: commit_time.unix_timestamp(),
            author_name: author.clone(),
            has_parent: false,
        };
        let boundary_notice = blame
            .boundary
            .then(|| shallow_boundary_notice(repository.clone(), workspace.clone(), window, cx))
            .flatten();

        Some(
            tooltip_container(cx, |this, cx| {
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
                                    .border_b_1()
                                    .border_color(cx.theme().colors().border_variant)
                                    .child(avatar)
                                    .child(author)
                                    .when(!author_email.is_empty(), |this| {
                                        this.child(
                                            div()
                                                .text_color(cx.theme().colors().text_muted)
                                                .child(author_email.to_owned()),
                                        )
                                    }),
                            )
                            .children(boundary_notice)
                            .child(
                                div()
                                    .id("inline-blame-commit-message")
                                    .track_scroll(&scroll_handle)
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
                                                .on_click(move |_, window, cx| {
                                                    CommitView::open(
                                                        commit_summary.sha.clone().into(),
                                                        repository.downgrade(),
                                                        workspace.clone(),
                                                        None,
                                                        None,
                                                        window,
                                                        cx,
                                                    );
                                                    cx.stop_propagation();
                                                }),
                                            )
                                            .child(Divider::vertical())
                                            .child(
                                                CopyButton::new("copy-blame-sha", sha.to_string())
                                                    .tooltip_label("Copy SHA"),
                                            ),
                                    ),
                            ),
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
            blame_entry.sha.to_string(),
            repository.downgrade(),
            workspace,
            None,
            None,
            window,
            cx,
        )
    }

    fn open_blame_revision(
        &self,
        path: RepoPath,
        revision: Oid,
        repository: Entity<Repository>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        open_buffer_blame_at_revision(repository, workspace, path, revision, window, cx);
    }
}

fn deploy_blame_entry_context_menu(
    blame_entry: &BlameEntry,
    details: Option<&ParsedCommitMessage>,
    editor: Entity<Editor>,
    repository: Entity<Repository>,
    workspace: WeakEntity<Workspace>,
    position: gpui::Point<Pixels>,
    window: &mut Window,
    cx: &mut App,
) {
    let highlighted_sha = editor
        .read(cx)
        .blame()
        .and_then(|blame| blame.read(cx).highlighted_sha());
    let context_menu = ContextMenu::build(window, cx, move |menu, _, _| {
        let sha = format!("{}", blame_entry.sha);
        let blame_revision = blame_entry.revision_target(highlighted_sha);
        let blame_previous_revision = blame_entry.previous_revision_target();
        let has_blame_targets = blame_revision.is_some() || blame_previous_revision.is_some();
        menu.on_blur_subscription(Subscription::new(|| {}))
            .entry("Copy Commit SHA", None, move |_, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(sha.clone()));
            })
            .when_some(
                details.and_then(|details| details.permalink.clone()),
                |this, url| {
                    this.entry("Open Commit Permalink", None, move |_, cx| {
                        cx.open_url(url.as_str())
                    })
                },
            )
            .when(has_blame_targets, |this| this.separator())
            .when_some(blame_revision, |this, (revision, path)| {
                let repository = repository.clone();
                let workspace = workspace.clone();
                this.entry("Blame Revision", None, move |window, cx| {
                    open_buffer_blame_at_revision(
                        repository.clone(),
                        workspace.clone(),
                        path.clone(),
                        revision,
                        window,
                        cx,
                    );
                })
            })
            .when_some(blame_previous_revision, |this, (revision, path)| {
                let repository = repository.clone();
                let workspace = workspace.clone();
                this.entry("Blame Previous Revision", None, move |window, cx| {
                    open_buffer_blame_at_revision(
                        repository.clone(),
                        workspace.clone(),
                        path.clone(),
                        revision,
                        window,
                        cx,
                    );
                })
            })
    });

    editor.update(cx, move |editor, cx| {
        editor.hide_blame_popover(false, cx);
        editor.deploy_mouse_context_menu(position, context_menu, window, cx);
        cx.notify();
    });
}

fn open_buffer_blame_at_revision(
    repository: Entity<Repository>,
    workspace: WeakEntity<Workspace>,
    path: RepoPath,
    revision: Oid,
    window: &mut Window,
    cx: &mut App,
) {
    window
        .spawn(cx, {
            let workspace = workspace.clone();
            async move |cx| {
                let (language_registry, worktree_id) =
                    workspace.read_with(cx, |workspace, cx| {
                        let project = workspace.project().read(cx);
                        (
                            project.languages().clone(),
                            worktree_id_for_repo_path(repository.read(cx), project, &path, cx),
                        )
                    })?;
                let worktree_id = worktree_id.context("project has no worktrees")?;

                let file_name = path
                    .file_name()
                    .map(|name| name.to_string())
                    .unwrap_or_else(|| path.display(PathStyle::local()).to_string());
                let display_name = format!("{file_name} @ {}", revision.display_short());

                let activated_existing = workspace.update_in(cx, |workspace, window, cx| {
                    activate_existing_blame_editor(
                        workspace,
                        &repository,
                        &path,
                        revision,
                        window,
                        cx,
                    )
                })?;
                if activated_existing {
                    return Ok(());
                }

                let (content, blame) = repository
                    .update(cx, |repository, cx| {
                        repository.blame_buffer_at_revision(path.clone(), revision, cx)
                    })
                    .await?;

                let file = Arc::new(GitBlob {
                    path: path.clone(),
                    worktree_id,
                    is_deleted: false,
                    is_binary: false,
                    display_name: display_name.clone(),
                }) as Arc<dyn language::File>;

                let buffer = build_buffer(content, file, &language_registry, cx).await?;

                workspace.update_in(cx, |workspace, window, cx| {
                    if activate_existing_blame_editor(
                        workspace,
                        &repository,
                        &path,
                        revision,
                        window,
                        cx,
                    ) {
                        return;
                    }

                    let project = workspace.project().clone();
                    let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));
                    let editor = cx.new(|cx| {
                        let mut editor = Editor::for_multibuffer(
                            multi_buffer.clone(),
                            Some(project.clone()),
                            window,
                            cx,
                        );
                        editor.set_read_only(true);
                        editor.set_should_serialize(false, cx);
                        editor
                    });
                    let git_blame = cx.new(|cx| {
                        GitBlame::new_static(
                            multi_buffer,
                            project,
                            repository,
                            [(buffer, blame)],
                            Some(revision),
                            cx,
                        )
                    });
                    editor.update(cx, |editor, cx| editor.set_blame(git_blame, window, cx));
                    workspace.add_item_to_active_pane(Box::new(editor), None, true, window, cx);
                })
            }
        })
        .detach_and_notify_err(workspace, window, cx);
}

fn activate_existing_blame_editor(
    workspace: &mut Workspace,
    repository: &Entity<Repository>,
    path: &RepoPath,
    revision: Oid,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> bool {
    match existing_blame_editor(workspace, repository, path, revision, cx) {
        Some(existing) => {
            workspace.activate_item(&existing, true, true, window, cx);
            true
        }
        None => false,
    }
}

fn existing_blame_editor(
    workspace: &Workspace,
    repository: &Entity<Repository>,
    path: &RepoPath,
    revision: Oid,
    cx: &App,
) -> Option<Entity<Editor>> {
    workspace
        .panes()
        .iter()
        .flat_map(|pane| pane.read(cx).items())
        .find_map(|item| {
            let editor = item.downcast::<Editor>()?;
            let blame = editor.read(cx).blame()?;
            if blame.read(cx).highlighted_sha() != Some(revision) {
                return None;
            }
            let buffer = editor.read(cx).buffer().read(cx).as_singleton()?;
            let buffer_id = buffer.read(cx).remote_id();
            if blame.read(cx).repository(cx, buffer_id).as_ref() != Some(repository) {
                return None;
            }
            let file = buffer.read(cx).file()?;
            let blob = (file.as_ref() as &dyn std::any::Any).downcast_ref::<GitBlob>()?;
            (blob.path == *path).then_some(editor)
        })
}

fn blame_entry_relative_timestamp(blame_entry: &BlameEntry) -> String {
    match blame_entry.author_offset_date_time() {
        Ok(timestamp) => {
            let local_offset =
                time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
            time_format::format_localized_timestamp(
                timestamp,
                time::OffsetDateTime::now_utc(),
                local_offset,
                time_format::TimestampFormat::Relative,
            )
        }
        Err(_) => "Error parsing date".to_string(),
    }
}
