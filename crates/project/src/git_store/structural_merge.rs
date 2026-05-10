use crate::git_store::conflict_set::ConflictRegion;
use collections::{HashMap, HashSet};
use language::Language;
use std::sync::Arc;
use text::{Anchor, OffsetRangeExt as _};
use tree_sitter::{Node, Parser, Query, QueryCursor, StreamingIterator as _, Tree};

/// Result of building three reconstructed views of a buffer (base / ours /
/// theirs) and parsing each with the buffer's language. The context can be
/// queried per [`ConflictRegion`] to ask "is this region a pure addition to a
/// mergeable container?" and, if so, get back the substitution text.
pub struct LanguageMergeContext {
    base_source: String,
    ours_source: String,
    theirs_source: String,
    base_tree: Tree,
    ours_tree: Tree,
    theirs_tree: Tree,
    base_merge_nodes: HashSet<usize>,
    ours_merge_nodes: HashSet<usize>,
    theirs_merge_nodes: HashSet<usize>,
    region_offsets: HashMap<Anchor, RegionOffsets>,
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
    /// Builds the three reconstructed sources and parses each, or returns
    /// `None` if the language has no grammar or `merges.scm` registered, or
    /// if any side fails to parse, or if there are no conflict regions.
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

        let base_tree = parser.parse(&base_source, None)?;
        let ours_tree = parser.parse(&ours_source, None)?;
        let theirs_tree = parser.parse(&theirs_source, None)?;

        let set_capture_ix = merges_config.set_capture_ix?;
        let base_merge_nodes = collect_merge_node_ids(&merges_config.query, &base_tree, set_capture_ix);
        let ours_merge_nodes = collect_merge_node_ids(&merges_config.query, &ours_tree, set_capture_ix);
        let theirs_merge_nodes =
            collect_merge_node_ids(&merges_config.query, &theirs_tree, set_capture_ix);

        Some(Self {
            base_source,
            ours_source,
            theirs_source,
            base_tree,
            ours_tree,
            theirs_tree,
            base_merge_nodes,
            ours_merge_nodes,
            theirs_merge_nodes,
            region_offsets,
        })
    }

    /// Try to structurally merge `region`. Returns the substitution text for
    /// the entire conflict region (including markers) when both sides are
    /// pure additions to the same mergeable container with disjoint new
    /// children. `None` means defer to line-level decomposition.
    pub fn try_merge_region(&self, region: &ConflictRegion) -> Option<String> {
        let offsets = *self.region_offsets.get(&region.range.start)?;

        let base_node =
            enclosing_merge_node(&self.base_tree, &self.base_merge_nodes, offsets.base, offsets.base_len)?;
        let ours_node =
            enclosing_merge_node(&self.ours_tree, &self.ours_merge_nodes, offsets.ours, offsets.ours_len)?;
        let theirs_node = enclosing_merge_node(
            &self.theirs_tree,
            &self.theirs_merge_nodes,
            offsets.theirs,
            offsets.theirs_len,
        )?;

        if base_node.kind_id() != ours_node.kind_id()
            || base_node.kind_id() != theirs_node.kind_id()
        {
            return None;
        }

        let base_items = in_region_children(
            base_node,
            &self.base_source,
            offsets.base..offsets.base + offsets.base_len,
        )?;
        let ours_items = in_region_children(
            ours_node,
            &self.ours_source,
            offsets.ours..offsets.ours + offsets.ours_len,
        )?;
        let theirs_items = in_region_children(
            theirs_node,
            &self.theirs_source,
            offsets.theirs..offsets.theirs + offsets.theirs_len,
        )?;

        let base_keys = multiset(base_items.iter().map(|item| item.key.clone()));
        let ours_keys = multiset(ours_items.iter().map(|item| item.key.clone()));
        let theirs_keys = multiset(theirs_items.iter().map(|item| item.key.clone()));

        if !is_subset(&base_keys, &ours_keys) || !is_subset(&base_keys, &theirs_keys) {
            return None;
        }
        let ours_added = subtract(&ours_keys, &base_keys);
        let theirs_added = subtract(&theirs_keys, &base_keys);
        if !is_disjoint(&ours_added, &theirs_added) {
            return None;
        }
        if ours_added.is_empty() && theirs_added.is_empty() {
            return None;
        }

        let ours_region_text = &self.ours_source[offsets.ours..offsets.ours + offsets.ours_len];
        let mut substitution = ours_region_text.to_string();
        if !substitution.is_empty() && !substitution.ends_with('\n') {
            substitution.push('\n');
        }
        for item in theirs_items.iter() {
            if theirs_added.contains_key(&item.key) && !ours_added.contains_key(&item.key) {
                let item_text = &self.theirs_source[item.start..item.end];
                substitution.push_str(item_text);
                if !substitution.ends_with('\n') {
                    substitution.push('\n');
                }
            }
        }
        Some(substitution)
    }
}

struct InRegionItem {
    key: String,
    start: usize,
    end: usize,
}

fn in_region_children(
    container: Node<'_>,
    source: &str,
    region: std::ops::Range<usize>,
) -> Option<Vec<InRegionItem>> {
    let mut cursor = container.walk();
    let mut items = Vec::new();
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
    for (child, next_start) in children.iter().zip(next_sibling_starts.iter()) {
        let child_start = child.start_byte();
        let child_end = child.end_byte();
        // A child counts as "in region" only if its entire byte range sits
        // inside the conflict region. Children straddling the boundary mean
        // the markers split a syntactic item; we conservatively refuse to
        // structurally merge in that case.
        if child_end <= region.start || child_start >= region.end {
            continue;
        }
        if child_start < region.start || child_end > region.end {
            return None;
        }
        let extended_end = (*next_start).min(region.end);
        let text = &source[child_start..extended_end];
        let key = text.trim().to_string();
        if key.is_empty() {
            continue;
        }
        items.push(InRegionItem {
            key,
            start: child_start,
            end: extended_end,
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

fn collect_merge_node_ids(query: &Query, tree: &Tree, capture_ix: u32) -> HashSet<usize> {
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), EmptyTextProvider);
    let mut ids = HashSet::default();
    while let Some(m) = matches.next() {
        for capture in m.captures {
            if capture.index == capture_ix {
                ids.insert(capture.node.id());
            }
        }
    }
    ids
}

struct EmptyTextProvider;
impl<'a> tree_sitter::TextProvider<&'a [u8]> for EmptyTextProvider {
    type I = std::iter::Empty<&'a [u8]>;
    fn text(&mut self, _node: Node) -> Self::I {
        std::iter::empty()
    }
}

fn multiset(keys: impl Iterator<Item = String>) -> HashMap<String, usize> {
    let mut out: HashMap<String, usize> = HashMap::default();
    for key in keys {
        *out.entry(key).or_default() += 1;
    }
    out
}

fn is_subset(small: &HashMap<String, usize>, big: &HashMap<String, usize>) -> bool {
    small
        .iter()
        .all(|(key, count)| big.get(key).copied().unwrap_or(0) >= *count)
}

fn subtract(
    big: &HashMap<String, usize>,
    small: &HashMap<String, usize>,
) -> HashMap<String, usize> {
    let mut out: HashMap<String, usize> = HashMap::default();
    for (key, count) in big {
        let remaining = count.saturating_sub(small.get(key).copied().unwrap_or(0));
        if remaining > 0 {
            out.insert(key.clone(), remaining);
        }
    }
    out
}

fn is_disjoint(a: &HashMap<String, usize>, b: &HashMap<String, usize>) -> bool {
    !a.keys().any(|key| b.contains_key(key))
}
