use agent_client_protocol as acp;
use anyhow::Result;
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{MultiBuffer, PathKey};
use gpui::{App, AppContext, AsyncApp, Context, Entity, Subscription, Task};
use itertools::Itertools;
use language::{
    Anchor, Buffer, Capability, LanguageRegistry, OffsetRangeExt as _, Point, Rope, TextBuffer,
};
use std::{
    cmp::Reverse,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::ResultExt;

pub enum Diff {
    Pending(PendingDiff),
    Finalized(FinalizedDiff),
}

impl Diff {
    pub fn from_acp(
        diff: acp::Diff,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut Context<Self>,
    ) -> Self {
        let acp::Diff {
            path,
            old_text,
            new_text,
        } = diff;

        let multibuffer = cx.new(|_cx| MultiBuffer::without_headers(Capability::ReadOnly));

        let new_buffer = cx.new(|cx| Buffer::local(new_text, cx));
        let old_buffer = cx.new(|cx| Buffer::local(old_text.unwrap_or("".into()), cx));
        let new_buffer_snapshot = new_buffer.read(cx).text_snapshot();
        let buffer_diff = cx.new(|cx| BufferDiff::new(&new_buffer_snapshot, cx));

        let task = cx.spawn({
            let multibuffer = multibuffer.clone();
            let path = path.clone();
            async move |_, cx| {
                let language = language_registry
                    .language_for_file_path(&path)
                    .await
                    .log_err();

                new_buffer.update(cx, |buffer, cx| buffer.set_language(language.clone(), cx))?;

                let old_buffer_snapshot = old_buffer.update(cx, |buffer, cx| {
                    buffer.set_language(language, cx);
                    buffer.snapshot()
                })?;

                buffer_diff
                    .update(cx, |diff, cx| {
                        diff.set_base_text(
                            old_buffer_snapshot,
                            Some(language_registry),
                            new_buffer_snapshot,
                            cx,
                        )
                    })?
                    .await?;

                multibuffer
                    .update(cx, |multibuffer, cx| {
                        let hunk_ranges = {
                            let buffer = new_buffer.read(cx);
                            let diff = buffer_diff.read(cx);
                            diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &buffer, cx)
                                .map(|diff_hunk| diff_hunk.buffer_range.to_point(&buffer))
                                .collect::<Vec<_>>()
                        };

                        multibuffer.set_excerpts_for_path(
                            PathKey::for_buffer(&new_buffer, cx),
                            new_buffer.clone(),
                            hunk_ranges,
                            editor::DEFAULT_MULTIBUFFER_CONTEXT,
                            cx,
                        );
                        multibuffer.add_diff(buffer_diff, cx);
                    })
                    .log_err();

                anyhow::Ok(())
            }
        });

        Self::Finalized(FinalizedDiff {
            multibuffer,
            path,
            _update_diff: task,
        })
    }

    pub fn new(buffer: Entity<Buffer>, cx: &mut Context<Self>) -> Self {
        let buffer_snapshot = buffer.read(cx).snapshot();
        let base_text = buffer_snapshot.text();
        let language_registry = buffer.read(cx).language_registry();
        let text_snapshot = buffer.read(cx).text_snapshot();
        let buffer_diff = cx.new(|cx| {
            let mut diff = BufferDiff::new(&text_snapshot, cx);
            let _ = diff.set_base_text(
                buffer_snapshot.clone(),
                language_registry,
                text_snapshot,
                cx,
            );
            diff
        });

        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::without_headers(Capability::ReadOnly);
            multibuffer.add_diff(buffer_diff.clone(), cx);
            multibuffer
        });

        Self::Pending(PendingDiff {
            multibuffer,
            base_text: Arc::new(base_text),
            _subscription: cx.observe(&buffer, |this, _, cx| {
                if let Diff::Pending(diff) = this {
                    diff.update(cx);
                }
            }),
            buffer,
            diff: buffer_diff,
            revealed_ranges: Vec::new(),
            update_diff: Task::ready(Ok(())),
        })
    }

    pub fn reveal_range(&mut self, range: Range<Anchor>, cx: &mut Context<Self>) {
        if let Self::Pending(diff) = self {
            diff.reveal_range(range, cx);
        }
    }

    pub fn finalize(&mut self, cx: &mut Context<Self>) {
        if let Self::Pending(diff) = self {
            *self = Self::Finalized(diff.finalize(cx));
        }
    }

    pub fn multibuffer(&self) -> &Entity<MultiBuffer> {
        match self {
            Self::Pending(PendingDiff { multibuffer, .. }) => multibuffer,
            Self::Finalized(FinalizedDiff { multibuffer, .. }) => multibuffer,
        }
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        let buffer_text = self
            .multibuffer()
            .read(cx)
            .all_buffers()
            .iter()
            .map(|buffer| buffer.read(cx).text())
            .join("\n");
        let path = match self {
            Diff::Pending(PendingDiff { buffer, .. }) => {
                buffer.read(cx).file().map(|file| file.path().as_ref())
            }
            Diff::Finalized(FinalizedDiff { path, .. }) => Some(path.as_path()),
        };
        format!(
            "Diff: {}\n```\n{}\n```\n",
            path.unwrap_or(Path::new("untitled")).display(),
            buffer_text
        )
    }
}

