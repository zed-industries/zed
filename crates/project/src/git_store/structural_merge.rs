use crate::git_store::conflict_set::ConflictRegion;
use collections::{HashMap, HashSet};
use language::Language;
use std::sync::Arc;
use text::{Anchor, OffsetRangeExt as _};
use tree_sitter::{Node, Parser, Query, QueryCursor, StreamingIterator as _, Tree};

/// The result of attempting a structural merge on a single conflict region.
#[derive(Debug, Clone)]
pub enum StructuralMergeOutcome {
    Resolved { text: String, method: ResolveMethod },
    Deferred(DeferReason),
}

impl StructuralMergeOutcome {
    pub fn resolved_text(&self) -> Option<&str> {
        match self {
            Self::Resolved { text, .. } => Some(text),
            Self::Deferred(_) => None,
        }
    }

    pub fn is_resolved(&self) -> bool {
        matches!(self, Self::Resolved { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolveMethod {
    /// The conflict was inside an `@merge.set` container and resolved by
    /// matching items by key.
    Set,
    /// The conflict was inside an `@merge.ordered_list` container and
    /// resolved by item-level 3-way merge.
    OrderedList,
}

/// Why a structural merge declined to resolve a conflict region. The
/// fallback after a `Deferred` is line-level decomposition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeferReason {
    /// No language/grammar or no `merges.scm` registered.
    NoStructuralSupport,
    /// The conflict region doesn't sit inside an `@merge.set` or
    /// `@merge.ordered_list` node in all three reconstructed parses.
    NoEnclosingContainer,
    /// The enclosing container has different syntactic kinds on the three
    /// sides — probably means the markers cut something the parser ended up
    /// healing differently.
    DifferentContainerKinds,
    /// At least one side has an item whose byte range straddles the conflict
    /// region boundary — the markers split a syntactic item.
    ChildSplitsRegion,
    /// Two items share an identity key on the same side (would be ambiguous
    /// to compare against the other side).
    DuplicateKey { side: ConflictSide, key: String },
    /// Same item modified differently on ours and theirs.
    BothModifiedDifferently { key: String },
    /// One side removed an item the other modified.
    DeleteVsModify {
        key: String,
        deleted_by: ConflictSide,
    },
    /// Both sides added a new item with the same key but different texts.
    BothAddedDifferently { key: String },
    /// ours adds an item with key K in this region; a different region adds
    /// the same key on theirs — auto-merging both would produce a duplicate
    /// definition. `other_region` is the anchor of one of the colliding
    /// regions (a single collision per region is reported, even when more
    /// than one other region adds the same key).
    CrossRegionKeyCollision {
        key: String,
        added_by: ConflictSide,
        other_region: Anchor,
    },
    /// Ordered-list merge found overlapping hunks on the two sides.
    OrderedHunksOverlap,
    /// The structural analysis ran but the result equals the original — no
    /// improvement to make.
    NoChange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictSide {
    Base,
    Ours,
    Theirs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContainerKind {
    Set,
    OrderedList,
}

/// Result of building three reconstructed views of a buffer (base / ours /
/// theirs) and parsing each with the buffer's language. The context can be
/// queried per [`ConflictRegion`] to attempt a structural merge.
pub struct LanguageMergeContext {
    base: SideContext,
    ours: SideContext,
    theirs: SideContext,
    region_offsets: HashMap<Anchor, RegionOffsets>,
    /// Cross-region collisions detected during build: keyed by region anchor,
    /// value is the pre-computed defer reason that overrides whatever a
    /// single-region analysis would otherwise produce.
    cross_region_defer: HashMap<Anchor, DeferReason>,
}

struct SideContext {
    source: String,
    tree: Tree,
    merge_set_nodes: HashSet<usize>,
    ordered_list_nodes: HashSet<usize>,
    item_keys: HashMap<usize, String>,
}

impl SideContext {
    fn in_region_children(
        &self,
        container: Node<'_>,
        offset: usize,
        len: usize,
    ) -> Option<Vec<InRegionItem>> {
        in_region_children(container, &self.source, &self.item_keys, offset..offset + len)
    }
}

#[derive(Debug, Clone, Copy)]
struct RegionOffsets {
    base: usize,
    ours: usize,
    theirs: usize,
    base_len: usize,
    ours_len: usize,
    theirs_len: usize,
}

impl LanguageMergeContext {
    /// Build three reconstructed sources and parse each, or return `None` if
    /// the language has no grammar or no `merges.scm` registered, no conflict
    /// regions, or any parse fails.
    pub fn build(
        buffer: &text::BufferSnapshot,
        language: Arc<Language>,
        conflicts: &[ConflictRegion],
    ) -> Option<Self> {
        if conflicts.is_empty() {
            return None;
        }
        let grammar = language.grammar()?.clone();
        let merges_config = grammar.merges_config.as_ref()?;
        if merges_config.set_capture_ix.is_none() && merges_config.ordered_capture_ix.is_none() {
            return None;
        }
        let capture_indices = MergesCaptureIndices {
            set: merges_config.set_capture_ix,
            ordered: merges_config.ordered_capture_ix,
            key: merges_config.key_capture_ix,
            key_normalized: merges_config.key_normalized_capture_ix,
        };

        let buffer_len = buffer.len();
        let mut base_source = String::with_capacity(buffer_len);
        let mut ours_source = String::with_capacity(buffer_len);
        let mut theirs_source = String::with_capacity(buffer_len);
        let mut region_offsets = HashMap::default();

        let mut cursor = 0usize;
        for conflict in conflicts {
            let outer = conflict.range.to_offset(buffer);
            if outer.start < cursor {
                return None;
            }
            let prefix: String = buffer.text_for_range(cursor..outer.start).collect();
            base_source.push_str(&prefix);
            ours_source.push_str(&prefix);
            theirs_source.push_str(&prefix);

            let base_text: String = conflict
                .base
                .as_ref()
                .map(|range| buffer.text_for_range(range.clone()).collect::<String>())
                .unwrap_or_default();
            let ours_text: String = buffer.text_for_range(conflict.ours.clone()).collect();
            let theirs_text: String = buffer.text_for_range(conflict.theirs.clone()).collect();

            region_offsets.insert(
                conflict.range.start,
                RegionOffsets {
                    base: base_source.len(),
                    ours: ours_source.len(),
                    theirs: theirs_source.len(),
                    base_len: base_text.len(),
                    ours_len: ours_text.len(),
                    theirs_len: theirs_text.len(),
                },
            );

            base_source.push_str(&base_text);
            ours_source.push_str(&ours_text);
            theirs_source.push_str(&theirs_text);
            cursor = outer.end;
        }
        let trailing: String = buffer.text_for_range(cursor..buffer_len).collect();
        base_source.push_str(&trailing);
        ours_source.push_str(&trailing);
        theirs_source.push_str(&trailing);

        let ts_language = &grammar.ts_language;
        let mut parser = Parser::new();
        parser.set_language(ts_language).ok()?;

        let base = build_side(&mut parser, base_source, &merges_config.query, &capture_indices, None)?;
        let mut ours_prior = base.tree.clone();
        apply_side_edits(&mut ours_prior, conflicts, &region_offsets, buffer, Side::Ours, &base.source);
        let ours = build_side(
            &mut parser,
            ours_source,
            &merges_config.query,
            &capture_indices,
            Some(&ours_prior),
        )?;
        let mut theirs_prior = base.tree.clone();
        apply_side_edits(
            &mut theirs_prior,
            conflicts,
            &region_offsets,
            buffer,
            Side::Theirs,
            &base.source,
        );
        let theirs = build_side(
            &mut parser,
            theirs_source,
            &merges_config.query,
            &capture_indices,
            Some(&theirs_prior),
        )?;

        let mut context = Self {
            base,
            ours,
            theirs,
            region_offsets,
            cross_region_defer: HashMap::default(),
        };
        context.detect_cross_region_collisions(conflicts);
        Some(context)
    }

    /// Try to structurally merge `region`. Returns either a `Resolved`
    /// substitution text or a `Deferred(DeferReason)` explaining why the
    /// engine punted. Line-level decomposition is the natural fallback.
    pub fn try_merge_region(&self, region: &ConflictRegion) -> StructuralMergeOutcome {
        let Some(offsets) = self.region_offsets.get(&region.range.start).copied() else {
            return StructuralMergeOutcome::Deferred(DeferReason::NoStructuralSupport);
        };

        if let Some(reason) = self.cross_region_defer.get(&region.range.start) {
            return StructuralMergeOutcome::Deferred(reason.clone());
        }

        let base_lookup = enclosing_container(&self.base, offsets.base, offsets.base_len);
        let ours_lookup = enclosing_container(&self.ours, offsets.ours, offsets.ours_len);
        let theirs_lookup = enclosing_container(&self.theirs, offsets.theirs, offsets.theirs_len);

        let (Some((base_node, base_kind)), Some((ours_node, ours_kind)), Some((theirs_node, theirs_kind))) =
            (base_lookup, ours_lookup, theirs_lookup)
        else {
            return StructuralMergeOutcome::Deferred(DeferReason::NoEnclosingContainer);
        };

        if base_node.kind_id() != ours_node.kind_id()
            || base_node.kind_id() != theirs_node.kind_id()
            || base_kind != ours_kind
            || base_kind != theirs_kind
        {
            return StructuralMergeOutcome::Deferred(DeferReason::DifferentContainerKinds);
        }

        let Some(base_items) = self.base.in_region_children(base_node, offsets.base, offsets.base_len)
        else {
            return StructuralMergeOutcome::Deferred(DeferReason::ChildSplitsRegion);
        };
        let Some(ours_items) = self.ours.in_region_children(ours_node, offsets.ours, offsets.ours_len)
        else {
            return StructuralMergeOutcome::Deferred(DeferReason::ChildSplitsRegion);
        };
        let Some(theirs_items) = self
            .theirs
            .in_region_children(theirs_node, offsets.theirs, offsets.theirs_len)
        else {
            return StructuralMergeOutcome::Deferred(DeferReason::ChildSplitsRegion);
        };

        match base_kind {
            ContainerKind::Set => self.merge_set(&base_items, &ours_items, &theirs_items),
            ContainerKind::OrderedList => self.merge_ordered(&base_items, &ours_items, &theirs_items),
        }
    }

    fn merge_set(
        &self,
        base_items: &[InRegionItem],
        ours_items: &[InRegionItem],
        theirs_items: &[InRegionItem],
    ) -> StructuralMergeOutcome {

        let by_key = |items, side| {
            items_by_key(items)
                .map_err(|key| StructuralMergeOutcome::Deferred(DeferReason::DuplicateKey { side, key }))
        };
        let base_by_key = match by_key(base_items, ConflictSide::Base) {
            Ok(map) => map,
            Err(outcome) => return outcome,
        };
        let theirs_by_key = match by_key(theirs_items, ConflictSide::Theirs) {
            Ok(map) => map,
            Err(outcome) => return outcome,
        };
        if let Err(outcome) = by_key(ours_items, ConflictSide::Ours) {
            return outcome;
        }

        let mut output = String::new();
        let mut handled: HashSet<String> = HashSet::default();
        let mut any_change = false;

        for ours_item in ours_items {
            let key = ours_item.key.clone();
            handled.insert(key.clone());
            let base_item = base_by_key.get(key.as_str()).copied();
            let theirs_item = theirs_by_key.get(key.as_str()).copied();

            let text = match (base_item, theirs_item) {
                (None, None) => {
                    any_change = true;
                    Some(ours_item.text(&self.ours.source))
                }
                (None, Some(t)) => {
                    if t.normalized_text(&self.theirs.source)
                        == ours_item.normalized_text(&self.ours.source)
                    {
                        any_change = true;
                        Some(ours_item.text(&self.ours.source))
                    } else {
                        return StructuralMergeOutcome::Deferred(
                            DeferReason::BothAddedDifferently { key },
                        );
                    }
                }
                (Some(b), Some(t)) => {
                    let ours_changed = b.normalized_text(&self.base.source)
                        != ours_item.normalized_text(&self.ours.source);
                    let theirs_changed = b.normalized_text(&self.base.source)
                        != t.normalized_text(&self.theirs.source);
                    match (ours_changed, theirs_changed) {
                        (false, false) => Some(ours_item.text(&self.ours.source)),
                        (true, false) => {
                            any_change = true;
                            Some(ours_item.text(&self.ours.source))
                        }
                        (false, true) => {
                            any_change = true;
                            Some(t.text(&self.theirs.source))
                        }
                        (true, true) => {
                            if ours_item.normalized_text(&self.ours.source)
                                == t.normalized_text(&self.theirs.source)
                            {
                                Some(ours_item.text(&self.ours.source))
                            } else {
                                return StructuralMergeOutcome::Deferred(
                                    DeferReason::BothModifiedDifferently { key },
                                );
                            }
                        }
                    }
                }
                (Some(b), None) => {
                    if b.normalized_text(&self.base.source)
                        == ours_item.normalized_text(&self.ours.source)
                    {
                        any_change = true;
                        None
                    } else {
                        return StructuralMergeOutcome::Deferred(DeferReason::DeleteVsModify {
                            key,
                            deleted_by: ConflictSide::Theirs,
                        });
                    }
                }
            };

            if let Some(text) = text {
                push_with_newline(&mut output, text);
            }
        }

        for theirs_item in theirs_items {
            let key = theirs_item.key.clone();
            if handled.contains(&key) {
                continue;
            }
            handled.insert(key.clone());
            let base_item = base_by_key.get(key.as_str()).copied();
            match base_item {
                None => {
                    any_change = true;
                    push_with_newline(&mut output, theirs_item.text(&self.theirs.source));
                }
                Some(b) => {
                    if b.normalized_text(&self.base.source)
                        == theirs_item.normalized_text(&self.theirs.source)
                    {
                        any_change = true;
                    } else {
                        return StructuralMergeOutcome::Deferred(DeferReason::DeleteVsModify {
                            key,
                            deleted_by: ConflictSide::Ours,
                        });
                    }
                }
            }
        }

        // Any base item not seen on `ours` or `theirs` means both sides
        // deleted it; treat that as a change so we don't bail with NoChange.
        if base_items.iter().any(|item| !handled.contains(item.key.as_str())) {
            any_change = true;
        }

        if !any_change {
            StructuralMergeOutcome::Deferred(DeferReason::NoChange)
        } else {
            StructuralMergeOutcome::Resolved {
                text: output,
                method: ResolveMethod::Set,
            }
        }
    }

    fn merge_ordered(
        &self,
        base_items: &[InRegionItem],
        ours_items: &[InRegionItem],
        theirs_items: &[InRegionItem],
    ) -> StructuralMergeOutcome {
        // Diff base→ours and base→theirs at item granularity, using each
        // item's normalized text as the comparison token so any modification
        // shows up as a delete+insert.
        let base_tokens: Vec<&str> = base_items
            .iter()
            .map(|i| i.normalized_text(&self.base.source))
            .collect();
        let ours_tokens: Vec<&str> = ours_items
            .iter()
            .map(|i| i.normalized_text(&self.ours.source))
            .collect();
        let theirs_tokens: Vec<&str> = theirs_items
            .iter()
            .map(|i| i.normalized_text(&self.theirs.source))
            .collect();

        let ours_hunks = item_diff(&base_tokens, &ours_tokens);
        let theirs_hunks = item_diff(&base_tokens, &theirs_tokens);

        let mut output = String::new();
        let mut o_idx = 0;
        let mut t_idx = 0;
        let mut base_cursor: usize = 0;
        let mut any_change = false;

        loop {
            let next_o = ours_hunks.get(o_idx).map(|(b, _)| b.start as usize);
            let next_t = theirs_hunks.get(t_idx).map(|(b, _)| b.start as usize);
            let cluster_start = match (next_o, next_t) {
                (None, None) => {
                    for item in &base_items[base_cursor..] {
                        push_with_newline(&mut output, item.text(&self.base.source));
                    }
                    break;
                }
                (Some(o), None) => o,
                (None, Some(t)) => t,
                (Some(o), Some(t)) => o.min(t),
            };

            for item in &base_items[base_cursor..cluster_start] {
                push_with_newline(&mut output, item.text(&self.base.source));
            }
            base_cursor = cluster_start;

            let mut cluster_o = Vec::new();
            let mut cluster_t = Vec::new();
            let mut cluster_end = base_cursor;
            loop {
                let mut grew = false;
                if let Some((b, _)) = ours_hunks.get(o_idx)
                    && (b.start as usize) <= cluster_end
                {
                    cluster_o.push(o_idx);
                    cluster_end = cluster_end.max(b.end as usize);
                    o_idx += 1;
                    grew = true;
                }
                if let Some((b, _)) = theirs_hunks.get(t_idx)
                    && (b.start as usize) <= cluster_end
                {
                    cluster_t.push(t_idx);
                    cluster_end = cluster_end.max(b.end as usize);
                    t_idx += 1;
                    grew = true;
                }
                if !grew {
                    break;
                }
            }

            let ours_cluster_text = compose_side_text(
                base_items,
                ours_items,
                &ours_hunks,
                &cluster_o,
                base_cursor,
                cluster_end,
                &self.base.source,
                &self.ours.source,
            );
            let theirs_cluster_text = compose_side_text(
                base_items,
                theirs_items,
                &theirs_hunks,
                &cluster_t,
                base_cursor,
                cluster_end,
                &self.base.source,
                &self.theirs.source,
            );

            let cluster_text = if cluster_o.is_empty() {
                any_change = true;
                theirs_cluster_text
            } else if cluster_t.is_empty() {
                any_change = true;
                ours_cluster_text
            } else if ours_cluster_text == theirs_cluster_text {
                any_change = true;
                ours_cluster_text
            } else {
                return StructuralMergeOutcome::Deferred(DeferReason::OrderedHunksOverlap);
            };
            output.push_str(&cluster_text);
            if !cluster_text.is_empty() && !output.ends_with('\n') {
                output.push('\n');
            }
            base_cursor = cluster_end;
        }

        if !any_change {
            StructuralMergeOutcome::Deferred(DeferReason::NoChange)
        } else {
            StructuralMergeOutcome::Resolved {
                text: output,
                method: ResolveMethod::OrderedList,
            }
        }
    }

    fn detect_cross_region_collisions(&mut self, conflicts: &[ConflictRegion]) {
        // For each region, compute ours_added_keys and theirs_added_keys: items
        // whose key is present in that side but absent from base (within the
        // region). If region R1 has key K in ours_added AND a different
        // region R2 has key K in theirs_added, auto-merging both would
        // produce two definitions of K in the file. Both regions must defer.
        //
        // Note: same-key additions inside the SAME region are already detected
        // by the in-region pass (BothAddedDifferently / equal additions).
        let mut ours_added_by_region: HashMap<Anchor, HashSet<String>> = HashMap::default();
        let mut theirs_added_by_region: HashMap<Anchor, HashSet<String>> = HashMap::default();
        for conflict in conflicts {
            let Some(offsets) = self.region_offsets.get(&conflict.range.start).copied() else {
                continue;
            };
            let base_node = enclosing_merge_node(
                &self.base.tree,
                &self.base.merge_set_nodes,
                offsets.base,
                offsets.base_len,
            );
            let ours_node = enclosing_merge_node(
                &self.ours.tree,
                &self.ours.merge_set_nodes,
                offsets.ours,
                offsets.ours_len,
            );
            let theirs_node = enclosing_merge_node(
                &self.theirs.tree,
                &self.theirs.merge_set_nodes,
                offsets.theirs,
                offsets.theirs_len,
            );
            let (Some(base_node), Some(ours_node), Some(theirs_node)) =
                (base_node, ours_node, theirs_node)
            else {
                continue;
            };

            let (Some(base_items), Some(ours_items), Some(theirs_items)) = (
                self.base
                    .in_region_children(base_node, offsets.base, offsets.base_len),
                self.ours
                    .in_region_children(ours_node, offsets.ours, offsets.ours_len),
                self.theirs
                    .in_region_children(theirs_node, offsets.theirs, offsets.theirs_len),
            ) else {
                continue;
            };

            let base_keys: HashSet<&str> =
                base_items.iter().map(|i| i.key.as_str()).collect();
            let ours_added: HashSet<String> = ours_items
                .iter()
                .filter_map(|i| {
                    if base_keys.contains(i.key.as_str()) {
                        None
                    } else {
                        Some(i.key.clone())
                    }
                })
                .collect();
            let theirs_added: HashSet<String> = theirs_items
                .iter()
                .filter_map(|i| {
                    if base_keys.contains(i.key.as_str()) {
                        None
                    } else {
                        Some(i.key.clone())
                    }
                })
                .collect();
            ours_added_by_region.insert(conflict.range.start, ours_added);
            theirs_added_by_region.insert(conflict.range.start, theirs_added);
        }

        // For each region R1, check if any key it adds (on either side) is
        // also added (on the OPPOSITE side) by some OTHER region.
        let mut collisions: HashMap<Anchor, DeferReason> = HashMap::default();
        for conflict in conflicts {
            let anchor = conflict.range.start;
            let Some(ours_added) = ours_added_by_region.get(&anchor) else {
                continue;
            };
            for key in ours_added {
                for (&other_anchor, theirs_added) in &theirs_added_by_region {
                    if other_anchor == anchor {
                        continue;
                    }
                    if theirs_added.contains(key) {
                        collisions
                            .entry(anchor)
                            .or_insert(DeferReason::CrossRegionKeyCollision {
                                key: key.clone(),
                                added_by: ConflictSide::Ours,
                                other_region: other_anchor,
                            });
                        collisions.entry(other_anchor).or_insert(
                            DeferReason::CrossRegionKeyCollision {
                                key: key.clone(),
                                added_by: ConflictSide::Theirs,
                                other_region: anchor,
                            },
                        );
                    }
                }
            }
        }
        self.cross_region_defer = collisions;
    }
}

struct MergesCaptureIndices {
    set: Option<u32>,
    ordered: Option<u32>,
    key: Option<u32>,
    key_normalized: Option<u32>,
}

fn build_side(
    parser: &mut Parser,
    source: String,
    query: &Query,
    captures: &MergesCaptureIndices,
    prior: Option<&Tree>,
) -> Option<SideContext> {
    let tree = parser.parse(&source, prior)?;
    let (merge_set_nodes, ordered_list_nodes, item_keys) =
        collect_set_and_keys(query, &tree, &source, captures);
    Some(SideContext {
        source,
        tree,
        merge_set_nodes,
        ordered_list_nodes,
        item_keys,
    })
}

#[derive(Copy, Clone)]
enum Side {
    Ours,
    Theirs,
}

/// Apply `InputEdit`s to `tree` (the prior tree, originally parsed from
/// `base_source`) so that tree-sitter sees the byte differences that will
/// turn `base_source` into the `Side`'s reconstructed source. After this
/// runs, calling `parser.parse(new_source, Some(&tree))` does incremental
/// reparsing instead of a full re-parse.
fn apply_side_edits(
    tree: &mut Tree,
    conflicts: &[ConflictRegion],
    region_offsets: &HashMap<Anchor, RegionOffsets>,
    buffer: &text::BufferSnapshot,
    side: Side,
    base_source: &str,
) {
    // We walk the regions in order. For each, the `start_byte` in the prior
    // tree's coordinates is the region's base offset (the tree is still in
    // base coordinates at the point of THIS edit, even though we've already
    // applied earlier edits — tree-sitter accepts subsequent edits in
    // post-previous-edit coordinates, which here means we need to track the
    // accumulated byte shift).
    let mut shift: isize = 0;
    for conflict in conflicts {
        let Some(offsets) = region_offsets.get(&conflict.range.start) else {
            continue;
        };
        let new_len = match side {
            Side::Ours => offsets.ours_len,
            Side::Theirs => offsets.theirs_len,
        };
        let side_text: String = side_text(buffer, conflict, side);
        let base_slice = &base_source[offsets.base..offsets.base + offsets.base_len];
        if new_len == offsets.base_len && base_slice == side_text {
            // No textual change in this region — no edit to apply.
            continue;
        }
        let start_byte = (offsets.base as isize + shift) as usize;
        let old_end_byte = start_byte + offsets.base_len;
        let new_end_byte = start_byte + new_len;
        let edit = tree_sitter::InputEdit {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_position: point_at_byte(base_source, offsets.base),
            old_end_position: point_at_byte(base_source, offsets.base + offsets.base_len),
            // tree-sitter only uses new_end_position as a hint; compute it by
            // walking the replacement text from the region start.
            new_end_position: point_after_replacement(base_source, offsets.base, &side_text),
        };
        tree.edit(&edit);
        shift += new_len as isize - offsets.base_len as isize;
    }
}

fn side_text(buffer: &text::BufferSnapshot, conflict: &ConflictRegion, side: Side) -> String {
    let range = match side {
        Side::Ours => conflict.ours.clone(),
        Side::Theirs => conflict.theirs.clone(),
    };
    buffer.text_for_range(range).collect()
}

fn point_at_byte(source: &str, byte_offset: usize) -> tree_sitter::Point {
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut bytes_seen = 0usize;
    for ch in source.chars() {
        if bytes_seen >= byte_offset {
            break;
        }
        let len = ch.len_utf8();
        if bytes_seen + len > byte_offset {
            break;
        }
        if ch == '\n' {
            row += 1;
            col = 0;
        } else {
            col += len;
        }
        bytes_seen += len;
    }
    tree_sitter::Point { row, column: col }
}

fn point_after_replacement(
    base_source: &str,
    base_offset: usize,
    replacement: &str,
) -> tree_sitter::Point {
    let start = point_at_byte(base_source, base_offset);
    let mut row = start.row;
    let mut col = start.column;
    for ch in replacement.chars() {
        if ch == '\n' {
            row += 1;
            col = 0;
        } else {
            col += ch.len_utf8();
        }
    }
    tree_sitter::Point { row, column: col }
}

fn collect_set_and_keys(
    query: &Query,
    tree: &Tree,
    source: &str,
    captures: &MergesCaptureIndices,
) -> (HashSet<usize>, HashSet<usize>, HashMap<usize, String>) {
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), EmptyTextProvider);
    let mut set_nodes: HashSet<usize> = HashSet::default();
    let mut ordered_nodes: HashSet<usize> = HashSet::default();
    let mut raw_key_records: Vec<(Node, String)> = Vec::new();
    let mut normalized_key_records: Vec<(Node, String)> = Vec::new();
    while let Some(m) = matches.next() {
        for capture in m.captures {
            let ix = Some(capture.index);
            if ix == captures.set {
                set_nodes.insert(capture.node.id());
            } else if ix == captures.ordered {
                ordered_nodes.insert(capture.node.id());
            } else if ix == captures.key {
                let text = source[capture.node.start_byte()..capture.node.end_byte()]
                    .trim()
                    .to_string();
                raw_key_records.push((capture.node, text));
            } else if ix == captures.key_normalized {
                let text = collapse_whitespace(
                    &source[capture.node.start_byte()..capture.node.end_byte()],
                );
                normalized_key_records.push((capture.node, text));
            }
        }
    }
    let mut item_keys: HashMap<usize, String> = HashMap::default();
    let mut assign_key = |key_node: Node, key_text: String| {
        let mut current = key_node;
        while let Some(parent) = current.parent() {
            if set_nodes.contains(&parent.id()) || ordered_nodes.contains(&parent.id()) {
                // Normalized key takes precedence: we insert raw first, then
                // overwrite from normalized records.
                item_keys.insert(current.id(), key_text);
                break;
            }
            current = parent;
        }
    };
    for (node, text) in raw_key_records {
        assign_key(node, text);
    }
    for (node, text) in normalized_key_records {
        assign_key(node, text);
    }
    (set_nodes, ordered_nodes, item_keys)
}

fn collapse_whitespace(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last_was_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !last_was_space && !out.is_empty() {
                out.push(' ');
            }
            last_was_space = true;
        } else {
            out.push(ch);
            last_was_space = false;
        }
    }
    while out.ends_with(' ') {
        out.pop();
    }
    out
}

#[derive(Debug, Clone)]
struct InRegionItem {
    key: String,
    start: usize,
    end: usize,
    node_start: usize,
    node_end: usize,
}

impl InRegionItem {
    fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end]
    }

    fn normalized_text<'a>(&self, source: &'a str) -> &'a str {
        source[self.node_start..self.node_end].trim_end()
    }
}

