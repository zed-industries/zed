use std::path::{Path, PathBuf};
use std::sync::Arc;

use collections::{HashMap, HashSet};
use editor::{
    Editor, EditorEvent,
    display_map::{BlockContext, BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
};
use gpui::{
    AnyElement, App, BorrowAppContext as _, Entity, Focusable as _, Global, Subscription,
    WeakEntity,
};
use language::{Bias, Point};
use ui::{KeyBinding, Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::{Workspace, notifications::NotifyTaskExt as _};

use super::{
    ReviewCopy, ReviewNotes, append_pending_comment, read_notes, refresh_review_views,
    show_message, write_notes,
};

const MAX_BODY_LINES: usize = 4;
const WRAP_COLUMNS: usize = 100;
/// Shares the `PrReviewModal > Editor` key bindings: cmd-enter adds, escape cancels.
const COMPOSER_KEY_CONTEXT: &str = "PrReviewModal";
const COMPOSER_MIN_LINES: usize = 2;
const COMPOSER_MAX_LINES: usize = 8;
/// The composer's label and footer rows around its editor.
const COMPOSER_CHROME_LINES: u32 = 3;

/// A review comment shown below its line. `author` is `None` for your own
/// pending comments that are not posted yet.
#[derive(Clone, Debug)]
struct InlineComment {
    line: u32,
    author: Option<String>,
    body: String,
    /// The text as saved, before clipping for display; identifies a pending
    /// comment when deleting it.
    full_body: String,
}

/// Comments per review copy directory, keyed by repository-relative path.
/// Pending comments are local and reload instantly; posted ones come from
/// GitHub, so they're kept separately and refreshed in the background.
#[derive(Default)]
struct ReviewCommentsStore {
    by_copy: HashMap<PathBuf, Arc<HashMap<String, Vec<InlineComment>>>>,
    posted: HashMap<PathBuf, Vec<(String, InlineComment)>>,
    pending: HashMap<PathBuf, Vec<(String, InlineComment)>>,
    loading: HashSet<PathBuf>,
}

impl ReviewCommentsStore {
    fn publish(&mut self, copy_dir: &Path) {
        let mut comments: HashMap<String, Vec<InlineComment>> = HashMap::default();
        for (path, comment) in self
            .posted
            .get(copy_dir)
            .into_iter()
            .chain(self.pending.get(copy_dir))
            .flatten()
        {
            comments
                .entry(path.clone())
                .or_default()
                .push(comment.clone());
        }
        self.by_copy
            .insert(copy_dir.to_path_buf(), Arc::new(comments));
    }
}

impl Global for ReviewCommentsStore {}

struct InlineCommentsAddon {
    copy_dir: PathBuf,
    block_ids: Vec<CustomBlockId>,
    composer: Option<OpenComposer>,
    _subscriptions: Vec<Subscription>,
}

struct OpenComposer {
    block_id: CustomBlockId,
    _composer: Entity<CommentComposer>,
}

impl editor::Addon for InlineCommentsAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}

pub(crate) fn init(cx: &mut App) {
    cx.set_global(ReviewCommentsStore::default());
    cx.observe_new(|editor: &mut Editor, _, cx| register_editor(editor, cx))
        .detach();
}

/// Reloads the posted and pending comments of the review copy this window shows.
pub(crate) fn refresh(workspace: &mut Workspace, cx: &mut Context<Workspace>) {
    if let Some(copy) = super::current_review_copy(workspace, cx) {
        load_comments(copy, cx);
    }
}

fn load_comments(copy: ReviewCopy, cx: &mut App) {
    if !cx.has_global::<ReviewCommentsStore>() {
        return;
    }

    let pending = cx.background_spawn({
        let copy = copy.clone();
        async move { read_pending(&copy).await }
    });
    cx.spawn({
        let copy_dir = copy.dir.clone();
        async move |cx| {
            let pending = pending.await;
            cx.update(|cx| {
                cx.update_global::<ReviewCommentsStore, _>(|store, _| {
                    store.pending.insert(copy_dir.clone(), pending);
                    store.publish(&copy_dir);
                });
            });
        }
    })
    .detach();

    let newly_loading = cx
        .update_global::<ReviewCommentsStore, _>(|store, _| store.loading.insert(copy.dir.clone()));
    if !newly_loading {
        return;
    }
    let posted = cx.background_spawn({
        let copy = copy.clone();
        async move { fetch_posted(&copy).await }
    });
    cx.spawn(async move |cx| {
        let posted = posted.await;
        cx.update(|cx| {
            cx.update_global::<ReviewCommentsStore, _>(|store, _| {
                store.loading.remove(&copy.dir);
                store.posted.insert(copy.dir.clone(), posted);
                store.publish(&copy.dir);
            });
        });
    })
    .detach();
}

async fn fetch_posted(copy: &ReviewCopy) -> Vec<(String, InlineComment)> {
    let posted = async {
        let repo = super::main_repository(&copy.dir).await?;
        super::load_line_comments(&repo, copy.pr_number).await
    }
    .await
    .log_err()
    .unwrap_or_default();
    posted
        .into_iter()
        .filter_map(|comment| {
            let line = comment.display_line()?;
            Some((
                comment.path,
                InlineComment {
                    line,
                    author: Some(comment.user.login),
                    full_body: comment.body.clone(),
                    body: comment.body,
                },
            ))
        })
        .collect()
}

async fn read_pending(copy: &ReviewCopy) -> Vec<(String, InlineComment)> {
    let notes = read_notes(copy).await.log_err().flatten();
    ReviewNotes::parse(&notes.unwrap_or_default())
        .comments
        .into_iter()
        .map(|pending| {
            (
                pending.path,
                InlineComment {
                    line: pending.line,
                    author: None,
                    full_body: pending.text.clone(),
                    body: pending.text,
                },
            )
        })
        .collect()
}

fn review_copy_for_editor(editor: &Editor, cx: &App) -> Option<ReviewCopy> {
    let project = editor.project()?;
    project
        .read(cx)
        .visible_worktrees(cx)
        .find_map(|worktree| ReviewCopy::for_worktree_root(&worktree.read(cx).abs_path()))
}

fn register_editor(editor: &mut Editor, cx: &mut Context<Editor>) {
    if !editor.mode().is_full() {
        return;
    }
    let Some(copy) = review_copy_for_editor(editor, cx) else {
        return;
    };

    let multibuffer = editor.buffer().clone();
    let subscriptions = vec![
        cx.observe_global::<ReviewCommentsStore>(update_blocks),
        cx.subscribe(&multibuffer, |editor, _, event, cx| {
            if matches!(
                event,
                multi_buffer::Event::BufferRangesUpdated { .. }
                    | multi_buffer::Event::BuffersRemoved { .. }
                    | multi_buffer::Event::FileHandleChanged
            ) {
                update_blocks(editor, cx);
            }
        }),
    ];
    editor.register_addon(InlineCommentsAddon {
        copy_dir: copy.dir.clone(),
        block_ids: Vec::new(),
        composer: None,
        _subscriptions: subscriptions,
    });

    let already_loaded = cx
        .try_global::<ReviewCommentsStore>()
        .is_some_and(|store| store.by_copy.contains_key(&copy.dir));
    if already_loaded {
        update_blocks(editor, cx);
    } else {
        load_comments(copy, cx);
    }
}

fn update_blocks(editor: &mut Editor, cx: &mut Context<Editor>) {
    let Some(addon) = editor.addon_mut::<InlineCommentsAddon>() else {
        return;
    };
    let copy_dir = addon.copy_dir.clone();
    let old_block_ids = std::mem::take(&mut addon.block_ids);
    if !old_block_ids.is_empty() {
        editor.remove_blocks(old_block_ids.into_iter().collect(), None, cx);
    }

    let Some(comments) = cx
        .try_global::<ReviewCommentsStore>()
        .and_then(|store| store.by_copy.get(&copy_dir).cloned())
    else {
        return;
    };
    if comments.is_empty() {
        return;
    }

    let multibuffer = editor.buffer().read(cx);
    let snapshot = multibuffer.snapshot(cx);
    let mut blocks = Vec::new();
    for buffer in multibuffer.all_buffers() {
        let buffer = buffer.read(cx);
        let Some(repo_path) = buffer
            .file()
            .and_then(|file| file.as_local())
            .and_then(|file| relative_unix_path(&file.abs_path(cx), &copy_dir))
        else {
            continue;
        };
        let Some(file_comments) = comments.get(&repo_path) else {
            continue;
        };
        let buffer_snapshot = buffer.snapshot();
        let max_row = buffer_snapshot.max_point().row;
        for comment in file_comments {
            let row = comment.line.saturating_sub(1);
            if comment.line == 0 || row > max_row {
                continue;
            }
            let end_of_line = Point::new(row, buffer_snapshot.line_len(row));
            let text_anchor = buffer_snapshot.anchor_before(end_of_line);
            let Some(anchor) = snapshot.anchor_in_excerpt(text_anchor) else {
                continue;
            };
            let (body, body_lines) = clip_body(&comment.body);
            let comment = InlineComment {
                body,
                ..comment.clone()
            };
            let delete_target = comment
                .author
                .is_none()
                .then(|| (copy_dir.clone(), repo_path.clone()));
            blocks.push(BlockProperties {
                placement: BlockPlacement::Below(anchor),
                height: Some(body_lines as u32 + 2),
                style: BlockStyle::Flex,
                render: Arc::new(move |cx| render_comment(&comment, delete_target.clone(), cx)),
                priority: 0,
            });
        }
    }

    let new_block_ids = editor.insert_blocks(blocks, None, cx);
    if let Some(addon) = editor.addon_mut::<InlineCommentsAddon>() {
        addon.block_ids = new_block_ids;
    }
}

/// Opens an inline box below the cursor's line in the active editor for writing
/// a pending review comment on `path:line`. Replaces any composer already open
/// in that editor.
pub(crate) fn open_composer(
    workspace: &mut Workspace,
    copy: ReviewCopy,
    path: String,
    line: u32,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(editor) = workspace.active_item_as::<Editor>(cx) else {
        return;
    };
    let workspace_handle = cx.weak_entity();
    let opened = editor.update(cx, |editor, cx| {
        if editor.addon::<InlineCommentsAddon>().is_none() {
            return false;
        }
        close_composer(editor, cx);

        let snapshot = editor.display_snapshot(cx);
        let head = editor.selections.newest::<Point>(&snapshot).head();
        let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
        let end_of_line = buffer_snapshot.clip_point(Point::new(head.row, u32::MAX), Bias::Left);
        let anchor = buffer_snapshot.anchor_before(end_of_line);

        let target_editor = cx.weak_entity();
        let composer = cx.new(|cx| {
            CommentComposer::new(
                copy,
                path,
                line,
                target_editor,
                workspace_handle,
                window,
                cx,
            )
        });
        let block_composer = composer.clone();
        let Some(block_id) = editor
            .insert_blocks(
                [BlockProperties {
                    placement: BlockPlacement::Below(anchor),
                    height: Some(COMPOSER_MIN_LINES as u32 + COMPOSER_CHROME_LINES),
                    style: BlockStyle::Flex,
                    render: Arc::new(move |cx| render_composer_block(&block_composer, cx)),
                    priority: 0,
                }],
                None,
                cx,
            )
            .into_iter()
            .next()
        else {
            return false;
        };
        composer.update(cx, |composer, _| composer.block_id = Some(block_id));
        if let Some(addon) = editor.addon_mut::<InlineCommentsAddon>() {
            addon.composer = Some(OpenComposer {
                block_id,
                _composer: composer.clone(),
            });
        }
        window.focus(&composer.read(cx).editor.focus_handle(cx), cx);
        true
    });
    if !opened {
        show_message(
            workspace,
            "Open one of this pull request's files to comment on it.",
            cx,
        );
    }
}

/// Removes the editor's open comment composer, if any.
fn close_composer(editor: &mut Editor, cx: &mut Context<Editor>) {
    let Some(composer) = editor
        .addon_mut::<InlineCommentsAddon>()
        .and_then(|addon| addon.composer.take())
    else {
        return;
    };
    editor.remove_blocks(HashSet::from_iter([composer.block_id]), None, cx);
}

/// The inline box for writing one pending review comment.
struct CommentComposer {
    editor: Entity<Editor>,
    copy: ReviewCopy,
    path: String,
    line: u32,
    target_editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
    block_id: Option<CustomBlockId>,
    saving: bool,
    _subscription: Subscription,
}

impl CommentComposer {
    fn new(
        copy: ReviewCopy,
        path: String,
        line: u32,
        target_editor: WeakEntity<Editor>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor =
                Editor::auto_height(COMPOSER_MIN_LINES, COMPOSER_MAX_LINES, window, cx);
            editor.set_placeholder_text("Leave a comment", window, cx);
            editor
        });
        let subscription = cx.subscribe(&editor, |this, editor, event, cx| {
            if matches!(event, EditorEvent::BufferEdited) {
                this.resize_block(&editor, cx);
            }
        });
        Self {
            editor,
            copy,
            path,
            line,
            target_editor,
            workspace,
            block_id: None,
            saving: false,
            _subscription: subscription,
        }
    }

    /// Grows the block with the text so the editor never scrolls inside it.
    fn resize_block(&self, editor: &Entity<Editor>, cx: &mut Context<Self>) {
        let Some(block_id) = self.block_id else {
            return;
        };
        let text_lines = editor.update(cx, |editor, cx| editor.max_point(cx).row().0 as usize + 1);
        let editor_lines = text_lines.clamp(COMPOSER_MIN_LINES, COMPOSER_MAX_LINES) as u32;
        self.target_editor
            .update(cx, |target_editor, cx| {
                target_editor.resize_blocks(
                    HashMap::from_iter([(block_id, editor_lines + COMPOSER_CHROME_LINES)]),
                    None,
                    cx,
                )
            })
            .log_err();
    }

    fn cancel(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        self.target_editor
            .update(cx, |target_editor, cx| {
                close_composer(target_editor, cx);
                window.focus(&target_editor.focus_handle(cx), cx);
            })
            .log_err();
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.editor.read(cx).text(cx).trim().to_string();
        if text.is_empty() || self.saving {
            return;
        }
        self.saving = true;
        cx.notify();

        let copy = self.copy.clone();
        let path = self.path.clone();
        let line = self.line;
        let target_editor = self.target_editor.clone();
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |this, cx| {
            let saved = cx
                .background_spawn(async move {
                    let existing = read_notes(&copy).await?;
                    let notes = append_pending_comment(existing.as_deref(), &path, line, &text);
                    write_notes(&copy, &notes).await
                })
                .await;
            if let Err(error) = saved {
                this.update(cx, |this, cx| {
                    this.saving = false;
                    cx.notify();
                })
                .log_err();
                return Err(error);
            }
            target_editor.update_in(cx, |target_editor, window, cx| {
                close_composer(target_editor, cx);
                window.focus(&target_editor.focus_handle(cx), cx);
            })?;
            workspace.update(cx, |workspace, cx| {
                refresh_review_views(workspace, cx);
                show_message(
                    workspace,
                    "Comment added. It posts when you submit the review.",
                    cx,
                );
            })?;
            anyhow::Ok(())
        })
        .detach_and_notify_err(self.workspace.clone(), window, cx);
    }
}

