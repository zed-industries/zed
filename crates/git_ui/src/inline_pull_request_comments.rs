use std::sync::Arc;

use collections::HashSet;
use editor::{
    Editor,
    display_map::{BlockContext, BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
    hover_markdown_style,
};
use gpui::{AnyElement, Context, Entity, Subscription};
use markdown::{Markdown, MarkdownElement};
use multi_buffer::{Event as MultiBufferEvent, MultiBufferPoint};
use project::{ProjectPath, project_settings::ProjectSettings};
use settings::{Settings as _, SettingsStore};
use time::{OffsetDateTime, UtcOffset};
use ui::prelude::*;

use crate::pull_request_store::PullRequestStore;

struct PullRequestCommentsAddon {
    store: Entity<PullRequestStore>,
    block_ids: Vec<CustomBlockId>,
    _subscriptions: Vec<Subscription>,
}

impl editor::Addon for PullRequestCommentsAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}

/// Attaches inline pull request comment blocks to a newly created editor.
pub fn register_editor(editor: &mut Editor, cx: &mut Context<Editor>) {
    if !editor.mode().is_full() {
        return;
    }
    let Some(project) = editor.project().cloned() else {
        return;
    };

    let store = PullRequestStore::for_project(&project, cx);

    let mut subscriptions =
        vec![cx.observe(&store, |editor, _store, cx| refresh_blocks(editor, cx))];
    let multibuffer = editor.buffer().clone();
    subscriptions.push(
        cx.subscribe(&multibuffer, |editor, _multibuffer, event, cx| {
            if matches!(
                event,
                MultiBufferEvent::BufferRangesUpdated { .. }
                    | MultiBufferEvent::BuffersRemoved { .. }
                    | MultiBufferEvent::DiffHunksToggled
            ) {
                refresh_blocks(editor, cx);
            }
        }),
    );
    subscriptions.push(cx.observe_global::<SettingsStore>(|editor, cx| {
        refresh_blocks(editor, cx);
    }));

    editor.register_addon(PullRequestCommentsAddon {
        store,
        block_ids: Vec::new(),
        _subscriptions: subscriptions,
    });
    refresh_blocks(editor, cx);
}

fn refresh_blocks(editor: &mut Editor, cx: &mut Context<Editor>) {
    let Some(addon) = editor.addon::<PullRequestCommentsAddon>() else {
        return;
    };
    let store = addon.store.clone();

    let old_block_ids: HashSet<CustomBlockId> = editor
        .addon_mut::<PullRequestCommentsAddon>()
        .unwrap()
        .block_ids
        .drain(..)
        .collect();
    if !old_block_ids.is_empty() {
        editor.remove_blocks(old_block_ids, None, cx);
    }

    if !ProjectSettings::get_global(cx)
        .git
        .pull_request
        .enable_inline_comments
    {
        return;
    }

    let Some(repository) = store.read(cx).active_repository().cloned() else {
        return;
    };

    let now = OffsetDateTime::now_utc();
    let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);

    let snapshot = editor.buffer().read(cx).snapshot(cx);
    let buffers = editor.buffer().read(cx).all_buffers();
    let mut placements = Vec::new();
    for buffer in buffers {
        let buffer = buffer.read(cx);
        let Some(file) = buffer.file() else {
            continue;
        };
        let project_path = ProjectPath::from_file(file.as_ref(), cx);
        let Some(repo_path) = repository
            .read(cx)
            .project_path_to_repo_path(&project_path, cx)
        else {
            continue;
        };
        let repo_path = repo_path.as_unix_str().to_string();

        let comments = store.read(cx).comments_for_file(&repo_path);
        if comments.is_empty() {
            continue;
        }

        let buffer_snapshot = buffer.snapshot();
        for comment in comments {
            let Some(line) = comment.line else {
                continue;
            };
            let row = line.saturating_sub(1);
            let text_anchor = buffer_snapshot.anchor_after(MultiBufferPoint::new(row, 0));
            let Some(anchor) = snapshot.anchor_in_excerpt(text_anchor) else {
                continue;
            };

            let timestamp = time_format::format_localized_timestamp(
                comment.created_at,
                now,
                local_offset,
                time_format::TimestampFormat::Relative,
            );
            placements.push((
                anchor,
                comment.author_name.clone(),
                timestamp,
                comment.body.clone(),
            ));
        }
    }

    if placements.is_empty() {
        return;
    }

    let mut blocks = Vec::new();
    for (anchor, author, timestamp, body) in placements {
        let height = (body.lines().count() as u32 + 1).clamp(2, 10);
        let body = cx.new(|cx| Markdown::new(body.into(), None, None, cx));
        blocks.push(BlockProperties {
            placement: BlockPlacement::Below(anchor),
            height: Some(height),
            style: BlockStyle::Sticky,
            render: Arc::new(move |cx| render_comment(&author, &timestamp, &body, cx)),
            priority: 0,
        });
    }

    let new_block_ids = editor.insert_blocks(blocks, None, cx);
    editor
        .addon_mut::<PullRequestCommentsAddon>()
        .unwrap()
        .block_ids = new_block_ids;
}

fn render_comment(
    author: &str,
    timestamp: &str,
    body: &Entity<Markdown>,
    cx: &mut BlockContext,
) -> AnyElement {
    let markdown_style = hover_markdown_style(cx.window, cx.app);

    v_flex()
        .id(cx.block_id)
        .ml(cx.margins.gutter.full_width())
        .mr(cx.margins.right)
        .pl_2()
        .border_l_2()
        .border_color(cx.theme().colors().border)
        .bg(cx.theme().colors().background)
        .child(
            h_flex()
                .gap_1p5()
                .child(Label::new(author.to_string()).size(LabelSize::Small))
                .child(
                    Label::new(timestamp.to_string())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
        .child(MarkdownElement::new(body.clone(), markdown_style))
        .into_any_element()
}