pub struct PendingDiff {
    multibuffer: Entity<MultiBuffer>,
    base_text: Arc<String>,
    buffer: Entity<Buffer>,
    diff: Entity<BufferDiff>,
    revealed_ranges: Vec<Range<Anchor>>,
    _subscription: Subscription,
    update_diff: Task<Result<()>>,
}

impl PendingDiff {
    pub fn update(&mut self, cx: &mut Context<Diff>) {
        let buffer = self.buffer.clone();
        let buffer_diff = self.diff.clone();
        let base_text = self.base_text.clone();
        self.update_diff = cx.spawn(async move |diff, cx| {
            let text_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot())?;
            let diff_snapshot = BufferDiff::update_diff(
                buffer_diff.clone(),
                text_snapshot.clone(),
                Some(base_text),
                false,
                false,
                None,
                None,
                cx,
            )
            .await?;
            buffer_diff.update(cx, |diff, cx| {
                diff.set_snapshot(diff_snapshot, &text_snapshot, cx)
            })?;
            diff.update(cx, |diff, cx| {
                if let Diff::Pending(diff) = diff {
                    diff.update_visible_ranges(cx);
                }
            })
        });
    }

    pub fn reveal_range(&mut self, range: Range<Anchor>, cx: &mut Context<Diff>) {
        self.revealed_ranges.push(range);
        self.update_visible_ranges(cx);
    }

    fn finalize(&self, cx: &mut Context<Diff>) -> FinalizedDiff {
        let ranges = self.excerpt_ranges(cx);
        let base_text = self.base_text.clone();
        let language_registry = self.buffer.read(cx).language_registry().clone();

        let path = self
            .buffer
            .read(cx)
            .file()
            .map(|file| file.path().as_ref())
            .unwrap_or(Path::new("untitled"))
            .into();

        // Replace the buffer in the multibuffer with the snapshot
        let buffer = cx.new(|cx| {
            let language = self.buffer.read(cx).language().cloned();
            let buffer = TextBuffer::new_normalized(
                0,
                cx.entity_id().as_non_zero_u64().into(),
                self.buffer.read(cx).line_ending(),
                self.buffer.read(cx).as_rope().clone(),
            );
            let mut buffer = Buffer::build(buffer, None, Capability::ReadWrite);
            buffer.set_language(language, cx);
            buffer
        });

        let buffer_diff = cx.spawn({
            let buffer = buffer.clone();
            let language_registry = language_registry.clone();
            async move |_this, cx| {
                build_buffer_diff(base_text, &buffer, language_registry, cx).await
            }
        });

        let update_diff = cx.spawn(async move |this, cx| {
            let buffer_diff = buffer_diff.await?;
            this.update(cx, |this, cx| {
                this.multibuffer().update(cx, |multibuffer, cx| {
                    let path_key = PathKey::for_buffer(&buffer, cx);
                    multibuffer.clear(cx);
                    multibuffer.set_excerpts_for_path(
                        path_key,
                        buffer,
                        ranges,
                        editor::DEFAULT_MULTIBUFFER_CONTEXT,
                        cx,
                    );
                    multibuffer.add_diff(buffer_diff.clone(), cx);
                });

                cx.notify();
            })
        });

        FinalizedDiff {
            path,
            multibuffer: self.multibuffer.clone(),
            _update_diff: update_diff,
        }
    }

    fn update_visible_ranges(&mut self, cx: &mut Context<Diff>) {
        let ranges = self.excerpt_ranges(cx);
        self.multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.set_excerpts_for_path(
                PathKey::for_buffer(&self.buffer, cx),
                self.buffer.clone(),
                ranges,
                editor::DEFAULT_MULTIBUFFER_CONTEXT,
                cx,
            );
            let end = multibuffer.len(cx);
            Some(multibuffer.snapshot(cx).offset_to_point(end).row + 1)
        });
        cx.notify();
    }

    fn excerpt_ranges(&self, cx: &App) -> Vec<Range<Point>> {
        let buffer = self.buffer.read(cx);
        let diff = self.diff.read(cx);
        let mut ranges = diff
            .hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &buffer, cx)
            .map(|diff_hunk| diff_hunk.buffer_range.to_point(&buffer))
            .collect::<Vec<_>>();
        ranges.extend(
            self.revealed_ranges
                .iter()
                .map(|range| range.to_point(&buffer)),
        );
        ranges.sort_unstable_by_key(|range| (range.start, Reverse(range.end)));

        // Merge adjacent ranges
        let mut ranges = ranges.into_iter().peekable();
        let mut merged_ranges = Vec::new();
        while let Some(mut range) = ranges.next() {
            while let Some(next_range) = ranges.peek() {
                if range.end >= next_range.start {
                    range.end = range.end.max(next_range.end);
                    ranges.next();
                } else {
                    break;
                }
            }

            merged_ranges.push(range);
        }
        merged_ranges
    }
}

