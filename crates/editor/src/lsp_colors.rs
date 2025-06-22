use std::{cmp, ops::Range};

use collections::HashMap;
use futures::future::join_all;
use gpui::{Hsla, Rgba};
use language::point_from_lsp;
use lsp::LanguageServerId;
use multi_buffer::Anchor;
use project::DocumentColor;
use settings::Settings as _;
use text::{Bias, BufferId, OffsetRangeExt as _};
use ui::{App, Context, Window};
use util::post_inc;

use crate::{
    DisplayPoint, Editor, EditorSettings, EditorSnapshot, InlayId, InlaySplice, RangeToAnchorExt,
    display_map::Inlay, editor_settings::DocumentColorsRenderMode,
};

#[derive(Debug)]
pub(super) struct LspColorData {
    colors: Vec<(Range<Anchor>, DocumentColor, InlayId)>,
    inlay_colors: HashMap<InlayId, usize>,
    render_mode: DocumentColorsRenderMode,
}

impl LspColorData {
    pub fn new(cx: &App) -> Self {
        Self {
            colors: Vec::new(),
            inlay_colors: HashMap::default(),
            render_mode: EditorSettings::get_global(cx).lsp_document_colors,
        }
    }

    pub fn render_mode_updated(
        &mut self,
        new_render_mode: DocumentColorsRenderMode,
    ) -> Option<InlaySplice> {
        if self.render_mode == new_render_mode {
            return None;
        }
        self.render_mode = new_render_mode;
        match new_render_mode {
            DocumentColorsRenderMode::Inlay => Some(InlaySplice {
                to_remove: Vec::new(),
                to_insert: self
                    .colors
                    .iter()
                    .map(|(range, color, id)| {
                        Inlay::color(
                            id.id(),
                            range.start,
                            Rgba {
                                r: color.color.red,
                                g: color.color.green,
                                b: color.color.blue,
                                a: color.color.alpha,
                            },
                        )
                    })
                    .collect(),
            }),
            DocumentColorsRenderMode::None => {
                self.colors.clear();
                Some(InlaySplice {
                    to_remove: self.inlay_colors.drain().map(|(id, _)| id).collect(),
                    to_insert: Vec::new(),
                })
            }
            DocumentColorsRenderMode::Border | DocumentColorsRenderMode::Background => {
                Some(InlaySplice {
                    to_remove: self.inlay_colors.drain().map(|(id, _)| id).collect(),
                    to_insert: Vec::new(),
                })
            }
        }
    }

    fn set_colors(&mut self, colors: Vec<(Range<Anchor>, DocumentColor, InlayId)>) -> bool {
        if self.colors == colors {
            return false;
        }

        self.inlay_colors = colors
            .iter()
            .enumerate()
            .map(|(i, (_, _, id))| (*id, i))
            .collect();
        self.colors = colors;
        true
    }

    pub fn editor_display_highlights(
        &self,
        snapshot: &EditorSnapshot,
    ) -> (DocumentColorsRenderMode, Vec<(Range<DisplayPoint>, Hsla)>) {
        let render_mode = self.render_mode;
        let highlights = if render_mode == DocumentColorsRenderMode::None
            || render_mode == DocumentColorsRenderMode::Inlay
        {
            Vec::new()
        } else {
            self.colors
                .iter()
                .map(|(range, color, _)| {
                    let display_range = range.clone().to_display_points(snapshot);
                    let color = Hsla::from(Rgba {
                        r: color.color.red,
                        g: color.color.green,
                        b: color.color.blue,
                        a: color.color.alpha,
                    });
                    (display_range, color)
                })
                .collect()
        };
        (render_mode, highlights)
    }
}

