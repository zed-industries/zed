use crate::{
    btree, dense_id::DenseId, Anchor, AnchorRange, BranchSnapshot, DocumentFragment,
    DocumentFragmentSummary, DocumentMetadata, InsertionFragment, OperationId, RepoSnapshot,
    Revision, RevisionId, RopeBuilder, Tombstone,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use std::{
    cmp::{self, Reverse},
    sync::Arc,
};
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
        let mut fragment = old_fragments.item().unwrap().clone();
        for (range, new_text) in self.edits {
            // Locate all the fragments in the parent revision intersecting this edit.
            for parent_fragment in parent_revision.visible_fragments_for_range(range.clone())? {
                let insertion_id = parent_fragment.insertion_id;
                let mut insertion_subrange = parent_fragment.insertion_subrange.clone();
                if parent_fragment.insertion_id == range.start_insertion_id {
                    insertion_subrange.start = range.start_offset_in_insertion;
                }
                if parent_fragment.insertion_id == range.end_insertion_id {
                    insertion_subrange.end =
                        cmp::min(insertion_subrange.end, range.end_offset_in_insertion);
                }

                // Find the locations of the parent fragment in the new revision.
                for fragment_location in
                    revision.fragment_locations(insertion_id, insertion_subrange)
                {
                    // Flush the current fragment if we are about to move past it.
                    if *fragment_location > fragment.location {
                        if fragment.insertion_subrange.len() > 0 || fragment.insertion_id == self.id
                        {
                            new_ropes.push_fragment(&fragment, fragment.visible());
                            new_fragments.push(fragment, &());
                        }

                        old_fragments.next(&());
                        new_fragments.append(
                            old_fragments.slice(
                                &(self.document_id, fragment_location),
                                Bias::Left,
                                &(),
                            ),
                            &(),
                        );
                        fragment = old_fragments.item().unwrap().clone();
                    }

                    let (prefix, mut intersection, suffix) = fragment.intersect(range.clone());
                    if let Some(prefix) = prefix {
                        new_insertions.push(btree::Edit::Insert(InsertionFragment::new(&prefix)));
                        new_ropes.push_fragment(&prefix, prefix.visible());
                        new_fragments.push(prefix, &());
                    }

                    let was_visible = intersection.visible();
                    intersection.tombstones.push(Tombstone {
                        id: self.id,
                        undo_count: 0,
                    });
                    new_ropes.push_fragment(&intersection, was_visible);
                    new_fragments.push(intersection, &());

                    if let Some(suffix) = suffix {
                        fragment = suffix;
                    } else {
                        old_fragments.next(&());
                        fragment = old_fragments.item().unwrap().clone();
                    }
                }
            }

            // Move past insertions that were causally after the current operation.
            while fragment.insertion_id.is_causally_after(self.id) {
                new_ropes.push_fragment(&fragment, fragment.visible());
                new_fragments.push(fragment, &());
                old_fragments.next(&());
                fragment = old_fragments.item().unwrap().clone();
            }

            // Finally insert the new text.
            if !new_text.is_empty() {
                let fragment = DocumentFragment {
                    document_id: self.id,
                    location: DenseId::between(
                        &new_fragments.summary().max_location,
                        &fragment.location,
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

        if fragment.insertion_subrange.len() > 0 || fragment.insertion_id == self.id {
            new_ropes.push_fragment(&fragment, fragment.visible());
            new_fragments.push(fragment, &());
        }
        old_fragments.next(&());

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
