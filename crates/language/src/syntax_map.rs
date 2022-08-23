use crate::{
    Grammar, InjectionConfig, Language, LanguageRegistry, QueryCursorHandle, TextProvider,
    ToTreeSitterPoint,
};
use std::{
    borrow::Cow, cell::RefCell, cmp::Ordering, collections::BinaryHeap, ops::Range, sync::Arc,
};
use sum_tree::{Bias, SeekTarget, SumTree};
use text::{Anchor, BufferSnapshot, OffsetRangeExt, Point, Rope, ToOffset, ToPoint};
use tree_sitter::{Node, Parser, Tree};

thread_local! {
    static PARSER: RefCell<Parser> = RefCell::new(Parser::new());
}

#[derive(Default)]
pub struct SyntaxMap {
    parsed_version: clock::Global,
    interpolated_version: clock::Global,
    snapshot: SyntaxSnapshot,
    language_registry: Option<Arc<LanguageRegistry>>,
}

#[derive(Clone, Default)]
pub struct SyntaxSnapshot {
    layers: SumTree<SyntaxLayer>,
}

#[derive(Clone)]
struct SyntaxLayer {
    depth: usize,
    range: Range<Anchor>,
    tree: tree_sitter::Tree,
    language: Arc<Language>,
}

#[derive(Debug, Clone)]
struct SyntaxLayerSummary {
    max_depth: usize,
    range: Range<Anchor>,
    last_layer_range: Range<Anchor>,
}

#[derive(Clone, Debug)]
struct DepthAndRange(usize, Range<Anchor>);

#[derive(Clone, Debug)]
struct DepthAndMaxPosition(usize, Anchor);

#[derive(Clone, Debug)]
struct DepthAndRangeOrMaxPosition(DepthAndRange, DepthAndMaxPosition);

struct ReparseStep {
    depth: usize,
    language: Arc<Language>,
    ranges: Vec<tree_sitter::Range>,
    range: Range<Anchor>,
}

#[derive(Debug, PartialEq, Eq)]
struct ChangedRegion {
    depth: usize,
    range: Range<Anchor>,
}

#[derive(Default)]
struct ChangeRegionSet(Vec<ChangedRegion>);

impl SyntaxMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_language_registry(&mut self, registry: Arc<LanguageRegistry>) {
        self.language_registry = Some(registry);
    }

    pub fn snapshot(&self) -> SyntaxSnapshot {
        self.snapshot.clone()
    }

    pub fn interpolate(&mut self, text: &BufferSnapshot) {
        self.snapshot.interpolate(&self.interpolated_version, text);
        self.interpolated_version = text.version.clone();
    }

    pub fn reparse(&mut self, language: Arc<Language>, text: &BufferSnapshot) {
        if !self.interpolated_version.observed_all(&text.version) {
            self.interpolate(text);
        }

        self.snapshot.reparse(
            &self.parsed_version,
            text,
            self.language_registry.clone(),
            language,
        );
        self.parsed_version = text.version.clone();
    }
}

