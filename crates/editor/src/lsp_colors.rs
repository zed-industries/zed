use std::{cmp, ops::Range};

use collections::HashMap;
use futures::future::join_all;
use gpui::{Hsla, Rgba, Task};
use itertools::Itertools;
use language::point_from_lsp;
use multi_buffer::Anchor;
use project::{DocumentColor, InlayId};
use settings::Settings as _;
use text::{Bias, BufferId, OffsetRangeExt as _};
use ui::{App, Context, Window};
use util::post_inc;

use crate::{
    DisplayPoint, Editor, EditorSettings, EditorSnapshot, FETCH_COLORS_DEBOUNCE_TIMEOUT,
    InlaySplice, RangeToAnchorExt, editor_settings::DocumentColorsRenderMode, inlays::Inlay,
};

#[derive(Debug)]
pub(super) struct LspColorData {
    buffer_colors: HashMap<BufferId, BufferColors>,
    render_mode: DocumentColorsRenderMode,
}

#[derive(Debug, Default)]
struct BufferColors {
    colors: Vec<(Range<Anchor>, DocumentColor, InlayId)>,
    inlay_colors: HashMap<InlayId, usize>,
    cache_version_used: usize,
}

impl LspColorData {
    pub fn new(cx: &App) -> Self {
        Self {
            buffer_colors: HashMap::default(),
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
                    .buffer_colors
                    .iter()
                    .flat_map(|(_, buffer_colors)| buffer_colors.colors.iter())
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
            DocumentColorsRenderMode::None => Some(InlaySplice {
                to_remove: self
                    .buffer_colors
                    .drain()
                    .flat_map(|(_, buffer_colors)| buffer_colors.inlay_colors)
                    .map(|(id, _)| id)
                    .collect(),
                to_insert: Vec::new(),
            }),
            DocumentColorsRenderMode::Border | DocumentColorsRenderMode::Background => {
                Some(InlaySplice {
                    to_remove: self
                        .buffer_colors
                        .iter_mut()
                        .flat_map(|(_, buffer_colors)| buffer_colors.inlay_colors.drain())
                        .map(|(id, _)| id)
                        .collect(),
                    to_insert: Vec::new(),
                })
            }
        }
    }

