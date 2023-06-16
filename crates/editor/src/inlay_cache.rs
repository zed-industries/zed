use std::ops::Range;

use crate::{
    display_map::InlayId, editor_settings, scroll::ScrollAnchor, Anchor, Editor, ExcerptId,
    MultiBuffer,
};
use clock::Global;
use gpui::{ModelHandle, Task, ViewContext};
use language::Buffer;
use project::{InlayHint, InlayHintKind};

use collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
pub enum InlayRefreshReason {
    SettingsChange(editor_settings::InlayHints),
    Scroll(ScrollAnchor),
    VisibleExcerptsChange,
}

#[derive(Debug, Clone, Default)]
pub struct InlayCache {
    inlay_hints: HashMap<InlayId, InlayHint>,
    inlays_in_buffers: HashMap<u64, BufferInlays>,
    allowed_hint_kinds: HashSet<Option<InlayHintKind>>,
}

#[derive(Clone, Debug, Default)]
struct BufferInlays {
    buffer_version: Global,
    ordered_by_anchor_inlays: Vec<(Anchor, InlayId)>,
}

#[derive(Debug, Default)]
pub struct InlaySplice {
    pub to_remove: Vec<InlayId>,
    pub to_insert: Vec<(Option<InlayId>, Anchor, InlayHint)>,
}

pub struct InlayHintQuery {
    pub buffer_id: u64,
    pub buffer_version: Global,
    pub excerpt_id: ExcerptId,
    pub excerpt_offset_query_range: Range<usize>,
}

impl InlayCache {
    pub fn new(inlay_hint_settings: editor_settings::InlayHints) -> Self {
        Self {
            allowed_hint_kinds: allowed_inlay_hint_types(inlay_hint_settings),
            inlays_in_buffers: HashMap::default(),
            inlay_hints: HashMap::default(),
        }
    }

    pub fn apply_settings(
        &mut self,
        multi_buffer: ModelHandle<MultiBuffer>,
        inlay_hint_settings: editor_settings::InlayHints,
        currently_visible_ranges: Vec<(ModelHandle<Buffer>, Range<usize>, ExcerptId)>,
        currently_shown_inlays: Vec<(Anchor, InlayId)>,
        cx: &mut ViewContext<Editor>,
    ) -> Option<InlaySplice> {
        let new_allowed_hint_kinds = allowed_inlay_hint_types(inlay_hint_settings);
        if new_allowed_hint_kinds == self.allowed_hint_kinds {
            None
        } else {
            self.allowed_hint_kinds = new_allowed_hint_kinds;
            let mut to_remove = Vec::new();
            let mut to_insert = Vec::new();

            let mut considered_inlay_ids = HashSet::default();
            for (_, shown_inlay_id) in currently_shown_inlays {
                if let Some(inlay_hint) = self.inlay_hints.get(&shown_inlay_id) {
                    if !self.allowed_hint_kinds.contains(&inlay_hint.kind) {
                        to_remove.push(shown_inlay_id);
                    }
                    considered_inlay_ids.insert(shown_inlay_id);
                }
            }

            let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
            for (inlay_id, inlay_hint) in &self.inlay_hints {
                if self.allowed_hint_kinds.contains(&inlay_hint.kind)
                    && !considered_inlay_ids.contains(inlay_id)
                {
                    if let Some(hint_to_readd) = currently_visible_ranges.iter()
                        .filter(|(_, visible_range, _)| visible_range.contains(&inlay_hint.position.offset))
                        .find_map(|(_, _, excerpt_id)| {
                            let Some(anchor) = multi_buffer_snapshot
                                .find_anchor_in_excerpt(*excerpt_id, inlay_hint.position) else { return None; };
                            Some((Some(*inlay_id), anchor, inlay_hint.clone()))
                        },
                    ) {
                        to_insert.push(hint_to_readd);
                    }
                }
            }

            Some(InlaySplice {
                to_remove,
                to_insert,
            })
        }
    }

    pub fn clear(&mut self) -> Vec<InlayId> {
        let ids_to_remove = self.inlay_hints.drain().map(|(id, _)| id).collect();
        self.inlays_in_buffers.clear();
        ids_to_remove
    }

    pub fn append_inlays(
        &mut self,
        multi_buffer: ModelHandle<MultiBuffer>,
        ranges_to_add: impl Iterator<Item = InlayHintQuery>,
        currently_shown_inlays: Vec<(Anchor, InlayId)>,
        cx: &mut ViewContext<Editor>,
    ) -> Task<anyhow::Result<InlaySplice>> {
        self.fetch_inlays(
            multi_buffer,
            ranges_to_add,
            currently_shown_inlays,
            false,
            cx,
        )
    }