impl Editor {
    pub(super) fn refresh_colors(
        &mut self,
        for_server_id: Option<LanguageServerId>,
        buffer_id: Option<BufferId>,
        _: &Window,
        cx: &mut Context<Self>,
    ) {
        if !self.mode().is_full() {
            return;
        }
        let Some(project) = self.project.clone() else {
            return;
        };
        if self
            .colors
            .as_ref()
            .is_none_or(|colors| colors.render_mode == DocumentColorsRenderMode::None)
        {
            return;
        }

        let all_colors_task = project.read(cx).lsp_store().update(cx, |lsp_store, cx| {
            self.buffer()
                .update(cx, |multi_buffer, cx| {
                    multi_buffer
                        .all_buffers()
                        .into_iter()
                        .filter(|editor_buffer| {
                            buffer_id.is_none_or(|buffer_id| {
                                buffer_id == editor_buffer.read(cx).remote_id()
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .into_iter()
                .filter_map(|buffer| {
                    let buffer_id = buffer.read(cx).remote_id();
                    let colors_task = lsp_store.document_colors(for_server_id, buffer, cx)?;
                    Some(async move { (buffer_id, colors_task.await) })
                })
                .collect::<Vec<_>>()
        });
        cx.spawn(async move |editor, cx| {
            let all_colors = join_all(all_colors_task).await;
            let Ok((multi_buffer_snapshot, editor_excerpts)) = editor.update(cx, |editor, cx| {
                let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                let editor_excerpts = multi_buffer_snapshot.excerpts().fold(
                    HashMap::default(),
                    |mut acc, (excerpt_id, buffer_snapshot, excerpt_range)| {
                        let excerpt_data = acc
                            .entry(buffer_snapshot.remote_id())
                            .or_insert_with(Vec::new);
                        let excerpt_point_range =
                            excerpt_range.context.to_point_utf16(&buffer_snapshot);
                        excerpt_data.push((
                            excerpt_id,
                            buffer_snapshot.clone(),
                            excerpt_point_range,
                        ));
                        acc
                    },
                );
                (multi_buffer_snapshot, editor_excerpts)
            }) else {
                return;
            };

            let mut new_editor_colors = Vec::<(Range<Anchor>, DocumentColor)>::new();
            for (buffer_id, colors) in all_colors {
                let Some(excerpts) = editor_excerpts.get(&buffer_id) else {
                    continue;
                };
                match colors {
                    Ok(colors) => {
                        for color in colors {
                            let color_start = point_from_lsp(color.lsp_range.start);
                            let color_end = point_from_lsp(color.lsp_range.end);

                            for (excerpt_id, buffer_snapshot, excerpt_range) in excerpts {
                                if !excerpt_range.contains(&color_start.0)
                                    || !excerpt_range.contains(&color_end.0)
                                {
                                    continue;
                                }
                                let Some(color_start_anchor) = multi_buffer_snapshot
                                    .anchor_in_excerpt(
                                        *excerpt_id,
                                        buffer_snapshot.anchor_before(
                                            buffer_snapshot
                                                .clip_point_utf16(color_start, Bias::Left),
                                        ),
                                    )
                                else {
                                    continue;
                                };
                                let Some(color_end_anchor) = multi_buffer_snapshot
                                    .anchor_in_excerpt(
                                        *excerpt_id,
                                        buffer_snapshot.anchor_after(
                                            buffer_snapshot
                                                .clip_point_utf16(color_end, Bias::Right),
                                        ),
                                    )
                                else {
                                    continue;
                                };

                                let (Ok(i) | Err(i)) =
                                    new_editor_colors.binary_search_by(|(probe, _)| {
                                        probe
                                            .start
                                            .cmp(&color_start_anchor, &multi_buffer_snapshot)
                                            .then_with(|| {
                                                probe
                                                    .end
                                                    .cmp(&color_end_anchor, &multi_buffer_snapshot)
                                            })
                                    });
                                new_editor_colors
                                    .insert(i, (color_start_anchor..color_end_anchor, color));
                                break;
                            }
                        }
                    }
                    Err(e) => log::error!("Failed to retrieve document colors: {e}"),
                }
            }

            editor
                .update(cx, |editor, cx| {
                    let mut colors_splice = InlaySplice::default();
                    let mut new_color_inlays = Vec::with_capacity(new_editor_colors.len());
                    let Some(colors) = &mut editor.colors else {
                        return;
                    };
                    let mut existing_colors = colors.colors.iter().peekable();
                    for (new_range, new_color) in new_editor_colors {
                        let rgba_color = Rgba {
                            r: new_color.color.red,
                            g: new_color.color.green,
                            b: new_color.color.blue,
                            a: new_color.color.alpha,
                        };

                        loop {
                            match existing_colors.peek() {
                                Some((existing_range, existing_color, existing_inlay_id)) => {
                                    match existing_range
                                        .start
                                        .cmp(&new_range.start, &multi_buffer_snapshot)
                                        .then_with(|| {
                                            existing_range
                                                .end
                                                .cmp(&new_range.end, &multi_buffer_snapshot)
                                        }) {
                                        cmp::Ordering::Less => {
                                            colors_splice.to_remove.push(*existing_inlay_id);
                                            existing_colors.next();
                                            continue;
                                        }
                                        cmp::Ordering::Equal => {
                                            if existing_color == &new_color {
                                                new_color_inlays.push((
                                                    new_range,
                                                    new_color,
                                                    *existing_inlay_id,
                                                ));
                                            } else {
                                                colors_splice.to_remove.push(*existing_inlay_id);

                                                let inlay = Inlay::color(
                                                    post_inc(&mut editor.next_color_inlay_id),
                                                    new_range.start,
                                                    rgba_color,
                                                );
                                                let inlay_id = inlay.id;
                                                colors_splice.to_insert.push(inlay);
                                                new_color_inlays
                                                    .push((new_range, new_color, inlay_id));
                                            }
                                            existing_colors.next();
                                            break;
                                        }
                                        cmp::Ordering::Greater => {
                                            let inlay = Inlay::color(
                                                post_inc(&mut editor.next_color_inlay_id),
                                                new_range.start,
                                                rgba_color,
                                            );
                                            let inlay_id = inlay.id;
                                            colors_splice.to_insert.push(inlay);
                                            new_color_inlays.push((new_range, new_color, inlay_id));
                                            break;
                                        }
                                    }
                                }
                                None => {
                                    let inlay = Inlay::color(
                                        post_inc(&mut editor.next_color_inlay_id),
                                        new_range.start,
                                        rgba_color,
                                    );
                                    let inlay_id = inlay.id;
                                    colors_splice.to_insert.push(inlay);
                                    new_color_inlays.push((new_range, new_color, inlay_id));
                                    break;
                                }
                            }
                        }
                    }
                    if existing_colors.peek().is_some() {
                        colors_splice
                            .to_remove
                            .extend(existing_colors.map(|(_, _, id)| *id));
                    }

                    let mut updated = colors.set_colors(new_color_inlays);
                    if colors.render_mode == DocumentColorsRenderMode::Inlay
                        && (!colors_splice.to_insert.is_empty()
                            || !colors_splice.to_remove.is_empty())
                    {
                        editor.splice_inlays(&colors_splice.to_remove, colors_splice.to_insert, cx);
                        updated = true;
                    }

                    if updated {
                        cx.notify();
                    }
                })
                .ok();
        })
        .detach();
    }
}