fn items_by_key(items: &[InRegionItem]) -> Result<HashMap<&str, &InRegionItem>, String> {
    let mut map: HashMap<&str, &InRegionItem> = HashMap::default();
    for item in items {
        if map.insert(item.key.as_str(), item).is_some() {
            return Err(item.key.clone());
        }
    }
    Ok(map)
}

fn in_region_children(
    container: Node<'_>,
    source: &str,
    item_keys: &HashMap<usize, String>,
    region: std::ops::Range<usize>,
) -> Option<Vec<InRegionItem>> {
    let mut cursor = container.walk();
    let children: Vec<Node> = container.named_children(&mut cursor).collect();
    let next_sibling_starts: Vec<usize> = children
        .iter()
        .enumerate()
        .map(|(ix, _)| {
            children
                .get(ix + 1)
                .map(|n| n.start_byte())
                .unwrap_or_else(|| container.end_byte())
        })
        .collect();
    let mut items = Vec::new();
    for (child, next_start) in children.iter().zip(next_sibling_starts.iter()) {
        let child_start = child.start_byte();
        let child_end = child.end_byte();
        if child_end <= region.start || child_start >= region.end {
            continue;
        }
        if child_start < region.start || child_end > region.end {
            return None;
        }
        let extended_end = (*next_start).min(region.end);
        let key = match item_keys.get(&child.id()) {
            Some(k) => k.clone(),
            None => source[child_start..child_end].trim().to_string(),
        };
        if key.is_empty() {
            continue;
        }
        items.push(InRegionItem {
            key,
            start: child_start,
            end: extended_end,
            node_start: child_start,
            node_end: child_end,
        });
    }
    Some(items)
}