impl Render for CommentComposer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.editor.focus_handle(cx);
        let colors = cx.theme().colors();
        v_flex()
            .key_context(COMPOSER_KEY_CONTEXT)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .w_full()
            .pl_2()
            .gap_0p5()
            .border_l_2()
            .border_color(colors.text_accent)
            .bg(colors.editor_background)
            .child(
                Label::new(format!("Comment on line {}", self.line))
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
            .child(
                div()
                    .px_1p5()
                    .py_0p5()
                    .rounded_sm()
                    .border_1()
                    .border_color(colors.border_variant)
                    .child(self.editor.clone()),
            )
            .child(
                h_flex()
                    .gap_2()
                    .justify_between()
                    .child(
                        Label::new("⌘⏎ to add · Esc to cancel")
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(
                        h_flex()
                            .flex_none()
                            .gap_1()
                            .child(
                                Button::new("cancel-review-comment", "Cancel")
                                    .size(ButtonSize::Compact)
                                    .label_size(LabelSize::Small)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.cancel(&menu::Cancel, window, cx)
                                    })),
                            )
                            .child(
                                Button::new("add-review-comment", "Add Comment")
                                    .size(ButtonSize::Compact)
                                    .label_size(LabelSize::Small)
                                    .style(ButtonStyle::Filled)
                                    .disabled(self.saving)
                                    .key_binding(
                                        KeyBinding::for_action_in(
                                            &menu::Confirm,
                                            &focus_handle,
                                            cx,
                                        )
                                        .size(rems_from_px(11_f32)),
                                    )
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.confirm(&menu::Confirm, window, cx)
                                    })),
                            ),
                    ),
            )
    }
}

