use crate::git_store::conflict_set::ConflictRegion;
use collections::{HashMap, HashSet};
use language::Language;
use std::sync::Arc;
use text::{Anchor, OffsetRangeExt as _};
use tree_sitter::{Node, Parser, Query, QueryCursor, StreamingIterator as _, Tree};

/// Result of building three reconstructed views of a buffer (base / ours /
/// theirs) and parsing each with the buffer's language. The context can be
/// queried per [`ConflictRegion`] to ask "is this region a structurally safe
/// merge?" and, if so, get back the substitution text.
pub struct LanguageMergeContext {
    base: SideContext,
    ours: SideContext,
    theirs: SideContext,
    region_offsets: HashMap<Anchor, RegionOffsets>,
}

struct SideContext {
    source: String,
    tree: Tree,
    merge_set_nodes: HashSet<usize>,
    item_keys: HashMap<usize, String>,
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
        let set_capture_ix = merges_config.set_capture_ix?;
        let key_capture_ix = merges_config.key_capture_ix;

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

        let base = build_side(&mut parser, base_source, &merges_config.query, set_capture_ix, key_capture_ix)?;
        let ours = build_side(&mut parser, ours_source, &merges_config.query, set_capture_ix, key_capture_ix)?;
        let theirs =
            build_side(&mut parser, theirs_source, &merges_config.query, set_capture_ix, key_capture_ix)?;

        Some(Self {
            base,
            ours,
            theirs,
            region_offsets,
        })
    }

    /// Try to structurally merge `region`. Returns the substitution text for
    /// the entire conflict region when v2 case analysis (modifications,
    /// deletions, and additions of keyed items) can resolve every difference;
    /// `None` means defer to line-level decomposition.
    pub fn try_merge_region(&self, region: &ConflictRegion) -> Option<String> {
        let offsets = *self.region_offsets.get(&region.range.start)?;

        let base_node = enclosing_merge_node(
            &self.base.tree,
            &self.base.merge_set_nodes,
            offsets.base,
            offsets.base_len,
        )?;
        let ours_node = enclosing_merge_node(
            &self.ours.tree,
            &self.ours.merge_set_nodes,
            offsets.ours,
            offsets.ours_len,
        )?;
        let theirs_node = enclosing_merge_node(
            &self.theirs.tree,
            &self.theirs.merge_set_nodes,
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
            &self.base.source,
            &self.base.item_keys,
            offsets.base..offsets.base + offsets.base_len,
        )?;
        let ours_items = in_region_children(
            ours_node,
            &self.ours.source,
            &self.ours.item_keys,
            offsets.ours..offsets.ours + offsets.ours_len,
        )?;
        let theirs_items = in_region_children(
            theirs_node,
            &self.theirs.source,
            &self.theirs.item_keys,
            offsets.theirs..offsets.theirs + offsets.theirs_len,
        )?;

        let base_by_key = items_by_key(&base_items)?;
        let theirs_by_key = items_by_key(&theirs_items)?;
        // Bail if ours has duplicate keys; we don't need a map lookup beyond that.
        let _ = items_by_key(&ours_items)?;

        let mut output = String::new();
        let mut handled: HashSet<&str> = HashSet::default();
        let mut any_change = false;

        for ours_item in &ours_items {
            let key = ours_item.key.as_str();
            handled.insert(key);
            let base_item = base_by_key.get(key).copied();
            let theirs_item = theirs_by_key.get(key).copied();

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
                        return None;
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
                                return None;
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
                        return None;
                    }
                }
            };

            if let Some(text) = text {
                push_with_newline(&mut output, text);
            }
        }

        for theirs_item in &theirs_items {
            let key = theirs_item.key.as_str();
            if handled.contains(key) {
                continue;
            }
            handled.insert(key);
            let base_item = base_by_key.get(key).copied();
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
                        return None;
                    }
                }
            }
        }

        for base_item in &base_items {
            let key = base_item.key.as_str();
            if handled.contains(key) {
                continue;
            }
            any_change = true;
        }

        if !any_change {
            return None;
        }
        Some(output)
    }
}

fn build_side(
    parser: &mut Parser,
    source: String,
    query: &Query,
    set_capture_ix: u32,
    key_capture_ix: Option<u32>,
) -> Option<SideContext> {
    let tree = parser.parse(&source, None)?;
    let (merge_set_nodes, item_keys) = collect_set_and_keys(
        query,
        &tree,
        &source,
        set_capture_ix,
        key_capture_ix,
    );
    Some(SideContext {
        source,
        tree,
        merge_set_nodes,
        item_keys,
    })
}

fn collect_set_and_keys(
    query: &Query,
    tree: &Tree,
    source: &str,
    set_capture_ix: u32,
    key_capture_ix: Option<u32>,
) -> (HashSet<usize>, HashMap<usize, String>) {
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), EmptyTextProvider);
    let mut set_nodes: HashSet<usize> = HashSet::default();
    let mut key_node_records: Vec<(Node, String)> = Vec::new();
    while let Some(m) = matches.next() {
        for capture in m.captures {
            if capture.index == set_capture_ix {
                set_nodes.insert(capture.node.id());
            } else if Some(capture.index) == key_capture_ix {
                let text = source[capture.node.start_byte()..capture.node.end_byte()]
                    .trim()
                    .to_string();
                key_node_records.push((capture.node, text));
            }
        }
    }
    let mut item_keys: HashMap<usize, String> = HashMap::default();
    for (key_node, key_text) in key_node_records {
        let mut current = key_node;
        while let Some(parent) = current.parent() {
            if set_nodes.contains(&parent.id()) {
                item_keys.entry(current.id()).or_insert(key_text);
                break;
            }
            current = parent;
        }
    }
    (set_nodes, item_keys)
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

fn items_by_key(items: &[InRegionItem]) -> Option<HashMap<&str, &InRegionItem>> {
    let mut map: HashMap<&str, &InRegionItem> = HashMap::default();
    for item in items {
        if map.insert(item.key.as_str(), item).is_some() {
            // Two items with the same key in the same container — ambiguous.
            return None;
        }
    }
    Some(map)
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