impl SyntaxSnapshot {
    pub fn interpolate(&mut self, from_version: &clock::Global, text: &BufferSnapshot) {
        let edits = text
            .edits_since::<(usize, Point)>(&from_version)
            .collect::<Vec<_>>();
        if edits.is_empty() {
            return;
        }

        let mut layers = SumTree::new();
        let mut edits_for_depth = &edits[..];
        let mut cursor = self.layers.cursor::<SyntaxLayerSummary>();
        cursor.next(text);

        'outer: loop {
            let depth = cursor.end(text).max_depth;

            // Preserve any layers at this depth that precede the first edit.
            if let Some(first_edit) = edits_for_depth.first() {
                let target = DepthAndMaxPosition(depth, text.anchor_before(first_edit.new.start.0));
                if target.cmp(&cursor.start(), text).is_gt() {
                    let slice = cursor.slice(&target, Bias::Left, text);
                    layers.push_tree(slice, text);
                }
            }
            // If this layer follows all of the edits, then preserve it and any
            // subsequent layers at this same depth.
            else {
                let slice = cursor.slice(
                    &DepthAndRange(depth + 1, Anchor::MIN..Anchor::MAX),
                    Bias::Left,
                    text,
                );
                layers.push_tree(slice, text);
                edits_for_depth = &edits[..];
                continue;
            };

            let layer = if let Some(layer) = cursor.item() {
                layer
            } else {
                break;
            };

            let mut endpoints = text
                .summaries_for_anchors::<(usize, Point), _>([&layer.range.start, &layer.range.end]);
            let layer_range = endpoints.next().unwrap()..endpoints.next().unwrap();
            let start_byte = layer_range.start.0;
            let start_point = layer_range.start.1;
            let end_byte = layer_range.end.0;

            // Ignore edits that end before the start of this layer, and don't consider them
            // for any subsequent layers at this same depth.
            loop {
                if let Some(edit) = edits_for_depth.first() {
                    if edit.new.end.0 < start_byte {
                        edits_for_depth = &edits_for_depth[1..];
                    } else {
                        break;
                    }
                } else {
                    continue 'outer;
                }
            }

            let mut layer = layer.clone();
            for edit in edits_for_depth {
                // Ignore any edits that follow this layer.
                if edit.new.start.0 > end_byte {
                    break;
                }

                // Apply any edits that intersect this layer to the layer's syntax tree.
                let tree_edit = if edit.new.start.0 >= start_byte {
                    tree_sitter::InputEdit {
                        start_byte: edit.new.start.0 - start_byte,
                        old_end_byte: edit.new.start.0 - start_byte
                            + (edit.old.end.0 - edit.old.start.0),
                        new_end_byte: edit.new.end.0 - start_byte,
                        start_position: (edit.new.start.1 - start_point).to_ts_point(),
                        old_end_position: (edit.new.start.1 - start_point
                            + (edit.old.end.1 - edit.old.start.1))
                            .to_ts_point(),
                        new_end_position: (edit.new.end.1 - start_point).to_ts_point(),
                    }
                } else {
                    tree_sitter::InputEdit {
                        start_byte: 0,
                        old_end_byte: edit.new.end.0 - start_byte,
                        new_end_byte: 0,
                        start_position: Default::default(),
                        old_end_position: (edit.new.end.1 - start_point).to_ts_point(),
                        new_end_position: Default::default(),
                    }
                };

                layer.tree.edit(&tree_edit);
                if edit.new.start.0 < start_byte {
                    break;
                }
            }

            layers.push(layer, text);
            cursor.next(text);
        }