    pub fn replace_inlays(
        &mut self,
        multi_buffer: ModelHandle<MultiBuffer>,
        new_ranges: impl Iterator<Item = InlayHintQuery>,
        currently_shown_inlays: Vec<(Anchor, InlayId)>,
        cx: &mut ViewContext<Editor>,
    ) -> Task<anyhow::Result<InlaySplice>> {
        self.fetch_inlays(multi_buffer, new_ranges, currently_shown_inlays, true, cx)
    }

    fn fetch_inlays(
        &mut self,
        multi_buffer: ModelHandle<MultiBuffer>,
        inlay_fetch_ranges: impl Iterator<Item = InlayHintQuery>,
        currently_shown_inlays: Vec<(Anchor, InlayId)>,
        replace_old: bool,
        cx: &mut ViewContext<Editor>,
    ) -> Task<anyhow::Result<InlaySplice>> {
        // TODO kb
        todo!("TODO kb")
    }

    // fn fetch_inlays(
    //     &mut self,
    //     multi_buffer: ModelHandle<MultiBuffer>,
    //     inlay_fetch_ranges: impl Iterator<Item = InlayHintQuery>,
    //     replace_old: bool,
    //     cx: &mut ViewContext<Editor>,
    // ) -> Task<anyhow::Result<InlaySplice>> {
    //     let mut inlay_fetch_tasks = Vec::new();
    //     for inlay_fetch_range in inlay_fetch_ranges {
    //         let inlays_up_to_date = self.inlays_up_to_date(
    //             &inlay_fetch_range.buffer_path,
    //             &inlay_fetch_range.buffer_version,
    //             inlay_fetch_range.excerpt_id,
    //         );
    //         let task_multi_buffer = multi_buffer.clone();
    //         let task = cx.spawn(|editor, mut cx| async move {
    //             if inlays_up_to_date {
    //                 anyhow::Ok((inlay_fetch_range, None))
    //             } else {
    //                 let Some(buffer_handle) = cx.read(|cx| task_multi_buffer.read(cx).buffer(inlay_fetch_range.buffer_id))
    //                     else { return Ok((inlay_fetch_range, Some(Vec::new()))) };
    //                 let task = editor
    //                     .update(&mut cx, |editor, cx| {
    //                         editor.project.as_ref().map(|project| {
    //                             project.update(cx, |project, cx| {
    //                                 project.query_inlay_hints_for_buffer(
    //                                     buffer_handle,
    //                                     inlay_fetch_range.excerpt_offset_query_range.clone(),
    //                                     cx,
    //                                 )
    //                             })
    //                         })
    //                     })
    //                     .context("inlays fecth task spawn")?;

    //                 Ok((inlay_fetch_range, match task {
    //                     Some(task) => task.await.context("inlays for buffer task")?,
    //                     None => Some(Vec::new()),
    //                 }))
    //             }
    //         });
    //         inlay_fetch_tasks.push(task);
    //     }

    //     let final_task = cx.spawn(|editor, mut cx| async move {
    //         let mut inlay_updates: HashMap<
    //             PathBuf,
    //             (
    //                 Global,
    //                 HashMap<ExcerptId, Option<(Range<usize>, OrderedByAnchorOffset<InlayHint>)>>,
    //             ),
    //         > = HashMap::default();
    //         let multi_buffer_snapshot =
    //             editor.read_with(&cx, |editor, cx| editor.buffer().read(cx).snapshot(cx))?;

