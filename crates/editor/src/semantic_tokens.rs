use anyhow::Context as _;
use gpui::Entity;
use itertools::Itertools;
use multi_buffer::MultiBuffer;
use text::OffsetRangeExt;
use ui::{ActiveTheme, Context};

use crate::{Editor, RangeToAnchorExt};

pub fn fetch_and_update_semantic_tokens(
    editor: &Editor,
    multibuffer: Entity<MultiBuffer>,
    buffer: Entity<language::Buffer>,
    cx: &mut Context<Editor>,
) -> Option<()> {
    let fetch_task = editor
        .semantics_provider
        .as_ref()?
        .semantic_tokens(buffer.clone(), cx)?;
    cx.spawn(async move |editor, cx| -> anyhow::Result<()> {
        let tokens = fetch_task.await.context("semantic tokens fetch task")?;
        let tokens: Vec<_> = cx.update(|cx| {
            let multibuffer = multibuffer.read(cx).snapshot(cx);
            let snapshot = buffer.read(cx).snapshot();
            tokens
                .into_iter()
                .filter_map(|token| {
                    let is_valid = token.range.end.offset != 1
                        && token.range.end.offset != 0
                        && token.range.start.is_valid(&snapshot)
                        && token.range.end.is_valid(&snapshot);
                    if is_valid {
                        let range = token.range.to_point(&snapshot);
                        let range = range.to_anchors(&multibuffer);
                        Some((range, token.r#type, token.modifiers))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })?;
        editor.update(cx, |this: &mut Editor, cx| {
            this.clear_semantic_highlights(cx);
            for (range, r#type, modifiers) in tokens {
                let Some(mut style) = cx.theme().tokens().get(r#type.as_str()) else {
                    continue;
                };
                for r#mod in modifiers {
                    let r#mod = cx.theme().modifiers().get(r#mod.as_str());
                    style.highlight(match r#mod {
                        Some(value) => value,
                        None => continue,
                    });
                }
                this.semantic_highlight(range, style, cx);
            }
        })?;
        Ok(())
    })
    .detach_and_log_err(cx);
    Some(())
}
