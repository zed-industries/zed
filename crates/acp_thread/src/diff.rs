use agent_client_protocol as acp;
use anyhow::Result;
use buffer_diff::BufferDiff;
use editor::{MultiBuffer, PathKey};
use gpui::{App, AppContext, Context, Entity, Task};
use itertools::Itertools;
use language::{Anchor, Buffer, Capability, LanguageRegistry, OffsetRangeExt as _};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use util::ResultExt;

pub enum Diff {
    Pending {
        multibuffer: Entity<MultiBuffer>,
        base_text: Arc<String>,
        buffer: Entity<Buffer>,
        buffer_diff: Entity<BufferDiff>,
    },
    Ready {
        path: PathBuf,
        multibuffer: Entity<MultiBuffer>,
        _task: Task<Result<()>>,
    },
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

        Self::Ready {
            multibuffer,
            path,
            _task: task,
        }
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

        Self::Pending {
            multibuffer,
            base_text: Arc::new(base_text),
            buffer,
            buffer_diff,
        }
    }

    pub fn multibuffer(&self) -> &Entity<MultiBuffer> {
        match self {
            Self::Pending { multibuffer, .. } => multibuffer,
            Self::Ready { multibuffer, .. } => multibuffer,
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
            Diff::Pending { buffer, .. } => buffer.read(cx).file().map(|file| file.path().as_ref()),
            Diff::Ready { path, .. } => Some(path.as_path()),
        };
        format!(
            "Diff: {}\n```\n{}\n```\n",
            path.unwrap_or(Path::new("untitled")).display(),
            buffer_text
        )
    }
}
