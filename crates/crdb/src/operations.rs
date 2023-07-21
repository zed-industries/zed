use crate::{
    btree, dense_id::DenseId, AnchorRange, BranchSnapshot, DocumentFragment,
    DocumentFragmentSummary, DocumentMetadata, InsertionFragment, OperationId, RepoSnapshot,
    Revision, RevisionId, RopeBuilder,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use std::{cmp::Reverse, sync::Arc};
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
        let mut fragment_insertion_start = 0;
        for (range, new_text) in self.edits {
            let start_insertion = revision
                .insertion_fragment(range.start())
                .ok_or_else(|| anyhow!("cannot find insertion start"))?;

            let fragment = old_fragments.item().unwrap();
            if start_insertion.insertion_id != fragment.insertion_id
                || start_insertion.offset_in_insertion > fragment.insertion_subrange.end
            {
                if fragment_insertion_start > fragment.insertion_subrange.start {
                    if fragment.insertion_subrange.end > fragment_insertion_start {
                        let mut suffix = fragment.clone();
                        suffix.insertion_subrange.start = fragment_insertion_start;
                        new_insertions.push(btree::Edit::Insert(InsertionFragment::new(&suffix)));
                        new_ropes.push_fragment(&suffix, suffix.visible());
                        new_fragments.push(suffix, &());
                    }

                    old_fragments.next(&());
                }

                new_fragments.append(
                    old_fragments.slice(
                        &(self.document_id, &start_insertion.fragment_location),
                        Bias::Left,
                        &(),
                    ),
                    &(),
                );
                fragment_insertion_start = old_fragments.item().unwrap().insertion_subrange.start;
            }

            let fragment = old_fragments.item().unwrap();
            if fragment.insertion_subrange.end == range.start_offset_in_insertion
                && (fragment.insertion_subrange.end > fragment_insertion_start
                    || fragment.insertion_id == self.document_id)
            {
                let mut fragment = fragment.clone();
                fragment.insertion_subrange.start = fragment_insertion_start;
                new_insertions.push(btree::Edit::Insert(InsertionFragment::new(&fragment)));
                new_ropes.push_fragment(&fragment, fragment.visible());
                new_fragments.push(fragment, &());
                old_fragments.next(&());
                fragment_insertion_start = old_fragments.item().unwrap().insertion_subrange.start;
            }

            // Skip over insertions that are concurrent to this edit, but have a higher lamport
            // timestamp.
            while let Some(fragment) = old_fragments.item() {
                if fragment.insertion_id.is_causally_after(self.id) {
                    new_ropes.push_fragment(fragment, fragment.visible());
                    new_fragments.push(fragment.clone(), &());
                    old_fragments.next(&());
                    fragment_insertion_start =
                        old_fragments.item().unwrap().insertion_subrange.start;
                } else {
                    break;
                }
            }

            let fragment = old_fragments.item().unwrap();
            if fragment.insertion_id == range.start_insertion_id
                && range.start_offset_in_insertion > fragment_insertion_start
            {
                let mut prefix = fragment.clone();
                let prefix_len = range.start_offset_in_insertion - fragment_insertion_start;
                prefix.insertion_subrange.start = fragment_insertion_start;
                prefix.insertion_subrange.end = prefix.insertion_subrange.start + prefix_len;
                prefix.location =
                    DenseId::between(&new_fragments.summary().max_location, &prefix.location);
                new_insertions.push(btree::Edit::Insert(InsertionFragment::new(&prefix)));
                new_ropes.push_fragment(&prefix, prefix.visible());
                new_fragments.push(prefix, &());
                fragment_insertion_start = range.start_offset_in_insertion;
            }

            if !new_text.is_empty() {
                let fragment = DocumentFragment {
                    document_id: self.document_id,
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

            while let Some(fragment) = old_fragments.item() {
                for parent_fragment in parent_revision.fragments_for_insertion_range(
                    fragment.insertion_id,
                    fragment.insertion_subrange,
                ) {}

                if fragment.insertion_id != range.end_insertion_id {
                    todo!()
                } else {
                    // calculate intersection
                    break;
                }
            }
        }

        // If the current fragment has been partially consumed, then consume the rest of it
        // and advance to the next fragment before slicing.
        let fragment = old_fragments.item().unwrap();
        if fragment_insertion_start > fragment.insertion_subrange.start {
            if fragment.insertion_subrange.end > fragment_insertion_start {
                let mut suffix = old_fragments.item().unwrap().clone();
                let suffix_len = fragment.insertion_subrange.end - fragment_insertion_start;
                suffix.insertion_subrange.start = fragment_insertion_start;
                suffix.insertion_subrange.end = suffix.insertion_subrange.start + suffix_len;
                new_insertions.push(btree::Edit::Insert(InsertionFragment::new(&suffix)));
                new_ropes.push_fragment(&suffix, suffix.visible());
                new_fragments.push(suffix, &());
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
