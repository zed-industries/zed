use crate::{
    btree, dense_id::DenseId, AnchorRange, DocumentFragment, DocumentFragmentSummary,
    DocumentMetadata, InsertionFragment, OperationId, Revision, RevisionId, RopeBuilder, Tombstone,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{cmp, sync::Arc};
use sum_tree::Bias;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateBranch {
    pub id: OperationId,
    pub parent: RevisionId,
    pub name: Arc<str>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDocument {
    pub id: OperationId,
    pub branch_id: OperationId,
    pub parent: RevisionId,
}

impl CreateDocument {
    pub fn apply(self, revision: &mut Revision) {
        let mut cursor = revision.document_fragments.cursor::<OperationId>();
        let mut new_document_fragments = cursor.slice(&self.id, Bias::Right, &());
        new_document_fragments.push(
            DocumentFragment {
                document_id: self.id,
                location: DenseId::min(),
                insertion_id: self.id,
                insertion_subrange: 0..0,
                tombstones: Default::default(),
                undo_count: 0,
            },
            &(),
        );
        new_document_fragments.append(cursor.suffix(&()), &());
        drop(cursor);

        revision.document_fragments = new_document_fragments;
        revision.insertion_fragments.insert_or_replace(
            InsertionFragment {
                insertion_id: self.id,
                offset_in_insertion: 0,
                fragment_location: DenseId::min(),
            },
            &(),
        );
        revision.document_metadata.insert(
            self.id,
            DocumentMetadata {
                path: None,
                last_change: self.id,
            },
        );
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edit {
    pub id: OperationId,
    pub document_id: OperationId,
    pub branch_id: OperationId,
    pub parent: RevisionId,
    pub edits: SmallVec<[(AnchorRange, Arc<str>); 2]>,
}

impl Edit {
    pub fn apply(self, parent_revision: &Revision, revision: &mut Revision) -> Result<()> {
        let mut old_fragments = revision
            .document_fragments
            .cursor::<DocumentFragmentSummary>();

        // Slice to the start of the document this to which this operation applies.
        let mut new_fragments = old_fragments.slice(&self.document_id, Bias::Left, &());
        let mut new_insertions = Vec::new();
        let mut new_ropes = RopeBuilder::new(
            revision.visible_text.cursor(0),
            revision.hidden_text.cursor(0),
        );
        new_ropes.append(
            new_fragments.summary().visible_len,
            new_fragments.summary().hidden_len,
        );

        let mut insertion_offset = 0;

        // Every document begins with a sentinel fragment, which we can skip.
        new_fragments.push(old_fragments.item().unwrap().clone(), &());
        old_fragments.next(&());

        let mut current_fragment = old_fragments.item().cloned();
        for (range, new_text) in self.edits {
            // We need to tombstone the intersection of the edit's range with fragments that
            // were visible in the operation's parent revision.
            for mut parent_fragment in parent_revision
                .visible_fragments_for_range(range.clone())?
                .cloned()
            {
                // Intersect the parent fragment with the edit's range.
                if parent_fragment.insertion_id == range.start_insertion_id {
                    parent_fragment.insertion_subrange.start = range.start_offset_in_insertion;
                }
                if parent_fragment.insertion_id == range.end_insertion_id {
                    parent_fragment.insertion_subrange.end = cmp::min(
                        parent_fragment.insertion_subrange.end,
                        range.end_offset_in_insertion,
                    );
                }

                // Find the locations of the parent fragment in the new revision.
                for fragment_location in revision.fragment_locations(
                    parent_fragment.insertion_id,
                    parent_fragment.insertion_subrange,
                ) {
                    if let Some(fragment) = current_fragment.as_ref() {
                        // Advance to fragment_location if it is greater than the location of the current fragment,
                        if *fragment_location > fragment.location {
                            // Flush the remainder of current fragment.
                            if !fragment.insertion_subrange.is_empty() {
                                new_ropes.push_fragment(fragment, fragment.visible());
                                new_fragments.push(fragment.clone(), &());
                            }
                            old_fragments.next(&());

                            // Append all fragments between the previous fragment and the new fragment_location.
                            let slice = old_fragments.slice(
                                &(self.document_id, fragment_location),
                                Bias::Left,
                                &(),
                            );
                            new_ropes
                                .append(slice.summary().visible_len, slice.summary().hidden_len);
                            new_fragments.append(slice, &());
                            current_fragment = old_fragments.item().cloned();

                            // We should always find a fragment when seeking to fragment_location.
                            debug_assert!(current_fragment.is_some());
                        }
                    }

                    // If the edit starts at the end of the current fragment, flush it.
                    if let Some(fragment) = current_fragment.as_ref() {
                        if fragment.insertion_id == range.start_insertion_id
                            && fragment.insertion_subrange.end == range.start_offset_in_insertion
                        {
                            let fragment = current_fragment.take().unwrap();
                            new_ropes.push_fragment(&fragment, fragment.visible());
                            new_fragments.push(fragment, &());
                            old_fragments.next(&());
                            if let Some(next_fragment) = old_fragments.item() {
                                if next_fragment.document_id == self.document_id {
                                    current_fragment = Some(next_fragment.clone());
                                }
                            }
                        }
                    }

                    if let Some(fragment) = current_fragment.take() {
                        // If we haven't advanced off the end, then the current fragment intersects
                        // the current edit's range.
                        let (prefix, mut intersection, suffix) = fragment.intersect(range.clone());

                        // If we have a prefix, push it.
                        if let Some(mut prefix) = prefix {
                            prefix.location = DenseId::between(
                                &new_fragments.summary().max_location,
                                &intersection.location,
                            );
                            new_insertions
                                .push(btree::Edit::Insert(InsertionFragment::new(&prefix)));
                            new_ropes.push_fragment(&prefix, prefix.visible());
                            new_fragments.push(prefix, &());
                        }

                        if let Some(suffix) = suffix {
                            intersection.location = DenseId::between(
                                &new_fragments.summary().max_location,
                                &suffix.location,
                            );
                            // If we still have a suffix, the next edit may be inside of it, so set it as
                            // the current fragment and continue the loop.
                            current_fragment = Some(suffix);
                        } else {
                            // Otherwise, advance to the next fragment if it's still part of the same document.
                            old_fragments.next(&());
                            if let Some(next_fragment) = old_fragments.item() {
                                if next_fragment.document_id == self.document_id {
                                    current_fragment = Some(next_fragment.clone());
                                }
                            }
                        }

                        // Then tombstone the intersecting portion.
                        let was_visible = intersection.visible();
                        intersection.tombstones.push(Tombstone {
                            id: self.id,
                            undo_count: 0,
                        });
                        new_ropes.push_fragment(&intersection, was_visible);
                        new_fragments.push(intersection, &());
                    }
                }
            }

            // Move past insertions that were causally after the current operation.
            while let Some(fragment) = current_fragment.as_ref() {
                if fragment.insertion_id.is_causally_after(self.id) {
                    new_ropes.push_fragment(fragment, fragment.visible());
                    new_fragments.push(fragment.clone(), &());
                    old_fragments.next(&());
                    current_fragment = old_fragments.item().cloned();
                } else {
                    break;
                }
            }

            // Finally, insert a fragment containing the new text.
            if !new_text.is_empty() {
                let fragment = DocumentFragment {
                    document_id: self.document_id,
                    location: DenseId::between(
                        &new_fragments.summary().max_location,
                        current_fragment
                            .as_ref()
                            .map_or(DenseId::max_ref(), |fragment| &fragment.location),
                    ),
                    insertion_id: self.id,
                    insertion_subrange: insertion_offset..insertion_offset + new_text.len(),
                    tombstones: Default::default(),
                    undo_count: 0,
                };
                new_insertions.push(btree::Edit::Insert(InsertionFragment::new(&fragment)));
                new_ropes.push_str(new_text.as_ref());
                new_fragments.push(fragment, &());
                insertion_offset += new_text.len();
            }
        }

        if let Some(fragment) = current_fragment {
            if !fragment.insertion_subrange.is_empty() {
                new_ropes.push_fragment(&fragment, fragment.visible());
                new_fragments.push(fragment, &());
            }
            old_fragments.next(&());
        }

        let suffix = old_fragments.suffix(&());
        new_ropes.append(suffix.summary().visible_len, suffix.summary().hidden_len);
        new_fragments.append(suffix, &());
        let (visible_text, hidden_text) = new_ropes.finish();
        drop(old_fragments);

        revision.document_fragments = new_fragments;
        revision.insertion_fragments.edit(new_insertions, &());
        revision.visible_text = visible_text;
        revision.hidden_text = hidden_text;

        Ok(())
    }
}