        layers.push_tree(cursor.suffix(&text), &text);
        drop(cursor);
        self.layers = layers;
    }

    pub fn reparse(
        &mut self,
        from_version: &clock::Global,
        text: &BufferSnapshot,
        registry: Option<Arc<LanguageRegistry>>,
        language: Arc<Language>,
    ) {
        let edits = text.edits_since::<usize>(from_version).collect::<Vec<_>>();
        if edits.is_empty() {
            return;
        }

        let max_depth = self.layers.summary().max_depth;
        let mut cursor = self.layers.cursor::<SyntaxLayerSummary>();
        cursor.next(&text);
        let mut layers = SumTree::new();

        let mut changed_regions = ChangeRegionSet::default();
        let mut queue = BinaryHeap::new();
        queue.push(ReparseStep {
            depth: 0,
            language: language.clone(),
            ranges: Vec::new(),
            range: Anchor::MIN..Anchor::MAX,
        });

        loop {
            let step = queue.pop();
            let (depth, range) = if let Some(step) = &step {
                (step.depth, step.range.clone())
            } else {
                (max_depth + 1, Anchor::MAX..Anchor::MAX)
            };

            let target = DepthAndRange(depth, range.clone());
            let mut done = cursor.item().is_none();
            while !done && target.cmp(&cursor.end(text), &text).is_gt() {
                done = true;

                let bounded_target =
                    DepthAndRangeOrMaxPosition(target.clone(), changed_regions.start_position());
                if bounded_target.cmp(&cursor.start(), &text).is_gt() {
                    let slice = cursor.slice(&bounded_target, Bias::Left, text);
                    if !slice.is_empty() {
                        layers.push_tree(slice, &text);
                        if changed_regions.prune(cursor.end(text), text) {
                            done = false;
                        }
                    }
                }

                while target.cmp(&cursor.end(text), text).is_gt() {
                    let layer = if let Some(layer) = cursor.item() {
                        layer
                    } else {
                        break;
                    };

                    if changed_regions.intersects(&layer, text) {
                        changed_regions.insert(
                            ChangedRegion {
                                depth: depth + 1,
                                range: layer.range.clone(),
                            },
                            text,
                        );
                    } else {
                        layers.push(layer.clone(), text);
                    }

                    cursor.next(text);
                    if changed_regions.prune(cursor.end(text), text) {
                        done = false;
                    }
                }
            }

            let (ranges, language) = if let Some(step) = step {
                (step.ranges, step.language)
            } else {
                break;
            };

            let start_point;
            let start_byte;
            let end_byte;
            if let Some((first, last)) = ranges.first().zip(ranges.last()) {
                start_point = first.start_point;
                start_byte = first.start_byte;
                end_byte = last.end_byte;
            } else {
                start_point = Point::zero().to_ts_point();
                start_byte = 0;
                end_byte = text.len();
            };

            let mut old_layer = cursor.item();
            if let Some(layer) = old_layer {
                if layer.range.to_offset(text) == (start_byte..end_byte) {
                    cursor.next(&text);
                } else {
                    old_layer = None;
                }
            }

            let grammar = if let Some(grammar) = language.grammar.as_deref() {
                grammar
            } else {
                continue;
            };

            let tree;
            let changed_ranges;
            if let Some(old_layer) = old_layer {
                tree = parse_text(
                    grammar,
                    text.as_rope(),
                    Some(old_layer.tree.clone()),
                    ranges,
                );
                changed_ranges = join_ranges(
                    edits
                        .iter()
                        .map(|e| e.new.clone())
                        .filter(|range| range.start < end_byte && range.end > start_byte),
                    old_layer
                        .tree
                        .changed_ranges(&tree)
                        .map(|r| start_byte + r.start_byte..start_byte + r.end_byte),
                );
            } else {
                tree = parse_text(grammar, text.as_rope(), None, ranges);
                changed_ranges = vec![start_byte..end_byte];
            }

            layers.push(
                SyntaxLayer {
                    depth,
                    range,
                    tree: tree.clone(),
                    language: language.clone(),
                },
                &text,
            );

            if let (Some((config, registry)), false) = (
                grammar.injection_config.as_ref().zip(registry.as_ref()),
                changed_ranges.is_empty(),
            ) {
                let depth = depth + 1;
                for range in &changed_ranges {
                    changed_regions.insert(
                        ChangedRegion {
                            depth,
                            range: text.anchor_before(range.start)..text.anchor_after(range.end),
                        },
                        text,
                    );
                }
                get_injections(
                    config,
                    text,
                    tree.root_node_with_offset(start_byte, start_point),
                    registry,
                    depth,
                    &changed_ranges,
                    &mut queue,
                );
            }
        }

        drop(cursor);
        self.layers = layers;
    }

    pub fn layers(&self, buffer: &BufferSnapshot) -> Vec<(&Grammar, Node)> {
        self.layers
            .iter()
            .filter_map(|layer| {
                if let Some(grammar) = &layer.language.grammar {
                    Some((
                        grammar.as_ref(),
                        layer.tree.root_node_with_offset(
                            layer.range.start.to_offset(buffer),
                            layer.range.start.to_point(buffer).to_ts_point(),
                        ),
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn layers_for_range<'a, T: ToOffset>(
        &self,
        range: Range<T>,
        buffer: &BufferSnapshot,
    ) -> Vec<(&Grammar, Node)> {
        let start = buffer.anchor_before(range.start.to_offset(buffer));
        let end = buffer.anchor_after(range.end.to_offset(buffer));

        let mut cursor = self.layers.filter::<_, ()>(|summary| {
            let is_before_start = summary.range.end.cmp(&start, buffer).is_lt();
            let is_after_end = summary.range.start.cmp(&end, buffer).is_gt();
            !is_before_start && !is_after_end
        });

        let mut result = Vec::new();
        cursor.next(buffer);
        while let Some(layer) = cursor.item() {
            if let Some(grammar) = &layer.language.grammar {
                result.push((
                    grammar.as_ref(),
                    layer.tree.root_node_with_offset(
                        layer.range.start.to_offset(buffer),
                        layer.range.start.to_point(buffer).to_ts_point(),
                    ),
                ));
            }
            cursor.next(buffer)
        }

        result
    }
}

fn join_ranges(
    a: impl Iterator<Item = Range<usize>>,
    b: impl Iterator<Item = Range<usize>>,
) -> Vec<Range<usize>> {
    let mut result = Vec::<Range<usize>>::new();
    let mut a = a.peekable();
    let mut b = b.peekable();
    loop {
        let range = match (a.peek(), b.peek()) {
            (Some(range_a), Some(range_b)) => {
                if range_a.start < range_b.start {
                    a.next().unwrap()
                } else {
                    b.next().unwrap()
                }
            }
            (None, Some(_)) => b.next().unwrap(),
            (Some(_), None) => a.next().unwrap(),
            (None, None) => break,
        };

        if let Some(last) = result.last_mut() {
            if range.start <= last.end {
                last.end = last.end.max(range.end);
                continue;
            }
        }
        result.push(range);
    }
    result
}

fn parse_text(
    grammar: &Grammar,
    text: &Rope,
    old_tree: Option<Tree>,
    mut ranges: Vec<tree_sitter::Range>,
) -> Tree {
    let (start_byte, start_point) = ranges
        .first()
        .map(|range| (range.start_byte, Point::from_ts_point(range.start_point)))
        .unwrap_or_default();

    for range in &mut ranges {
        range.start_byte -= start_byte;
        range.end_byte -= start_byte;
        range.start_point = (Point::from_ts_point(range.start_point) - start_point).to_ts_point();
        range.end_point = (Point::from_ts_point(range.end_point) - start_point).to_ts_point();
    }

    PARSER.with(|parser| {
        let mut parser = parser.borrow_mut();
        let mut chunks = text.chunks_in_range(start_byte..text.len());
        parser
            .set_included_ranges(&ranges)
            .expect("overlapping ranges");
        parser
            .set_language(grammar.ts_language)
            .expect("incompatible grammar");
        parser
            .parse_with(
                &mut move |offset, _| {
                    chunks.seek(start_byte + offset);
                    chunks.next().unwrap_or("").as_bytes()
                },
                old_tree.as_ref(),
            )
            .expect("invalid language")
    })
}

fn get_injections(
    config: &InjectionConfig,
    text: &BufferSnapshot,
    node: Node,
    language_registry: &LanguageRegistry,
    depth: usize,
    query_ranges: &[Range<usize>],
    queue: &mut BinaryHeap<ReparseStep>,
) -> bool {
    let mut result = false;
    let mut query_cursor = QueryCursorHandle::new();
    let mut prev_match = None;
    for query_range in query_ranges {
        query_cursor.set_byte_range(query_range.start..query_range.end);
        for mat in query_cursor.matches(&config.query, node, TextProvider(text.as_rope())) {
            let content_ranges = mat
                .nodes_for_capture_index(config.content_capture_ix)
                .map(|node| node.range())
                .collect::<Vec<_>>();
            if content_ranges.is_empty() {
                continue;
            }

            // Avoid duplicate matches if two changed ranges intersect the same injection.
            let content_range =
                content_ranges.first().unwrap().start_byte..content_ranges.last().unwrap().end_byte;
            if let Some((last_pattern_ix, last_range)) = &prev_match {
                if mat.pattern_index == *last_pattern_ix && content_range == *last_range {
                    continue;
                }
            }
            prev_match = Some((mat.pattern_index, content_range.clone()));

            let language_name = config.languages_by_pattern_ix[mat.pattern_index]
                .as_ref()
                .map(|s| Cow::Borrowed(s.as_ref()))
                .or_else(|| {
                    let ix = config.language_capture_ix?;
                    let node = mat.nodes_for_capture_index(ix).next()?;
                    Some(Cow::Owned(text.text_for_range(node.byte_range()).collect()))
                });

            if let Some(language_name) = language_name {
                if let Some(language) = language_registry.get_language(language_name.as_ref()) {
                    result = true;
                    let range = text.anchor_before(content_range.start)
                        ..text.anchor_after(content_range.end);
                    queue.push(ReparseStep {
                        depth,
                        language,
                        ranges: content_ranges,
                        range,
                    })
                }
            }
        }
    }
    result
}

impl std::ops::Deref for SyntaxMap {
    type Target = SyntaxSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl PartialEq for ReparseStep {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Eq for ReparseStep {}

impl PartialOrd for ReparseStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for ReparseStep {
    fn cmp(&self, other: &Self) -> Ordering {
        let range_a = self.range();
        let range_b = other.range();
        Ord::cmp(&other.depth, &self.depth)
            .then_with(|| Ord::cmp(&range_b.start, &range_a.start))
            .then_with(|| Ord::cmp(&range_a.end, &range_b.end))
    }
}

impl ReparseStep {
    fn range(&self) -> Range<usize> {
        let start = self.ranges.first().map_or(0, |r| r.start_byte);
        let end = self.ranges.last().map_or(0, |r| r.end_byte);
        start..end
    }
}

impl ChangedRegion {
    fn cmp(&self, other: &Self, buffer: &BufferSnapshot) -> Ordering {
        let range_a = &self.range;
        let range_b = &other.range;
        Ord::cmp(&self.depth, &other.depth)
            .then_with(|| range_a.start.cmp(&range_b.start, buffer))
            .then_with(|| range_b.end.cmp(&range_a.end, buffer))
    }
}

impl ChangeRegionSet {
    fn start_position(&self) -> DepthAndMaxPosition {
        self.0
            .first()
            .map_or(DepthAndMaxPosition(usize::MAX, Anchor::MAX), |region| {
                DepthAndMaxPosition(region.depth, region.range.start)
            })
    }

    fn intersects(&self, layer: &SyntaxLayer, text: &BufferSnapshot) -> bool {
        for region in &self.0 {
            if region.depth < layer.depth {
                continue;
            }
            if region.depth > layer.depth {
                break;
            }
            if region.range.end.cmp(&layer.range.start, text).is_le() {
                continue;
            }
            if region.range.start.cmp(&layer.range.end, text).is_ge() {
                break;
            }
            return true;
        }
        false
    }

    fn insert(&mut self, region: ChangedRegion, text: &BufferSnapshot) {
        if let Err(ix) = self.0.binary_search_by(|probe| probe.cmp(&region, text)) {
            self.0.insert(ix, region);
        }
    }

    fn prune(&mut self, summary: SyntaxLayerSummary, text: &BufferSnapshot) -> bool {
        let prev_len = self.0.len();
        self.0.retain(|region| {
            region.depth > summary.max_depth
                || (region.depth == summary.max_depth
                    && region
                        .range
                        .end
                        .cmp(&summary.last_layer_range.start, text)
                        .is_gt())
        });
        self.0.len() < prev_len
    }
}

impl Default for SyntaxLayerSummary {
    fn default() -> Self {
        Self {
            max_depth: 0,
            range: Anchor::MAX..Anchor::MIN,
            last_layer_range: Anchor::MIN..Anchor::MAX,
        }
    }
}

impl sum_tree::Summary for SyntaxLayerSummary {
    type Context = BufferSnapshot;

    fn add_summary(&mut self, other: &Self, buffer: &Self::Context) {
        if other.max_depth > self.max_depth {
            *self = other.clone();
        } else {
            if other.range.start.cmp(&self.range.start, buffer).is_lt() {
                self.range.start = other.range.start;
            }
            if other.range.end.cmp(&self.range.end, buffer).is_gt() {
                self.range.end = other.range.end;
            }
            self.last_layer_range = other.last_layer_range.clone();
        }
    }
}

impl<'a> SeekTarget<'a, SyntaxLayerSummary, SyntaxLayerSummary> for DepthAndRange {
    fn cmp(&self, cursor_location: &SyntaxLayerSummary, buffer: &BufferSnapshot) -> Ordering {
        Ord::cmp(&self.0, &cursor_location.max_depth)
            .then_with(|| {
                self.1
                    .start
                    .cmp(&cursor_location.last_layer_range.start, buffer)
            })
            .then_with(|| {
                cursor_location
                    .last_layer_range
                    .end
                    .cmp(&self.1.end, buffer)
            })
    }
}

impl<'a> SeekTarget<'a, SyntaxLayerSummary, SyntaxLayerSummary> for DepthAndMaxPosition {
    fn cmp(&self, cursor_location: &SyntaxLayerSummary, text: &BufferSnapshot) -> Ordering {
        Ord::cmp(&self.0, &cursor_location.max_depth)
            .then_with(|| self.1.cmp(&cursor_location.range.end, text))
    }
}

impl<'a> SeekTarget<'a, SyntaxLayerSummary, SyntaxLayerSummary> for DepthAndRangeOrMaxPosition {
    fn cmp(&self, cursor_location: &SyntaxLayerSummary, buffer: &BufferSnapshot) -> Ordering {
        if self.1.cmp(cursor_location, buffer).is_le() {
            return Ordering::Less;
        } else {
            self.0.cmp(cursor_location, buffer)
        }
    }
}

impl sum_tree::Item for SyntaxLayer {
    type Summary = SyntaxLayerSummary;

    fn summary(&self) -> Self::Summary {
        SyntaxLayerSummary {
            max_depth: self.depth,
            range: self.range.clone(),
            last_layer_range: self.range.clone(),
        }
    }
}

impl std::fmt::Debug for SyntaxLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxLayer")
            .field("depth", &self.depth)
            .field("range", &self.range)
            .field("tree", &self.tree)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LanguageConfig;
    use text::{Buffer, Point};
    use tree_sitter::Query;
    use unindent::Unindent as _;
    use util::test::marked_text_ranges;

    #[gpui::test]
    fn test_syntax_map_layers_for_range() {
        let registry = Arc::new(LanguageRegistry::test());
        let language = Arc::new(rust_lang());
        registry.add(language.clone());

        let mut buffer = Buffer::new(
            0,
            0,
            r#"
                fn a() {
                    assert_eq!(
                        b(vec![C {}]),
                        vec![d.e],
                    );
                    println!("{}", f(|_| true));
                }
            "#
            .unindent(),
        );

        let mut syntax_map = SyntaxMap::new();
        syntax_map.set_language_registry(registry.clone());
        syntax_map.reparse(language.clone(), &buffer);

        assert_layers_for_range(
            &syntax_map,
            &buffer,
            Point::new(2, 0)..Point::new(2, 0),
            &[
                "...(function_item ... (block (expression_statement (macro_invocation...",
                "...(tuple_expression (call_expression ... arguments: (arguments (macro_invocation...",
            ],
        );
        assert_layers_for_range(
            &syntax_map,
            &buffer,
            Point::new(2, 14)..Point::new(2, 16),
            &[
                "...(function_item ...",
                "...(tuple_expression (call_expression ... arguments: (arguments (macro_invocation...",
                "...(array_expression (struct_expression ...",
            ],
        );
        assert_layers_for_range(
            &syntax_map,
            &buffer,
            Point::new(3, 14)..Point::new(3, 16),
            &[
                "...(function_item ...",
                "...(tuple_expression (call_expression ... arguments: (arguments (macro_invocation...",
                "...(array_expression (field_expression ...",
            ],
        );
        assert_layers_for_range(
            &syntax_map,
            &buffer,
            Point::new(5, 12)..Point::new(5, 16),
            &[
                "...(function_item ...",
                "...(call_expression ... (arguments (closure_expression ...",
            ],
        );

        // Replace a vec! macro invocation with a plain slice, removing a syntactic layer.
        let macro_name_range = range_for_text(&buffer, "vec!");
        buffer.edit([(macro_name_range, "&")]);
        syntax_map.interpolate(&buffer);
        syntax_map.reparse(language.clone(), &buffer);

        assert_layers_for_range(
            &syntax_map,
            &buffer,
            Point::new(2, 14)..Point::new(2, 16),
            &[
                "...(function_item ...",
                "...(tuple_expression (call_expression ... arguments: (arguments (reference_expression value: (array_expression...",
            ],
        );

        // Put the vec! macro back, adding back the syntactic layer.
        buffer.undo();
        syntax_map.interpolate(&buffer);
        syntax_map.reparse(language.clone(), &buffer);

        assert_layers_for_range(
            &syntax_map,
            &buffer,
            Point::new(2, 14)..Point::new(2, 16),
            &[
                "...(function_item ...",
                "...(tuple_expression (call_expression ... arguments: (arguments (macro_invocation...",
                "...(array_expression (struct_expression ...",
            ],
        );
    }

    #[gpui::test]
    fn test_typing_multiple_new_injections() {
        let (buffer, syntax_map) = test_edit_sequence(&[
            "fn a() { dbg }",
            "fn a() { dbg«!» }",
            "fn a() { dbg!«()» }",
            "fn a() { dbg!(«b») }",
            "fn a() { dbg!(b«.») }",
            "fn a() { dbg!(b.«c») }",
            "fn a() { dbg!(b.c«()») }",
            "fn a() { dbg!(b.c(«vec»)) }",
            "fn a() { dbg!(b.c(vec«!»)) }",
            "fn a() { dbg!(b.c(vec!«[]»)) }",
            "fn a() { dbg!(b.c(vec![«d»])) }",
            "fn a() { dbg!(b.c(vec![d«.»])) }",
            "fn a() { dbg!(b.c(vec![d.«e»])) }",
        ]);

        assert_node_ranges(
            &syntax_map,
            &buffer,
            "(field_identifier) @_",
            "fn a() { dbg!(b.«c»(vec![d.«e»])) }",
        );
    }

    #[gpui::test]
    fn test_pasting_new_injection_line_between_others() {
        let (buffer, syntax_map) = test_edit_sequence(&[
            "
                fn a() {
                    b!(B {});
                    c!(C {});
                    d!(D {});
                    e!(E {});
                    f!(F {});
                    g!(G {});
                }
            ",
            "
                fn a() {
                    b!(B {});
                    c!(C {});
                    d!(D {});
                «    h!(H {});
                »    e!(E {});
                    f!(F {});
                    g!(G {});
                }
            ",
        ]);

        assert_node_ranges(
            &syntax_map,
            &buffer,
            "(struct_expression) @_",
            "
            fn a() {
                b!(«B {}»);
                c!(«C {}»);
                d!(«D {}»);
                h!(«H {}»);
                e!(«E {}»);
                f!(«F {}»);
                g!(«G {}»);
            }
            ",
        );
    }

    #[gpui::test]
    fn test_joining_injections_with_child_injections() {
        let (buffer, syntax_map) = test_edit_sequence(&[
            "
                fn a() {
                    b!(
                        c![one.two.three],
                        d![four.five.six],
                    );
                    e!(
                        f![seven.eight],
                    );
                }
            ",
            "
                fn a() {
                    b!(
                        c![one.two.three],
                        d![four.five.six],
                    ˇ    f![seven.eight],
                    );
                }
            ",
        ]);

        assert_node_ranges(
            &syntax_map,
            &buffer,
            "(field_identifier) @_",
            "
            fn a() {
                b!(
                    c![one.«two».«three»],
                    d![four.«five».«six»],
                    f![seven.«eight»],
                );
            }
            ",
        );
    }

    #[gpui::test]
    fn test_editing_edges_of_injection() {
        test_edit_sequence(&[
            "
                fn a() {
                    b!(c!())
                }
            ",
            "
                fn a() {
                    «d»!(c!())
                }
            ",
            "
                fn a() {
                    «e»d!(c!())
                }
            ",
            "
                fn a() {
                    ed!«[»c!()«]»
                }
            ",
        ]);
    }

    #[gpui::test]
    fn test_edits_preceding_and_intersecting_injection() {
        test_edit_sequence(&[
            //
            "const aaaaaaaaaaaa: B = c!(d(e.f));",
            "const aˇa: B = c!(d(eˇ));",
        ]);
    }

    #[gpui::test]
    fn test_non_local_changes_create_injections() {
        test_edit_sequence(&[
            "
                // a! {
                    static B: C = d;
                // }
            ",
            "
                ˇa! {
                    static B: C = d;
                ˇ}
            ",
        ]);
    }

    #[gpui::test]
    fn test_creating_many_injections_in_one_edit() {
        test_edit_sequence(&[
            "
                fn a() {
                    one(Two::three(3));
                    four(Five::six(6));
                    seven(Eight::nine(9));
                }
            ",
            "
                fn a() {
                    one«!»(Two::three(3));
                    four«!»(Five::six(6));
                    seven«!»(Eight::nine(9));
                }
            ",
            "
                fn a() {
                    one!(Two::three«!»(3));
                    four!(Five::six«!»(6));
                    seven!(Eight::nine«!»(9));
                }
            ",
        ]);
    }

    #[gpui::test]
    fn test_editing_across_injection_boundary() {
        test_edit_sequence(&[
            "
                fn one() {
                    two();
                    three!(
                        three.four,
                        five.six,
                    );
                }
            ",
            "
                fn one() {
                    two();
                    th«irty_five![»
                        three.four,
                        five.six,
                    «   seven.eight,
                    ];»
                }
            ",
        ]);
    }

    fn test_edit_sequence(steps: &[&str]) -> (Buffer, SyntaxMap) {
        let registry = Arc::new(LanguageRegistry::test());
        let language = Arc::new(rust_lang());
        registry.add(language.clone());
        let mut buffer = Buffer::new(0, 0, Default::default());

        let mut mutated_syntax_map = SyntaxMap::new();
        mutated_syntax_map.set_language_registry(registry.clone());
        mutated_syntax_map.reparse(language.clone(), &buffer);

        for (i, marked_string) in steps.into_iter().enumerate() {
            edit_buffer(&mut buffer, &marked_string.unindent());

            // Reparse the syntax map
            mutated_syntax_map.interpolate(&buffer);
            mutated_syntax_map.reparse(language.clone(), &buffer);

            // Create a second syntax map from scratch
            let mut reference_syntax_map = SyntaxMap::new();
            reference_syntax_map.set_language_registry(registry.clone());
            reference_syntax_map.reparse(language.clone(), &buffer);

            // Compare the mutated syntax map to the new syntax map
            let mutated_layers = mutated_syntax_map.layers(&buffer);
            let reference_layers = reference_syntax_map.layers(&buffer);
            assert_eq!(
                mutated_layers.len(),
                reference_layers.len(),
                "wrong number of layers at step {i}"
            );
            for (edited_layer, reference_layer) in
                mutated_layers.into_iter().zip(reference_layers.into_iter())
            {
                assert_eq!(
                    edited_layer.1.to_sexp(),
                    reference_layer.1.to_sexp(),
                    "different layer at step {i}"
                );
                assert_eq!(
                    edited_layer.1.range(),
                    reference_layer.1.range(),
                    "different layer at step {i}"
                );
            }
        }

        (buffer, mutated_syntax_map)
    }

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )
        .with_injection_query(
            r#"
                (macro_invocation
                    (token_tree) @content
                    (#set! "language" "rust"))
            "#,
        )
        .unwrap()
    }

    fn range_for_text(buffer: &Buffer, text: &str) -> Range<usize> {
        let start = buffer.as_rope().to_string().find(text).unwrap();
        start..start + text.len()
    }

    fn assert_layers_for_range(
        syntax_map: &SyntaxMap,
        buffer: &BufferSnapshot,
        range: Range<Point>,
        expected_layers: &[&str],
    ) {
        let layers = syntax_map.layers_for_range(range, &buffer);
        assert_eq!(
            layers.len(),
            expected_layers.len(),
            "wrong number of layers"
        );
        for (i, ((_, node), expected_s_exp)) in
            layers.iter().zip(expected_layers.iter()).enumerate()
        {
            let actual_s_exp = node.to_sexp();
            assert!(
                string_contains_sequence(
                    &actual_s_exp,
                    &expected_s_exp.split("...").collect::<Vec<_>>()
                ),
                "layer {i}:\n\nexpected: {expected_s_exp}\nactual:   {actual_s_exp}",
            );
        }
    }

    fn assert_node_ranges(
        syntax_map: &SyntaxMap,
        buffer: &BufferSnapshot,
        query: &str,
        marked_string: &str,
    ) {
        let mut cursor = QueryCursorHandle::new();
        let mut actual_ranges = Vec::<Range<usize>>::new();
        for (grammar, node) in syntax_map.layers(buffer) {
            let query = Query::new(grammar.ts_language, query).unwrap();
            for (mat, ix) in cursor.captures(&query, node, TextProvider(buffer.as_rope())) {
                actual_ranges.push(mat.captures[ix].node.byte_range());
            }
        }

        let (text, expected_ranges) = marked_text_ranges(&marked_string.unindent(), false);
        assert_eq!(text, buffer.text());
        assert_eq!(actual_ranges, expected_ranges);
    }

    fn edit_buffer(buffer: &mut Buffer, marked_string: &str) {
        let old_text = buffer.text();
        let (new_text, mut ranges) = marked_text_ranges(marked_string, false);
        if ranges.is_empty() {
            ranges.push(0..new_text.len());
        }

        assert_eq!(
            old_text[..ranges[0].start],
            new_text[..ranges[0].start],
            "invalid edit"
        );

        let mut delta = 0;
        let mut edits = Vec::new();
        let mut ranges = ranges.into_iter().peekable();

        while let Some(inserted_range) = ranges.next() {
            let new_start = inserted_range.start;
            let old_start = (new_start as isize - delta) as usize;

            let following_text = if let Some(next_range) = ranges.peek() {
                &new_text[inserted_range.end..next_range.start]
            } else {
                &new_text[inserted_range.end..]
            };

            let inserted_len = inserted_range.len();
            let deleted_len = old_text[old_start..]
                .find(following_text)
                .expect("invalid edit");

            let old_range = old_start..old_start + deleted_len;
            edits.push((old_range, new_text[inserted_range].to_string()));
            delta += inserted_len as isize - deleted_len as isize;
        }

        assert_eq!(
            old_text.len() as isize + delta,
            new_text.len() as isize,
            "invalid edit"
        );

        buffer.edit(edits);
    }

    pub fn string_contains_sequence(text: &str, parts: &[&str]) -> bool {
        let mut last_part_end = 0;
        for part in parts {
            if let Some(start_ix) = text[last_part_end..].find(part) {
                last_part_end = start_ix + part.len();
            } else {
                return false;
            }
        }
        true
    }
}