fn render_composer_block(composer: &Entity<CommentComposer>, cx: &mut BlockContext) -> AnyElement {
    h_flex()
        .id(cx.block_id)
        .w_full()
        .pl(cx.margins.gutter.width + cx.em_width * 2.)
        .pr_4()
        .py_0p5()
        .child(
            div()
                .w_full()
                .max_w(cx.max_width - cx.margins.gutter.width)
                .child(composer.clone()),
        )
        .into_any_element()
}

fn relative_unix_path(abs_path: &Path, copy_dir: &Path) -> Option<String> {
    let relative = abs_path.strip_prefix(copy_dir).ok()?;
    let parts = relative
        .components()
        .map(|component| component.as_os_str().to_str())
        .collect::<Option<Vec<_>>>()?;
    Some(parts.join("/"))
}

/// Keeps a comment to a few lines so the block height stays predictable.
fn clip_body(body: &str) -> (String, usize) {
    let mut lines = Vec::new();
    for source_line in body.trim().lines() {
        let source_line = source_line.trim_end();
        if source_line.is_empty() {
            continue;
        }
        let characters = source_line.chars().collect::<Vec<_>>();
        for chunk in characters.chunks(WRAP_COLUMNS) {
            lines.push(chunk.iter().collect::<String>());
        }
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    if lines.len() > MAX_BODY_LINES {
        lines.truncate(MAX_BODY_LINES);
        if let Some(last) = lines.last_mut() {
            last.push('…');
        }
    }
    let count = lines.len();
    (lines.join("\n"), count)
}

fn render_comment(
    comment: &InlineComment,
    delete_target: Option<(PathBuf, String)>,
    cx: &mut BlockContext,
) -> AnyElement {
    let colors = cx.theme().colors();
    let (border_color, header) = match &comment.author {
        Some(author) => (colors.border, format!("@{author}")),
        None => (colors.text_accent, "Pending".to_string()),
    };
    let header_color = if comment.author.is_some() {
        Color::Muted
    } else {
        Color::Accent
    };

    h_flex()
        .id(cx.block_id)
        .w_full()
        .pl(cx.margins.gutter.width + cx.em_width * 2.)
        .pr_4()
        .py_0p5()
        .child(
            v_flex()
                .w_full()
                .max_w(cx.max_width - cx.margins.gutter.width)
                .pl_2()
                .border_l_2()
                .border_color(border_color)
                .bg(colors.editor_background)
                .group("pr-review-inline-comment")
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            Label::new(header)
                                .size(LabelSize::XSmall)
                                .color(header_color),
                        )
                        .when_some(delete_target, |this, (copy_dir, path)| {
                            let line = comment.line;
                            let text = comment.full_body.clone();
                            this.child(
                                IconButton::new(
                                    ("delete-pending-comment", line as u64),
                                    IconName::Trash,
                                )
                                .icon_size(IconSize::XSmall)
                                .size(ButtonSize::Compact)
                                .visible_on_hover("pr-review-inline-comment")
                                .tooltip(Tooltip::text("Delete Pending Comment"))
                                .on_click(move |_, _, cx| {
                                    delete_pending_comment(
                                        copy_dir.clone(),
                                        path.clone(),
                                        line,
                                        text.clone(),
                                        cx,
                                    )
                                }),
                            )
                        }),
                )
                .children(comment.body.lines().map(|line| {
                    Label::new(line.to_string())
                        .size(LabelSize::Small)
                        .single_line()
                })),
        )
        .into_any_element()
}