pub struct FinalizedDiff {
    path: PathBuf,
    multibuffer: Entity<MultiBuffer>,
    _update_diff: Task<Result<()>>,
}

async fn build_buffer_diff(
    old_text: Arc<String>,
    buffer: &Entity<Buffer>,
    language_registry: Option<Arc<LanguageRegistry>>,
    cx: &mut AsyncApp,
) -> Result<Entity<BufferDiff>> {
    let buffer = cx.update(|cx| buffer.read(cx).snapshot())?;

    let old_text_rope = cx
        .background_spawn({
            let old_text = old_text.clone();
            async move { Rope::from(old_text.as_str()) }
        })
        .await;
    let base_buffer = cx
        .update(|cx| {
            Buffer::build_snapshot(
                old_text_rope,
                buffer.language().cloned(),
                language_registry,
                cx,
            )
        })?
        .await;

    let diff_snapshot = cx
        .update(|cx| {
            BufferDiffSnapshot::new_with_base_buffer(
                buffer.text.clone(),
                Some(old_text),
                base_buffer,
                cx,
            )
        })?
        .await;

    let secondary_diff = cx.new(|cx| {
        let mut diff = BufferDiff::new(&buffer, cx);
        diff.set_snapshot(diff_snapshot.clone(), &buffer, cx);
        diff
    })?;

    cx.new(|cx| {
        let mut diff = BufferDiff::new(&buffer.text, cx);
        diff.set_snapshot(diff_snapshot, &buffer, cx);
        diff.set_secondary_diff(secondary_diff);
        diff
    })
}