    //         for task_result in futures::future::join_all(inlay_fetch_tasks).await {
    //             match task_result {
    //                 Ok((inlay_fetch_range, response_inlays)) => {
    //                     // TODO kb different caching now
    //                     let inlays_per_excerpt = HashMap::from_iter([(
    //                         inlay_fetch_range.excerpt_id,
    //                         response_inlays
    //                             .map(|excerpt_inlays| {
    //                                 excerpt_inlays.into_iter().fold(
    //                                     OrderedByAnchorOffset::default(),
    //                                     |mut ordered_inlays, inlay| {
    //                                         let anchor = multi_buffer_snapshot.anchor_in_excerpt(
    //                                             inlay_fetch_range.excerpt_id,
    //                                             inlay.position,
    //                                         );
    //                                         ordered_inlays.add(anchor, inlay);
    //                                         ordered_inlays
    //                                     },
    //                                 )
    //                             })
    //                             .map(|inlays| {
    //                                 (inlay_fetch_range.excerpt_offset_query_range, inlays)
    //                             }),
    //                     )]);
    //                     match inlay_updates.entry(inlay_fetch_range.buffer_path) {
    //                         hash_map::Entry::Occupied(mut o) => {
    //                             o.get_mut().1.extend(inlays_per_excerpt);
    //                         }
    //                         hash_map::Entry::Vacant(v) => {
    //                             v.insert((inlay_fetch_range.buffer_version, inlays_per_excerpt));
    //                         }
    //                     }
    //                 }
    //                 Err(e) => error!("Failed to update inlays for buffer: {e:#}"),
    //             }
    //         }

    //         let updates = if !inlay_updates.is_empty() {
    //             let inlays_update = editor.update(&mut cx, |editor, _| {
    //                 editor.inlay_cache.apply_fetch_inlays(inlay_updates)
    //             })?;
    //             inlays_update
    //         } else {
    //             InlaySplice::default()
    //         };

    //         anyhow::Ok(updates)
    //     });

    //     final_task
    // }

    // fn inlays_up_to_date(
    //     &self,
    //     buffer_path: &Path,
    //     buffer_version: &Global,
    //     excerpt_id: ExcerptId,
    // ) -> bool {
    //     let Some(buffer_inlays) = self.inlays_per_buffer.get(buffer_path) else { return false };
    //     let buffer_up_to_date = buffer_version == &buffer_inlays.buffer_version
    //         || buffer_inlays.buffer_version.changed_since(&buffer_version);
    //     buffer_up_to_date && buffer_inlays.inlays_per_excerpts.contains_key(&excerpt_id)
    // }

    // fn apply_fetch_inlays(
    //     &mut self,
    //     fetched_inlays: HashMap<
    //         PathBuf,
    //         (
    //             Global,
    //             HashMap<ExcerptId, Option<(Range<usize>, OrderedByAnchorOffset<InlayHint>)>>,
    //         ),
    //     >,
    // ) -> InlaySplice {
    //     let mut old_inlays = self.inlays_per_buffer.clone();
    //     let mut to_remove = Vec::new();
    //     let mut to_insert = Vec::new();

    //     for (buffer_path, (buffer_version, new_buffer_inlays)) in fetched_inlays {
    //         match old_inlays.remove(&buffer_path) {
    //             Some(mut old_buffer_inlays) => {
    //                 for (excerpt_id, new_excerpt_inlays) in new_buffer_inlays {
    //                     let (_, mut new_excerpt_inlays) = match new_excerpt_inlays {
    //                         Some((excerpt_offset_range, new_inlays)) => (
    //                             excerpt_offset_range,
    //                             new_inlays.into_ordered_elements().fuse().peekable(),
    //                         ),
    //                         None => continue,
    //                     };
    //                     if self.inlays_up_to_date(&buffer_path, &buffer_version, excerpt_id) {
    //                         continue;
    //                     }

    //                     let self_inlays_per_buffer = self
    //                         .inlays_per_buffer
    //                         .get_mut(&buffer_path)
    //                         .expect("element expected: `old_inlays.remove` returned `Some`");

    //                     if old_buffer_inlays
    //                         .inlays_per_excerpts
    //                         .remove(&excerpt_id)
    //                         .is_some()
    //                     {
    //                         let self_excerpt_inlays = self_inlays_per_buffer
    //                             .inlays_per_excerpts
    //                             .get_mut(&excerpt_id)
    //                             .expect("element expected: `old_excerpt_inlays` is `Some`");
    //                         let mut hints_to_add = Vec::<(Anchor, (InlayId, InlayHint))>::new();
    //                         // TODO kb update inner buffer_id and version with the new data?
    //                         self_excerpt_inlays.0.retain(
    //                             |_, (old_anchor, (old_inlay_id, old_inlay))| {
    //                                 let mut retain = false;