fn delete_pending_comment(copy_dir: PathBuf, path: String, line: u32, text: String, cx: &mut App) {
    let Some(copy) = ReviewCopy::for_worktree_root(&copy_dir) else {
        return;
    };
    let write = cx.background_spawn({
        let copy = copy.clone();
        async move {
            let existing = read_notes(&copy).await?.unwrap_or_default();
            let notes = super::remove_pending_comment(&existing, &path, line, &text);
            write_notes(&copy, &notes).await
        }
    });
    cx.spawn(async move |cx| {
        write.await.log_err();
        cx.update(|cx| load_comments(copy, cx));
    })
    .detach();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_body() {
        assert_eq!(
            clip_body("  hello\n\nworld \n"),
            ("hello\nworld".to_string(), 2)
        );
        let long = "line\n".repeat(10);
        let (clipped, count) = clip_body(&long);
        assert_eq!(count, MAX_BODY_LINES);
        assert!(clipped.ends_with("line…"));
        let wide = "x".repeat(WRAP_COLUMNS + 5);
        assert_eq!(clip_body(&wide).1, 2);
    }

    #[test]
    fn test_relative_unix_path() {
        assert_eq!(
            relative_unix_path(
                Path::new("/cache/kip-pr-1/src/main.rs"),
                Path::new("/cache/kip-pr-1")
            ),
            Some("src/main.rs".to_string())
        );
        assert_eq!(
            relative_unix_path(
                Path::new("/elsewhere/main.rs"),
                Path::new("/cache/kip-pr-1")
            ),
            None
        );
    }
}