fn enclosing_merge_node<'tree>(
    tree: &'tree Tree,
    merge_node_ids: &HashSet<usize>,
    offset: usize,
    len: usize,
) -> Option<Node<'tree>> {
    let probe_end = offset + len.max(1);
    let mut node = tree
        .root_node()
        .descendant_for_byte_range(offset, probe_end)?;
    loop {
        if merge_node_ids.contains(&node.id()) {
            return Some(node);
        }
        node = node.parent()?;
    }
}

fn enclosing_container<'tree>(
    side: &'tree SideContext,
    offset: usize,
    len: usize,
) -> Option<(Node<'tree>, ContainerKind)> {
    let probe_end = offset + len.max(1);
    let mut node = side
        .tree
        .root_node()
        .descendant_for_byte_range(offset, probe_end)?;
    loop {
        if side.merge_set_nodes.contains(&node.id()) {
            return Some((node, ContainerKind::Set));
        }
        if side.ordered_list_nodes.contains(&node.id()) {
            return Some((node, ContainerKind::OrderedList));
        }
        node = node.parent()?;
    }
}

fn item_diff(base: &[&str], side: &[&str]) -> Vec<(std::ops::Range<u32>, std::ops::Range<u32>)> {
    use imara_diff::{Algorithm, diff, intern::InternedInput};
    let mut input: InternedInput<&str> = InternedInput::default();
    input.update_before(base.iter().copied());
    input.update_after(side.iter().copied());
    let mut hunks = Vec::new();
    diff(
        Algorithm::Histogram,
        &input,
        |before: std::ops::Range<u32>, after: std::ops::Range<u32>| {
            hunks.push((before, after));
        },
    );
    hunks
}