    fn set_colors(
        &mut self,
        buffer_id: BufferId,
        colors: Vec<(Range<Anchor>, DocumentColor, InlayId)>,
        cache_version: Option<usize>,
    ) -> bool {
        let buffer_colors = self.buffer_colors.entry(buffer_id).or_default();
        if let Some(cache_version) = cache_version {
            buffer_colors.cache_version_used = cache_version;
        }
        if buffer_colors.colors == colors {
            return false;
        }

        buffer_colors.inlay_colors = colors
            .iter()
            .enumerate()
            .map(|(i, (_, _, id))| (*id, i))
            .collect();
        buffer_colors.colors = colors;
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
            self.buffer_colors
                .iter()
                .flat_map(|(_, buffer_colors)| &buffer_colors.colors)
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
    pub(super) fn refresh_colors_for_visible_range(
        &mut self,
        buffer_id: Option<BufferId>,
        _: &Window,
        cx: &mut Context<Self>,
    ) {
        if self.ignore_lsp_data() {
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

        let visible_buffers = self
            .visible_excerpts(true, cx)
            .into_values()
            .map(|(buffer, ..)| buffer)
            .filter(|editor_buffer| {
                let editor_buffer_id = editor_buffer.read(cx).remote_id();
                buffer_id.is_none_or(|buffer_id| buffer_id == editor_buffer_id)
                    && self.registered_buffers.contains_key(&editor_buffer_id)
            })
            .unique_by(|buffer| buffer.read(cx).remote_id())
            .collect::<Vec<_>>();

        let all_colors_task = project.read(cx).lsp_store().update(cx, |lsp_store, cx| {
            visible_buffers
                .into_iter()
                .filter_map(|buffer| {
                    let buffer_id = buffer.read(cx).remote_id();
                    let known_cache_version = self.colors.as_ref().and_then(|colors| {
                        Some(colors.buffer_colors.get(&buffer_id)?.cache_version_used)
                    });
                    let colors_task = lsp_store.document_colors(known_cache_version, buffer, cx)?;
                    Some(async move { (buffer_id, colors_task.await) })
                })
                .collect::<Vec<_>>()
        });

        if all_colors_task.is_empty() {
            self.refresh_colors_task = Task::ready(());
            return;
        }

        self.refresh_colors_task = cx.spawn(async move |editor, cx| {
            cx.background_executor()
                .timer(FETCH_COLORS_DEBOUNCE_TIMEOUT)
                .await;

            let all_colors = join_all(all_colors_task).await;
            if all_colors.is_empty() {
                return;
            }
            let Ok((multi_buffer_snapshot, editor_excerpts)) = editor.update(cx, |editor, cx| {
                let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                let editor_excerpts = multi_buffer_snapshot.excerpts().fold(
                    HashMap::default(),
                    |mut acc, (excerpt_id, buffer_snapshot, excerpt_range)| {
                        let excerpt_data = acc
                            .entry(buffer_snapshot.remote_id())
                            .or_insert_with(Vec::new);
                        let excerpt_point_range =
                            excerpt_range.context.to_point_utf16(buffer_snapshot);
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

            let mut new_editor_colors = HashMap::default();
            for (buffer_id, colors) in all_colors {
                let Some(excerpts) = editor_excerpts.get(&buffer_id) else {
                    continue;
                };
                match colors {
                    Ok(colors) => {
                        if colors.colors.is_empty() {
                            let new_entry =
                                new_editor_colors.entry(buffer_id).or_insert_with(|| {
                                    (Vec::<(Range<Anchor>, DocumentColor)>::new(), None)
                                });
                            new_entry.0.clear();
                            new_entry.1 = colors.cache_version;
                        } else {
                            for color in colors.colors {
                                let color_start = point_from_lsp(color.lsp_range.start);
                                let color_end = point_from_lsp(color.lsp_range.end);

                                for (excerpt_id, buffer_snapshot, excerpt_range) in excerpts {
                                    if !excerpt_range.contains(&color_start.0)
                                        || !excerpt_range.contains(&color_end.0)
                                    {
                                        continue;
                                    }
                                    let start = buffer_snapshot.anchor_before(
                                        buffer_snapshot.clip_point_utf16(color_start, Bias::Left),
                                    );
                                    let end = buffer_snapshot.anchor_after(
                                        buffer_snapshot.clip_point_utf16(color_end, Bias::Right),
                                    );
                                    let Some(range) = multi_buffer_snapshot
                                        .anchor_range_in_excerpt(*excerpt_id, start..end)
                                    else {
                                        continue;
                                    };

                                    let new_entry =
                                        new_editor_colors.entry(buffer_id).or_insert_with(|| {
                                            (Vec::<(Range<Anchor>, DocumentColor)>::new(), None)
                                        });
                                    new_entry.1 = colors.cache_version;
                                    let new_buffer_colors = &mut new_entry.0;

                                    let (Ok(i) | Err(i)) =
                                        new_buffer_colors.binary_search_by(|(probe, _)| {
                                            probe
                                                .start
                                                .cmp(&range.start, &multi_buffer_snapshot)
                                                .then_with(|| {
                                                    probe
                                                        .end
                                                        .cmp(&range.end, &multi_buffer_snapshot)
                                                })
                                        });
                                    new_buffer_colors.insert(i, (range, color));
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => log::error!("Failed to retrieve document colors: {e}"),
                }
            }

            editor
                .update(cx, |editor, cx| {
                    let mut colors_splice = InlaySplice::default();
                    let Some(colors) = &mut editor.colors else {
                        return;
                    };
                    let mut updated = false;
                    for (buffer_id, (new_buffer_colors, new_cache_version)) in new_editor_colors {
                        let mut new_buffer_color_inlays =
                            Vec::with_capacity(new_buffer_colors.len());
                        let mut existing_buffer_colors = colors
                            .buffer_colors
                            .entry(buffer_id)
                            .or_default()
                            .colors
                            .iter()
                            .peekable();
                        for (new_range, new_color) in new_buffer_colors {
                            let rgba_color = Rgba {
                                r: new_color.color.red,
                                g: new_color.color.green,
                                b: new_color.color.blue,
                                a: new_color.color.alpha,
                            };

                            loop {
                                match existing_buffer_colors.peek() {
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
                                                existing_buffer_colors.next();
                                                continue;
                                            }
                                            cmp::Ordering::Equal => {
                                                if existing_color == &new_color {
                                                    new_buffer_color_inlays.push((
                                                        new_range,
                                                        new_color,
                                                        *existing_inlay_id,
                                                    ));
                                                } else {
                                                    colors_splice
                                                        .to_remove
                                                        .push(*existing_inlay_id);

                                                    let inlay = Inlay::color(
                                                        post_inc(&mut editor.next_color_inlay_id),
                                                        new_range.start,
                                                        rgba_color,
                                                    );
                                                    let inlay_id = inlay.id;
                                                    colors_splice.to_insert.push(inlay);
                                                    new_buffer_color_inlays
                                                        .push((new_range, new_color, inlay_id));
                                                }
                                                existing_buffer_colors.next();
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
                                                new_buffer_color_inlays
                                                    .push((new_range, new_color, inlay_id));
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
                                        new_buffer_color_inlays
                                            .push((new_range, new_color, inlay_id));
                                        break;
                                    }
                                }
                            }
                        }

                        if existing_buffer_colors.peek().is_some() {
                            colors_splice
                                .to_remove
                                .extend(existing_buffer_colors.map(|(_, _, id)| *id));
                        }
                        updated |= colors.set_colors(
                            buffer_id,
                            new_buffer_color_inlays,
                            new_cache_version,
                        );
                    }

                    if colors.render_mode == DocumentColorsRenderMode::Inlay
                        && !colors_splice.is_empty()
                    {
                        editor.splice_inlays(&colors_splice.to_remove, colors_splice.to_insert, cx);
                        updated = true;
                    }

                    if updated {
                        cx.notify();
                    }
                })
                .ok();
        });
    }
}
