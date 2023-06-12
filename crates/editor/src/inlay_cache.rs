use std::cmp;

use crate::{Anchor, ExcerptId};
use clock::Global;
use project::InlayHint;
use util::post_inc;

use collections::{BTreeMap, HashMap};

#[derive(Clone, Debug, Default)]
pub struct InlayCache {
    inlays_per_buffer: HashMap<u64, BufferInlays>,
    next_inlay_id: usize,
}

#[derive(Clone, Debug)]
pub struct OrderedByAnchorOffset<T>(pub BTreeMap<usize, (Anchor, T)>);

impl<T> OrderedByAnchorOffset<T> {
    pub fn add(&mut self, anchor: Anchor, t: T) {
        self.0.insert(anchor.text_anchor.offset, (anchor, t));
    }

    fn into_ordered_elements(self) -> impl Iterator<Item = (Anchor, T)> {
        self.0.into_values()
    }
}

impl<T> Default for OrderedByAnchorOffset<T> {
    fn default() -> Self {
        Self(BTreeMap::default())
    }
}

#[derive(Clone, Debug, Default)]
struct BufferInlays {
    buffer_version: Global,
    inlays_per_excerpts: HashMap<ExcerptId, OrderedByAnchorOffset<(InlayId, InlayHint)>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InlayId(pub usize);

#[derive(Debug)]
pub struct InlaysUpdate {
    pub to_remove: Vec<InlayId>,
    pub to_insert: Vec<(InlayId, Anchor, InlayHint)>,
}

impl InlayCache {
    pub fn inlays_up_to_date(
        &self,
        buffer_id: u64,
        buffer_version: &Global,
        excerpt_id: ExcerptId,
    ) -> bool {
        let Some(buffer_inlays) = self.inlays_per_buffer.get(&buffer_id) else { return false };
        let buffer_up_to_date = buffer_version == &buffer_inlays.buffer_version
            || buffer_inlays.buffer_version.changed_since(buffer_version);
        buffer_up_to_date && buffer_inlays.inlays_per_excerpts.contains_key(&excerpt_id)
    }

    pub fn update_inlays(
        &mut self,
        new_inlays: HashMap<u64, (Global, HashMap<ExcerptId, OrderedByAnchorOffset<InlayHint>>)>,
    ) -> InlaysUpdate {
        let mut old_inlays = self.inlays_per_buffer.clone();
        let mut to_remove = Vec::new();
        let mut to_insert = Vec::new();

        for (buffer_id, (buffer_version, new_buffer_inlays)) in new_inlays {
            match old_inlays.remove(&buffer_id) {
                Some(mut old_buffer_inlays) => {
                    for (excerpt_id, new_excerpt_inlays) in new_buffer_inlays {
                        if self.inlays_up_to_date(buffer_id, &buffer_version, excerpt_id) {
                            continue;
                        }

                        let self_inlays_per_buffer = self
                            .inlays_per_buffer
                            .get_mut(&buffer_id)
                            .expect("element expected: `old_inlays.remove` returned `Some`");
                        let mut new_excerpt_inlays =
                            new_excerpt_inlays.into_ordered_elements().fuse().peekable();
                        if old_buffer_inlays
                            .inlays_per_excerpts
                            .remove(&excerpt_id)
                            .is_some()
                        {
                            let self_excerpt_inlays = self_inlays_per_buffer
                                .inlays_per_excerpts
                                .get_mut(&excerpt_id)
                                .expect("element expected: `old_excerpt_inlays` is `Some`");
                            let mut hints_to_add = Vec::<(Anchor, (InlayId, InlayHint))>::new();
                            self_excerpt_inlays.0.retain(
                                |_, (old_anchor, (old_inlay_id, old_inlay))| {
                                    let mut retain = false;

                                    while let Some(new_offset) = new_excerpt_inlays
                                        .peek()
                                        .map(|(new_anchor, _)| new_anchor.text_anchor.offset)
                                    {
                                        let old_offset = old_anchor.text_anchor.offset;
                                        match new_offset.cmp(&old_offset) {
                                            cmp::Ordering::Less => {
                                                let (new_anchor, new_inlay) =
                                                    new_excerpt_inlays.next().expect(
                                                        "element expected: `peek` returned `Some`",
                                                    );
                                                hints_to_add.push((
                                                    new_anchor,
                                                    (
                                                        InlayId(post_inc(&mut self.next_inlay_id)),
                                                        new_inlay,
                                                    ),
                                                ));
                                            }
                                            cmp::Ordering::Equal => {
                                                let (new_anchor, new_inlay) =
                                                    new_excerpt_inlays.next().expect(
                                                        "element expected: `peek` returned `Some`",
                                                    );
                                                if &new_inlay == old_inlay {
                                                    retain = true;
                                                } else {
                                                    hints_to_add.push((
                                                        new_anchor,
                                                        (
                                                            InlayId(post_inc(
                                                                &mut self.next_inlay_id,
                                                            )),
                                                            new_inlay,
                                                        ),
                                                    ));
                                                }
                                            }
                                            cmp::Ordering::Greater => break,
                                        }
                                    }

                                    if !retain {
                                        to_remove.push(*old_inlay_id);
                                    }
                                    retain
                                },
                            );

                            for (new_anchor, (id, new_inlay)) in hints_to_add {
                                self_excerpt_inlays.add(new_anchor, (id, new_inlay.clone()));
                                to_insert.push((id, new_anchor, new_inlay));
                            }
                        }

                        for (new_anchor, new_inlay) in new_excerpt_inlays {
                            let id = InlayId(post_inc(&mut self.next_inlay_id));
                            self_inlays_per_buffer
                                .inlays_per_excerpts
                                .entry(excerpt_id)
                                .or_default()
                                .add(new_anchor, (id, new_inlay.clone()));
                            to_insert.push((id, new_anchor, new_inlay));
                        }
                    }
                }
                None => {
                    let mut inlays_per_excerpts: HashMap<
                        ExcerptId,
                        OrderedByAnchorOffset<(InlayId, InlayHint)>,
                    > = HashMap::default();
                    for (new_excerpt_id, new_ordered_inlays) in new_buffer_inlays {
                        for (new_anchor, new_inlay) in new_ordered_inlays.into_ordered_elements() {
                            let id = InlayId(post_inc(&mut self.next_inlay_id));
                            inlays_per_excerpts
                                .entry(new_excerpt_id)
                                .or_default()
                                .add(new_anchor, (id, new_inlay.clone()));
                            to_insert.push((id, new_anchor, new_inlay));
                        }
                    }
                    self.inlays_per_buffer.insert(
                        buffer_id,
                        BufferInlays {
                            buffer_version,
                            inlays_per_excerpts,
                        },
                    );
                }
            }
        }

        for (_, old_buffer_inlays) in old_inlays {
            for (_, old_excerpt_inlays) in old_buffer_inlays.inlays_per_excerpts {
                for (_, (id_to_remove, _)) in old_excerpt_inlays.into_ordered_elements() {
                    to_remove.push(id_to_remove);
                }
            }
        }

        InlaysUpdate {
            to_remove,
            to_insert,
        }
    }
}