#[allow(clippy::too_many_arguments)]
fn compose_side_text(
    base_items: &[InRegionItem],
    side_items: &[InRegionItem],
    side_hunks: &[(std::ops::Range<u32>, std::ops::Range<u32>)],
    cluster_hunk_indices: &[usize],
    cluster_start: usize,
    cluster_end: usize,
    base_source: &str,
    side_source: &str,
) -> String {
    let mut result = String::new();
    let mut cursor = cluster_start;
    for &i in cluster_hunk_indices {
        let (base_range, side_range) = &side_hunks[i];
        let base_start = base_range.start as usize;
        let base_end = base_range.end as usize;
        if base_start > cursor {
            for item in &base_items[cursor..base_start] {
                push_with_newline(&mut result, item.text(base_source));
            }
        }
        for item in &side_items[side_range.start as usize..side_range.end as usize] {
            push_with_newline(&mut result, item.text(side_source));
        }
        cursor = base_end;
    }
    if cursor < cluster_end {
        for item in &base_items[cursor..cluster_end] {
            push_with_newline(&mut result, item.text(base_source));
        }
    }
    result
}

fn push_with_newline(buffer: &mut String, text: &str) {
    buffer.push_str(text);
    if !buffer.is_empty() && !buffer.ends_with('\n') {
        buffer.push('\n');
    }
}

struct EmptyTextProvider;
impl<'a> tree_sitter::TextProvider<&'a [u8]> for EmptyTextProvider {
    type I = std::iter::Empty<&'a [u8]>;
    fn text(&mut self, _node: Node) -> Self::I {
        std::iter::empty()
    }
}