    //                                 while let Some(new_offset) = new_excerpt_inlays
    //                                     .peek()
    //                                     .map(|(new_anchor, _)| new_anchor.text_anchor.offset)
    //                                 {
    //                                     let old_offset = old_anchor.text_anchor.offset;
    //                                     match new_offset.cmp(&old_offset) {
    //                                         cmp::Ordering::Less => {
    //                                             let (new_anchor, new_inlay) =
    //                                                 new_excerpt_inlays.next().expect(
    //                                                     "element expected: `peek` returned `Some`",
    //                                                 );
    //                                             hints_to_add.push((
    //                                                 new_anchor,
    //                                                 (
    //                                                     InlayId(post_inc(&mut self.next_inlay_id)),
    //                                                     new_inlay,
    //                                                 ),
    //                                             ));
    //                                         }
    //                                         cmp::Ordering::Equal => {
    //                                             let (new_anchor, new_inlay) =
    //                                                 new_excerpt_inlays.next().expect(
    //                                                     "element expected: `peek` returned `Some`",
    //                                                 );
    //                                             if &new_inlay == old_inlay {
    //                                                 retain = true;
    //                                             } else {
    //                                                 hints_to_add.push((
    //                                                     new_anchor,
    //                                                     (
    //                                                         InlayId(post_inc(
    //                                                             &mut self.next_inlay_id,
    //                                                         )),
    //                                                         new_inlay,
    //                                                     ),
    //                                                 ));
    //                                             }
    //                                         }
    //                                         cmp::Ordering::Greater => break,
    //                                     }
    //                                 }

    //                                 if !retain {
    //                                     to_remove.push(*old_inlay_id);
    //                                 }
    //                                 retain
    //                             },
    //                         );

    //                         for (new_anchor, (id, new_inlay)) in hints_to_add {
    //                             self_excerpt_inlays.add(new_anchor, (id, new_inlay.clone()));
    //                             to_insert.push((id, new_anchor, new_inlay));
    //                         }
    //                     }

    //                     for (new_anchor, new_inlay) in new_excerpt_inlays {
    //                         let id = InlayId(post_inc(&mut self.next_inlay_id));
    //                         self_inlays_per_buffer
    //                             .inlays_per_excerpts
    //                             .entry(excerpt_id)
    //                             .or_default()
    //                             .add(new_anchor, (id, new_inlay.clone()));
    //                         to_insert.push((id, new_anchor, new_inlay));
    //                     }
    //                 }
    //             }
    //             None => {
    //                 let mut inlays_per_excerpts: HashMap<
    //                     ExcerptId,
    //                     OrderedByAnchorOffset<(InlayId, InlayHint)>,
    //                 > = HashMap::default();
    //                 for (new_excerpt_id, new_ordered_inlays) in new_buffer_inlays {
    //                     if let Some((_, new_ordered_inlays)) = new_ordered_inlays {
    //                         for (new_anchor, new_inlay) in
    //                             new_ordered_inlays.into_ordered_elements()
    //                         {
    //                             let id = InlayId(post_inc(&mut self.next_inlay_id));
    //                             inlays_per_excerpts
    //                                 .entry(new_excerpt_id)
    //                                 .or_default()
    //                                 .add(new_anchor, (id, new_inlay.clone()));
    //                             to_insert.push((id, new_anchor, new_inlay));
    //                         }
    //                     }
    //                 }
    //                 self.inlays_per_buffer.insert(
    //                     buffer_path,
    //                     BufferInlays {
    //                         buffer_version,
    //                         inlays_per_excerpts,
    //                     },
    //                 );
    //             }
    //         }
    //     }

    //     for (_, old_buffer_inlays) in old_inlays {
    //         for (_, old_excerpt_inlays) in old_buffer_inlays.inlays_per_excerpts {
    //             for (_, (id_to_remove, _)) in old_excerpt_inlays.into_ordered_elements() {
    //                 to_remove.push(id_to_remove);
    //             }
    //         }
    //     }

    //     to_insert.retain(|(_, _, new_hint)| self.allowed_hint_kinds.contains(&new_hint.kind));

    //     InlaySplice {
    //         to_remove,
    //         to_insert,
    //     }
    // }
}

fn allowed_inlay_hint_types(
    inlay_hint_settings: editor_settings::InlayHints,
) -> HashSet<Option<InlayHintKind>> {
    let mut new_allowed_inlay_hint_types = HashSet::default();
    if inlay_hint_settings.show_type_hints {
        new_allowed_inlay_hint_types.insert(Some(InlayHintKind::Type));
    }
    if inlay_hint_settings.show_parameter_hints {
        new_allowed_inlay_hint_types.insert(Some(InlayHintKind::Parameter));
    }
    if inlay_hint_settings.show_other_hints {
        new_allowed_inlay_hint_types.insert(None);
    }
    new_allowed_inlay_hint_types
}
