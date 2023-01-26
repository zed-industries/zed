use crate::{Grammar, InjectionConfig, Language, LanguageRegistry};
use collections::HashMap;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::{
    borrow::Cow,
    cell::RefCell,
    cmp::{self, Ordering, Reverse},
    collections::BinaryHeap,
    iter,
    ops::{Deref, DerefMut, Range},
    sync::Arc,
};
use sum_tree::{Bias, SeekTarget, SumTree};
use text::{Anchor, BufferSnapshot, OffsetRangeExt, Point, Rope, ToOffset, ToPoint};
use tree_sitter::{
    Node, Parser, Query, QueryCapture, QueryCaptures, QueryCursor, QueryMatches, Tree,
};

thread_local! {
    static PARSER: RefCell<Parser> = RefCell::new(Parser::new());
}

lazy_static! {
    static ref QUERY_CURSORS: Mutex<Vec<QueryCursor>> = Default::default();
}

#[derive(Default)]
pub struct SyntaxMap {
    snapshot: SyntaxSnapshot,
    language_registry: Option<Arc<LanguageRegistry>>,
}

#[derive(Clone, Default)]
pub struct SyntaxSnapshot {
    layers: SumTree<SyntaxLayer>,
    parsed_version: clock::Global,
    interpolated_version: clock::Global,
    language_registry_version: usize,
}

#[derive(Default)]
pub struct SyntaxMapCaptures<'a> {
    layers: Vec<SyntaxMapCapturesLayer<'a>>,
    active_layer_count: usize,
    grammars: Vec<&'a Grammar>,
}

#[derive(Default)]
pub struct SyntaxMapMatches<'a> {
    layers: Vec<SyntaxMapMatchesLayer<'a>>,
    active_layer_count: usize,
    grammars: Vec<&'a Grammar>,
}

#[derive(Debug)]
pub struct SyntaxMapCapture<'a> {
    pub depth: usize,
    pub node: Node<'a>,
    pub index: u32,
    pub grammar_index: usize,
}

#[derive(Debug)]
pub struct SyntaxMapMatch<'a> {
    pub depth: usize,
    pub pattern_index: usize,
    pub captures: &'a [QueryCapture<'a>],
    pub grammar_index: usize,
}

struct SyntaxMapCapturesLayer<'a> {
    depth: usize,
    captures: QueryCaptures<'a, 'a, TextProvider<'a>>,
    next_capture: Option<QueryCapture<'a>>,
    grammar_index: usize,
    _query_cursor: QueryCursorHandle,
}

struct SyntaxMapMatchesLayer<'a> {
    depth: usize,
    next_pattern_index: usize,
    next_captures: Vec<QueryCapture<'a>>,
    has_next: bool,
    matches: QueryMatches<'a, 'a, TextProvider<'a>>,
    grammar_index: usize,
    _query_cursor: QueryCursorHandle,
}

#[derive(Clone)]
struct SyntaxLayer {
    depth: usize,
    range: Range<Anchor>,
    content: SyntaxLayerContent,
}

#[derive(Clone)]
enum SyntaxLayerContent {
    Parsed {
        tree: tree_sitter::Tree,
        language: Arc<Language>,
    },
    Pending {
        language_name: Arc<str>,
    },
}

impl SyntaxLayerContent {
    fn language_id(&self) -> Option<usize> {
        match self {
            SyntaxLayerContent::Parsed { language, .. } => language.id(),
            SyntaxLayerContent::Pending { .. } => None,
        }
    }

    fn tree(&self) -> Option<&Tree> {
        match self {
            SyntaxLayerContent::Parsed { tree, .. } => Some(tree),
            SyntaxLayerContent::Pending { .. } => None,
        }
    }
}

#[derive(Debug)]
pub struct SyntaxLayerInfo<'a> {
    pub depth: usize,
    pub node: Node<'a>,
    pub language: &'a Arc<Language>,
}

#[derive(Debug, Clone)]
struct SyntaxLayerSummary {
    min_depth: usize,
    max_depth: usize,
    range: Range<Anchor>,
    last_layer_range: Range<Anchor>,
    last_layer_language: Option<usize>,
    contains_unknown_injections: bool,
}

#[derive(Clone, Debug)]
struct SyntaxLayerPosition {
    depth: usize,
    range: Range<Anchor>,
    language: Option<usize>,
}

#[derive(Clone, Debug)]
struct ChangeStartPosition {
    depth: usize,
    position: Anchor,
}

#[derive(Clone, Debug)]
struct SyntaxLayerPositionBeforeChange {
    position: SyntaxLayerPosition,
    change: ChangeStartPosition,
}

struct ParseStep {
    depth: usize,
    language: ParseStepLanguage,
    range: Range<Anchor>,
    included_ranges: Vec<tree_sitter::Range>,
    mode: ParseMode,
}

enum ParseStepLanguage {
    Loaded { language: Arc<Language> },
    Pending { name: Arc<str> },
}

impl ParseStepLanguage {
    fn id(&self) -> Option<usize> {
        match self {
            ParseStepLanguage::Loaded { language } => language.id(),
            ParseStepLanguage::Pending { .. } => None,
        }
    }
}

enum ParseMode {
    Single,
    Combined {
        parent_layer_range: Range<usize>,
        parent_layer_changed_ranges: Vec<Range<usize>>,
    },
}

#[derive(Debug, PartialEq, Eq)]
struct ChangedRegion {
    depth: usize,
    range: Range<Anchor>,
}

#[derive(Default)]
struct ChangeRegionSet(Vec<ChangedRegion>);

struct TextProvider<'a>(&'a Rope);

struct ByteChunks<'a>(text::Chunks<'a>);

struct QueryCursorHandle(Option<QueryCursor>);

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

    pub fn language_registry(&self) -> Option<Arc<LanguageRegistry>> {
        self.language_registry.clone()
    }

    pub fn interpolate(&mut self, text: &BufferSnapshot) {
        self.snapshot.interpolate(text);
    }

    #[cfg(test)]
    pub fn reparse(&mut self, language: Arc<Language>, text: &BufferSnapshot) {
        self.snapshot
            .reparse(text, self.language_registry.clone(), language);
    }

    pub fn did_parse(&mut self, snapshot: SyntaxSnapshot) {
        self.snapshot = snapshot;
    }

    pub fn clear(&mut self) {
        self.snapshot = SyntaxSnapshot::default();
    }
}

impl SyntaxSnapshot {
    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }

    fn interpolate(&mut self, text: &BufferSnapshot) {
        let edits = text
            .anchored_edits_since::<(usize, Point)>(&self.interpolated_version)
            .collect::<Vec<_>>();
        self.interpolated_version = text.version().clone();

        if edits.is_empty() {
            return;
        }

        let mut layers = SumTree::new();
        let mut first_edit_ix_for_depth = 0;
        let mut prev_depth = 0;
        let mut cursor = self.layers.cursor::<SyntaxLayerSummary>();
        cursor.next(text);

        'outer: loop {
            let depth = cursor.end(text).max_depth;
            if depth > prev_depth {
                first_edit_ix_for_depth = 0;
                prev_depth = depth;
            }

            // Preserve any layers at this depth that precede the first edit.
            if let Some((_, edit_range)) = edits.get(first_edit_ix_for_depth) {
                let target = ChangeStartPosition {
                    depth,
                    position: edit_range.start,
                };
                if target.cmp(&cursor.start(), text).is_gt() {
                    let slice = cursor.slice(&target, Bias::Left, text);
                    layers.push_tree(slice, text);
                }
            }
            // If this layer follows all of the edits, then preserve it and any
            // subsequent layers at this same depth.
            else if cursor.item().is_some() {
                let slice = cursor.slice(
                    &SyntaxLayerPosition {
                        depth: depth + 1,
                        range: Anchor::MIN..Anchor::MAX,
                        language: None,
                    },
                    Bias::Left,
                    text,
                );
                layers.push_tree(slice, text);
                continue;
            };

            let Some(layer) = cursor.item() else { break };
            let (start_byte, start_point) = layer.range.start.summary::<(usize, Point)>(text);

            // Ignore edits that end before the start of this layer, and don't consider them
            // for any subsequent layers at this same depth.
            loop {
                let Some((_, edit_range)) = edits.get(first_edit_ix_for_depth) else { continue 'outer };
                if edit_range.end.cmp(&layer.range.start, text).is_le() {
                    first_edit_ix_for_depth += 1;
                } else {
                    break;
                }
            }

            let mut layer = layer.clone();
            if let SyntaxLayerContent::Parsed { tree, .. } = &mut layer.content {
                for (edit, edit_range) in &edits[first_edit_ix_for_depth..] {
                    // Ignore any edits that follow this layer.
                    if edit_range.start.cmp(&layer.range.end, text).is_ge() {
                        break;
                    }

                    // Apply any edits that intersect this layer to the layer's syntax tree.
                    let tree_edit = if edit_range.start.cmp(&layer.range.start, text).is_ge() {
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
                        let node = tree.root_node();
                        tree_sitter::InputEdit {
                            start_byte: 0,
                            old_end_byte: node.end_byte(),
                            new_end_byte: 0,
                            start_position: Default::default(),
                            old_end_position: node.end_position(),
                            new_end_position: Default::default(),
                        }
                    };

                    tree.edit(&tree_edit);
                }

                debug_assert!(
                    tree.root_node().end_byte() <= text.len(),
                    "tree's size {}, is larger than text size {}",
                    tree.root_node().end_byte(),
                    text.len(),
                );
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
        text: &BufferSnapshot,
        registry: Option<Arc<LanguageRegistry>>,
        root_language: Arc<Language>,
    ) {
        let edit_ranges = text
            .edits_since::<usize>(&self.parsed_version)
            .map(|edit| edit.new)
            .collect::<Vec<_>>();
        self.reparse_with_ranges(text, root_language.clone(), edit_ranges, registry.as_ref());

        if let Some(registry) = registry {
            if registry.version() != self.language_registry_version {
                let mut resolved_injection_ranges = Vec::new();
                let mut cursor = self
                    .layers
                    .filter::<_, ()>(|summary| summary.contains_unknown_injections);
                cursor.next(text);
                while let Some(layer) = cursor.item() {
                    let SyntaxLayerContent::Pending { language_name } = &layer.content else { unreachable!() };
                    if {
                        let language_registry = &registry;
                        language_registry.language_for_name_or_extension(language_name)
                    }
                    .is_some()
                    {
                        resolved_injection_ranges.push(layer.range.to_offset(text));
                    }

                    cursor.next(text);
                }
                drop(cursor);

                if !resolved_injection_ranges.is_empty() {
                    self.reparse_with_ranges(
                        text,
                        root_language,
                        resolved_injection_ranges,
                        Some(&registry),
                    );
                }
                self.language_registry_version = registry.version();
            }
        }
    }

    fn reparse_with_ranges(
        &mut self,
        text: &BufferSnapshot,
        root_language: Arc<Language>,
        invalidated_ranges: Vec<Range<usize>>,
        registry: Option<&Arc<LanguageRegistry>>,
    ) {
        let max_depth = self.layers.summary().max_depth;
        let mut cursor = self.layers.cursor::<SyntaxLayerSummary>();
        cursor.next(&text);
        let mut layers = SumTree::new();

        let mut changed_regions = ChangeRegionSet::default();
        let mut queue = BinaryHeap::new();
        let mut combined_injection_ranges = HashMap::default();
        queue.push(ParseStep {
            depth: 0,
            language: ParseStepLanguage::Loaded {
                language: root_language,
            },
            included_ranges: vec![tree_sitter::Range {
                start_byte: 0,
                end_byte: text.len(),
                start_point: Point::zero().to_ts_point(),
                end_point: text.max_point().to_ts_point(),
            }],
            range: Anchor::MIN..Anchor::MAX,
            mode: ParseMode::Single,
        });

        loop {
            let step = queue.pop();
            let position = if let Some(step) = &step {
                SyntaxLayerPosition {
                    depth: step.depth,
                    range: step.range.clone(),
                    language: step.language.id(),
                }
            } else {
                SyntaxLayerPosition {
                    depth: max_depth + 1,
                    range: Anchor::MAX..Anchor::MAX,
                    language: None,
                }
            };

            let mut done = cursor.item().is_none();
            while !done && position.cmp(&cursor.end(text), &text).is_gt() {
                done = true;

                let bounded_position = SyntaxLayerPositionBeforeChange {
                    position: position.clone(),
                    change: changed_regions.start_position(),
                };
                if bounded_position.cmp(&cursor.start(), &text).is_gt() {
                    let slice = cursor.slice(&bounded_position, Bias::Left, text);
                    if !slice.is_empty() {
                        layers.push_tree(slice, &text);
                        if changed_regions.prune(cursor.end(text), text) {
                            done = false;
                        }
                    }
                }

                while position.cmp(&cursor.end(text), text).is_gt() {
                    let Some(layer) = cursor.item() else { break };

                    if changed_regions.intersects(&layer, text) {
                        changed_regions.insert(
                            ChangedRegion {
                                depth: layer.depth + 1,
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

            let Some(step) = step else { break };
            let (step_start_byte, step_start_point) =
                step.range.start.summary::<(usize, Point)>(text);
            let step_end_byte = step.range.end.to_offset(text);

            let mut old_layer = cursor.item();
            if let Some(layer) = old_layer {
                if layer.range.to_offset(text) == (step_start_byte..step_end_byte)
                    && layer.content.language_id() == step.language.id()
                {
                    cursor.next(&text);
                } else {
                    old_layer = None;
                }
            }

            let content = match step.language {
                ParseStepLanguage::Loaded { language } => {
                    let Some(grammar) = language.grammar() else { continue };
                    let tree;
                    let changed_ranges;
                    let mut included_ranges = step.included_ranges;
                    if let Some(SyntaxLayerContent::Parsed { tree: old_tree, .. }) =
                        old_layer.map(|layer| &layer.content)
                    {
                        if let ParseMode::Combined {
                            parent_layer_changed_ranges,
                            ..
                        } = step.mode
                        {
                            included_ranges = splice_included_ranges(
                                old_tree.included_ranges(),
                                &parent_layer_changed_ranges,
                                &included_ranges,
                            );
                        }

                        tree = parse_text(
                            grammar,
                            text.as_rope(),
                            step_start_byte,
                            step_start_point,
                            included_ranges,
                            Some(old_tree.clone()),
                        );
                        changed_ranges = join_ranges(
                            invalidated_ranges.iter().cloned().filter(|range| {
                                range.start <= step_end_byte && range.end >= step_start_byte
                            }),
                            old_tree.changed_ranges(&tree).map(|r| {
                                step_start_byte + r.start_byte..step_start_byte + r.end_byte
                            }),
                        );
                    } else {
                        tree = parse_text(
                            grammar,
                            text.as_rope(),
                            step_start_byte,
                            step_start_point,
                            included_ranges,
                            None,
                        );
                        changed_ranges = vec![step_start_byte..step_end_byte];
                    }

                    if let (Some((config, registry)), false) = (
                        grammar.injection_config.as_ref().zip(registry.as_ref()),
                        changed_ranges.is_empty(),
                    ) {
                        for range in &changed_ranges {
                            changed_regions.insert(
                                ChangedRegion {
                                    depth: step.depth + 1,
                                    range: text.anchor_before(range.start)
                                        ..text.anchor_after(range.end),
                                },
                                text,
                            );
                        }
                        get_injections(
                            config,
                            text,
                            tree.root_node_with_offset(
                                step_start_byte,
                                step_start_point.to_ts_point(),
                            ),
                            registry,
                            step.depth + 1,
                            &changed_ranges,
                            &mut combined_injection_ranges,
                            &mut queue,
                        );
                    }

                    SyntaxLayerContent::Parsed { tree, language }
                }
                ParseStepLanguage::Pending { name } => SyntaxLayerContent::Pending {
                    language_name: name,
                },
            };

            layers.push(
                SyntaxLayer {
                    depth: step.depth,
                    range: step.range,
                    content,
                },
                &text,
            );
        }

        drop(cursor);
        self.layers = layers;
        self.interpolated_version = text.version.clone();
        self.parsed_version = text.version.clone();
    }

    pub fn single_tree_captures<'a>(
        range: Range<usize>,
        text: &'a Rope,
        tree: &'a Tree,
        language: &'a Arc<Language>,
        query: fn(&Grammar) -> Option<&Query>,
    ) -> SyntaxMapCaptures<'a> {
        SyntaxMapCaptures::new(
            range.clone(),
            text,
            [SyntaxLayerInfo {
                language,
                depth: 0,
                node: tree.root_node(),
            }]
            .into_iter(),
            query,
        )
    }

    pub fn captures<'a>(
        &'a self,
        range: Range<usize>,
        buffer: &'a BufferSnapshot,
        query: fn(&Grammar) -> Option<&Query>,
    ) -> SyntaxMapCaptures {
        SyntaxMapCaptures::new(
            range.clone(),
            buffer.as_rope(),
            self.layers_for_range(range, buffer).into_iter(),
            query,
        )
    }

    pub fn matches<'a>(
        &'a self,
        range: Range<usize>,
        buffer: &'a BufferSnapshot,
        query: fn(&Grammar) -> Option<&Query>,
    ) -> SyntaxMapMatches {
        SyntaxMapMatches::new(
            range.clone(),
            buffer.as_rope(),
            self.layers_for_range(range, buffer).into_iter(),
            query,
        )
    }

    #[cfg(test)]
    pub fn layers<'a>(&'a self, buffer: &'a BufferSnapshot) -> Vec<SyntaxLayerInfo> {
        self.layers_for_range(0..buffer.len(), buffer).collect()
    }

    pub fn layers_for_range<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
        buffer: &'a BufferSnapshot,
    ) -> impl 'a + Iterator<Item = SyntaxLayerInfo> {
        let start = buffer.anchor_before(range.start.to_offset(buffer));
        let end = buffer.anchor_after(range.end.to_offset(buffer));

        let mut cursor = self.layers.filter::<_, ()>(move |summary| {
            if summary.max_depth > summary.min_depth {
                true
            } else {
                let is_before_start = summary.range.end.cmp(&start, buffer).is_lt();
                let is_after_end = summary.range.start.cmp(&end, buffer).is_gt();
                !is_before_start && !is_after_end
            }
        });

        cursor.next(buffer);
        iter::from_fn(move || {
            while let Some(layer) = cursor.item() {
                if let SyntaxLayerContent::Parsed { tree, language } = &layer.content {
                    let info = SyntaxLayerInfo {
                        language,
                        depth: layer.depth,
                        node: tree.root_node_with_offset(
                            layer.range.start.to_offset(buffer),
                            layer.range.start.to_point(buffer).to_ts_point(),
                        ),
                    };
                    cursor.next(buffer);
                    return Some(info);
                } else {
                    cursor.next(buffer);
                }
            }
            None
        })
    }

    pub fn contains_unknown_injections(&self) -> bool {
        self.layers.summary().contains_unknown_injections
    }

    pub fn language_registry_version(&self) -> usize {
        self.language_registry_version
    }
}

impl<'a> SyntaxMapCaptures<'a> {
    fn new(
        range: Range<usize>,
        text: &'a Rope,
        layers: impl Iterator<Item = SyntaxLayerInfo<'a>>,
        query: fn(&Grammar) -> Option<&Query>,
    ) -> Self {
        let mut result = Self {
            layers: Vec::new(),
            grammars: Vec::new(),
            active_layer_count: 0,
        };
        for SyntaxLayerInfo {
            language,
            depth,
            node,
        } in layers
        {
            let grammar = match &language.grammar {
                Some(grammer) => grammer,
                None => continue,
            };
            let query = match query(&grammar) {
                Some(query) => query,
                None => continue,
            };

            let mut query_cursor = QueryCursorHandle::new();

            // TODO - add a Tree-sitter API to remove the need for this.
            let cursor = unsafe {
                std::mem::transmute::<_, &'static mut QueryCursor>(query_cursor.deref_mut())
            };

            cursor.set_byte_range(range.clone());
            let captures = cursor.captures(query, node, TextProvider(text));
            let grammar_index = result
                .grammars
                .iter()
                .position(|g| g.id == grammar.id())
                .unwrap_or_else(|| {
                    result.grammars.push(grammar);
                    result.grammars.len() - 1
                });
            let mut layer = SyntaxMapCapturesLayer {
                depth,
                grammar_index,
                next_capture: None,
                captures,
                _query_cursor: query_cursor,
            };

            layer.advance();
            if layer.next_capture.is_some() {
                let key = layer.sort_key();
                let ix = match result.layers[..result.active_layer_count]
                    .binary_search_by_key(&key, |layer| layer.sort_key())
                {
                    Ok(ix) | Err(ix) => ix,
                };
                result.layers.insert(ix, layer);
                result.active_layer_count += 1;
            } else {
                result.layers.push(layer);
            }
        }

        result
    }

    pub fn grammars(&self) -> &[&'a Grammar] {
        &self.grammars
    }

    pub fn peek(&self) -> Option<SyntaxMapCapture<'a>> {
        let layer = self.layers[..self.active_layer_count].first()?;
        let capture = layer.next_capture?;
        Some(SyntaxMapCapture {
            depth: layer.depth,
            grammar_index: layer.grammar_index,
            index: capture.index,
            node: capture.node,
        })
    }

    pub fn advance(&mut self) -> bool {
        let layer = if let Some(layer) = self.layers[..self.active_layer_count].first_mut() {
            layer
        } else {
            return false;
        };

        layer.advance();
        if layer.next_capture.is_some() {
            let key = layer.sort_key();
            let i = 1 + self.layers[1..self.active_layer_count]
                .iter()
                .position(|later_layer| key < later_layer.sort_key())
                .unwrap_or(self.active_layer_count - 1);
            self.layers[0..i].rotate_left(1);
        } else {
            self.layers[0..self.active_layer_count].rotate_left(1);
            self.active_layer_count -= 1;
        }

        true
    }

    pub fn set_byte_range(&mut self, range: Range<usize>) {
        for layer in &mut self.layers {
            layer.captures.set_byte_range(range.clone());
            if let Some(capture) = &layer.next_capture {
                if capture.node.end_byte() > range.start {
                    continue;
                }
            }
            layer.advance();
        }
        self.layers.sort_unstable_by_key(|layer| layer.sort_key());
        self.active_layer_count = self
            .layers
            .iter()
            .position(|layer| layer.next_capture.is_none())
            .unwrap_or(self.layers.len());
    }
}

impl<'a> SyntaxMapMatches<'a> {
    fn new(
        range: Range<usize>,
        text: &'a Rope,
        layers: impl Iterator<Item = SyntaxLayerInfo<'a>>,
        query: fn(&Grammar) -> Option<&Query>,
    ) -> Self {
        let mut result = Self::default();
        for SyntaxLayerInfo {
            language,
            depth,
            node,
        } in layers
        {
            let grammar = match &language.grammar {
                Some(grammer) => grammer,
                None => continue,
            };
            let query = match query(&grammar) {
                Some(query) => query,
                None => continue,
            };

            let mut query_cursor = QueryCursorHandle::new();

            // TODO - add a Tree-sitter API to remove the need for this.
            let cursor = unsafe {
                std::mem::transmute::<_, &'static mut QueryCursor>(query_cursor.deref_mut())
            };

            cursor.set_byte_range(range.clone());
            let matches = cursor.matches(query, node, TextProvider(text));
            let grammar_index = result
                .grammars
                .iter()
                .position(|g| g.id == grammar.id())
                .unwrap_or_else(|| {
                    result.grammars.push(grammar);
                    result.grammars.len() - 1
                });
            let mut layer = SyntaxMapMatchesLayer {
                depth,
                grammar_index,
                matches,
                next_pattern_index: 0,
                next_captures: Vec::new(),
                has_next: false,
                _query_cursor: query_cursor,
            };

            layer.advance();
            if layer.has_next {
                let key = layer.sort_key();
                let ix = match result.layers[..result.active_layer_count]
                    .binary_search_by_key(&key, |layer| layer.sort_key())
                {
                    Ok(ix) | Err(ix) => ix,
                };
                result.layers.insert(ix, layer);
                result.active_layer_count += 1;
            } else {
                result.layers.push(layer);
            }
        }
        result
    }

    pub fn grammars(&self) -> &[&'a Grammar] {
        &self.grammars
    }

    pub fn peek(&self) -> Option<SyntaxMapMatch> {
        let layer = self.layers.first()?;
        if !layer.has_next {
            return None;
        }
        Some(SyntaxMapMatch {
            depth: layer.depth,
            grammar_index: layer.grammar_index,
            pattern_index: layer.next_pattern_index,
            captures: &layer.next_captures,
        })
    }

    pub fn advance(&mut self) -> bool {
        let layer = if let Some(layer) = self.layers.first_mut() {
            layer
        } else {
            return false;
        };

        layer.advance();
        if layer.has_next {
            let key = layer.sort_key();
            let i = 1 + self.layers[1..self.active_layer_count]
                .iter()
                .position(|later_layer| key < later_layer.sort_key())
                .unwrap_or(self.active_layer_count - 1);
            self.layers[0..i].rotate_left(1);
        } else {
            self.layers[0..self.active_layer_count].rotate_left(1);
            self.active_layer_count -= 1;
        }

        true
    }
}

impl<'a> SyntaxMapCapturesLayer<'a> {
    fn advance(&mut self) {
        self.next_capture = self.captures.next().map(|(mat, ix)| mat.captures[ix]);
    }

    fn sort_key(&self) -> (usize, Reverse<usize>, usize) {
        if let Some(capture) = &self.next_capture {
            let range = capture.node.byte_range();
            (range.start, Reverse(range.end), self.depth)
        } else {
            (usize::MAX, Reverse(0), usize::MAX)
        }
    }
}

impl<'a> SyntaxMapMatchesLayer<'a> {
    fn advance(&mut self) {
        if let Some(mat) = self.matches.next() {
            self.next_captures.clear();
            self.next_captures.extend_from_slice(&mat.captures);
            self.next_pattern_index = mat.pattern_index;
            self.has_next = true;
        } else {
            self.has_next = false;
        }
    }

    fn sort_key(&self) -> (usize, Reverse<usize>, usize) {
        if self.has_next {
            let captures = &self.next_captures;
            if let Some((first, last)) = captures.first().zip(captures.last()) {
                return (
                    first.node.start_byte(),
                    Reverse(last.node.end_byte()),
                    self.depth,
                );
            }
        }
        (usize::MAX, Reverse(0), usize::MAX)
    }
}

impl<'a> Iterator for SyntaxMapCaptures<'a> {
    type Item = SyntaxMapCapture<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.peek();
        self.advance();
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
    start_byte: usize,
    start_point: Point,
    mut ranges: Vec<tree_sitter::Range>,
    old_tree: Option<Tree>,
) -> Tree {
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
    language_registry: &Arc<LanguageRegistry>,
    depth: usize,
    changed_ranges: &[Range<usize>],
    combined_injection_ranges: &mut HashMap<Arc<Language>, Vec<tree_sitter::Range>>,
    queue: &mut BinaryHeap<ParseStep>,
) {
    let mut query_cursor = QueryCursorHandle::new();
    let mut prev_match = None;

    combined_injection_ranges.clear();
    for pattern in &config.patterns {
        if let (Some(language_name), true) = (pattern.language.as_ref(), pattern.combined) {
            if let Some(language) = language_registry.language_for_name_or_extension(language_name)
            {
                combined_injection_ranges.insert(language, Vec::new());
            }
        }
    }

    for query_range in changed_ranges {
        query_cursor.set_byte_range(query_range.start.saturating_sub(1)..query_range.end + 1);
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

            let combined = config.patterns[mat.pattern_index].combined;

            let mut language_name = None;
            let mut step_range = content_range.clone();
            if let Some(name) = config.patterns[mat.pattern_index].language.as_ref() {
                language_name = Some(Cow::Borrowed(name.as_ref()))
            } else if let Some(language_node) = config
                .language_capture_ix
                .and_then(|ix| mat.nodes_for_capture_index(ix).next())
            {
                step_range.start = cmp::min(content_range.start, language_node.start_byte());
                step_range.end = cmp::max(content_range.end, language_node.end_byte());
                language_name = Some(Cow::Owned(
                    text.text_for_range(language_node.byte_range()).collect(),
                ))
            };

            if let Some(language_name) = language_name {
                let language = {
                    let language_name: &str = &language_name;
                    language_registry.language_for_name_or_extension(language_name)
                };
                let range = text.anchor_before(step_range.start)..text.anchor_after(step_range.end);
                if let Some(language) = language {
                    if combined {
                        combined_injection_ranges
                            .get_mut(&language.clone())
                            .unwrap()
                            .extend(content_ranges);
                    } else {
                        queue.push(ParseStep {
                            depth,
                            language: ParseStepLanguage::Loaded { language },
                            included_ranges: content_ranges,
                            range,
                            mode: ParseMode::Single,
                        });
                    }
                } else {
                    queue.push(ParseStep {
                        depth,
                        language: ParseStepLanguage::Pending {
                            name: language_name.into(),
                        },
                        included_ranges: content_ranges,
                        range,
                        mode: ParseMode::Single,
                    });
                }
            }
        }
    }

    for (language, mut included_ranges) in combined_injection_ranges.drain() {
        included_ranges.sort_unstable();
        let range = text.anchor_before(node.start_byte())..text.anchor_after(node.end_byte());
        queue.push(ParseStep {
            depth,
            language: ParseStepLanguage::Loaded { language },
            range,
            included_ranges,
            mode: ParseMode::Combined {
                parent_layer_range: node.start_byte()..node.end_byte(),
                parent_layer_changed_ranges: changed_ranges.to_vec(),
            },
        })
    }
}

fn splice_included_ranges(
    mut ranges: Vec<tree_sitter::Range>,
    changed_ranges: &[Range<usize>],
    new_ranges: &[tree_sitter::Range],
) -> Vec<tree_sitter::Range> {
    let mut changed_ranges = changed_ranges.into_iter().peekable();
    let mut new_ranges = new_ranges.into_iter().peekable();
    let mut ranges_ix = 0;
    loop {
        let new_range = new_ranges.peek();
        let mut changed_range = changed_ranges.peek();

        // Remove ranges that have changed before inserting any new ranges
        // into those ranges.
        if let Some((changed, new)) = changed_range.zip(new_range) {
            if new.end_byte < changed.start {
                changed_range = None;
            }
        }

        if let Some(changed) = changed_range {
            let mut start_ix = ranges_ix
                + match ranges[ranges_ix..].binary_search_by_key(&changed.start, |r| r.end_byte) {
                    Ok(ix) | Err(ix) => ix,
                };
            let mut end_ix = ranges_ix
                + match ranges[ranges_ix..].binary_search_by_key(&changed.end, |r| r.start_byte) {
                    Ok(ix) => ix + 1,
                    Err(ix) => ix,
                };

            // If there are empty ranges, then there may be multiple ranges with the same
            // start or end. Expand the splice to include any adjacent ranges that touch
            // the changed range.
            while start_ix > 0 {
                if ranges[start_ix - 1].end_byte == changed.start {
                    start_ix -= 1;
                } else {
                    break;
                }
            }
            while let Some(range) = ranges.get(end_ix) {
                if range.start_byte == changed.end {
                    end_ix += 1;
                } else {
                    break;
                }
            }

            if end_ix > start_ix {
                ranges.splice(start_ix..end_ix, []);
            }
            changed_ranges.next();
            ranges_ix = start_ix;
        } else if let Some(new_range) = new_range {
            let ix = ranges_ix
                + match ranges[ranges_ix..]
                    .binary_search_by_key(&new_range.start_byte, |r| r.start_byte)
                {
                    Ok(ix) | Err(ix) => ix,
                };
            ranges.insert(ix, **new_range);
            new_ranges.next();
            ranges_ix = ix + 1;
        } else {
            break;
        }
    }
    ranges
}

impl<'a> SyntaxLayerInfo<'a> {
    pub(crate) fn override_id(&self, offset: usize, text: &text::BufferSnapshot) -> Option<u32> {
        let text = TextProvider(text.as_rope());
        let config = self.language.grammar.as_ref()?.override_config.as_ref()?;

        let mut query_cursor = QueryCursorHandle::new();
        query_cursor.set_byte_range(offset..offset);

        let mut smallest_match: Option<(u32, Range<usize>)> = None;
        for mat in query_cursor.matches(&config.query, self.node, text) {
            for capture in mat.captures {
                if !config.values.contains_key(&capture.index) {
                    continue;
                }

                let range = capture.node.byte_range();
                if offset <= range.start || offset >= range.end {
                    continue;
                }

                if let Some((_, smallest_range)) = &smallest_match {
                    if range.len() < smallest_range.len() {
                        smallest_match = Some((capture.index, range))
                    }
                    continue;
                }

                smallest_match = Some((capture.index, range));
            }
        }

        smallest_match.map(|(index, _)| index)
    }
}

impl std::ops::Deref for SyntaxMap {
    type Target = SyntaxSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl PartialEq for ParseStep {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Eq for ParseStep {}

impl PartialOrd for ParseStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for ParseStep {
    fn cmp(&self, other: &Self) -> Ordering {
        let range_a = self.range();
        let range_b = other.range();
        Ord::cmp(&other.depth, &self.depth)
            .then_with(|| Ord::cmp(&range_b.start, &range_a.start))
            .then_with(|| Ord::cmp(&range_a.end, &range_b.end))
            .then_with(|| self.language.id().cmp(&other.language.id()))
    }
}

impl ParseStep {
    fn range(&self) -> Range<usize> {
        if let ParseMode::Combined {
            parent_layer_range, ..
        } = &self.mode
        {
            parent_layer_range.clone()
        } else {
            let start = self.included_ranges.first().map_or(0, |r| r.start_byte);
            let end = self.included_ranges.last().map_or(0, |r| r.end_byte);
            start..end
        }
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
    fn start_position(&self) -> ChangeStartPosition {
        self.0.first().map_or(
            ChangeStartPosition {
                depth: usize::MAX,
                position: Anchor::MAX,
            },
            |region| ChangeStartPosition {
                depth: region.depth,
                position: region.range.start,
            },
        )
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
            min_depth: 0,
            range: Anchor::MAX..Anchor::MIN,
            last_layer_range: Anchor::MIN..Anchor::MAX,
            last_layer_language: None,
            contains_unknown_injections: false,
        }
    }
}

impl sum_tree::Summary for SyntaxLayerSummary {
    type Context = BufferSnapshot;

    fn add_summary(&mut self, other: &Self, buffer: &Self::Context) {
        if other.max_depth > self.max_depth {
            self.max_depth = other.max_depth;
            self.range = other.range.clone();
        } else {
            if other.range.start.cmp(&self.range.start, buffer).is_lt() {
                self.range.start = other.range.start;
            }
            if other.range.end.cmp(&self.range.end, buffer).is_gt() {
                self.range.end = other.range.end;
            }
        }
        self.last_layer_range = other.last_layer_range.clone();
        self.last_layer_language = other.last_layer_language;
        self.contains_unknown_injections |= other.contains_unknown_injections;
    }
}

impl<'a> SeekTarget<'a, SyntaxLayerSummary, SyntaxLayerSummary> for SyntaxLayerPosition {
    fn cmp(&self, cursor_location: &SyntaxLayerSummary, buffer: &BufferSnapshot) -> Ordering {
        Ord::cmp(&self.depth, &cursor_location.max_depth)
            .then_with(|| {
                self.range
                    .start
                    .cmp(&cursor_location.last_layer_range.start, buffer)
            })
            .then_with(|| {
                cursor_location
                    .last_layer_range
                    .end
                    .cmp(&self.range.end, buffer)
            })
            .then_with(|| self.language.cmp(&cursor_location.last_layer_language))
    }
}

impl<'a> SeekTarget<'a, SyntaxLayerSummary, SyntaxLayerSummary> for ChangeStartPosition {
    fn cmp(&self, cursor_location: &SyntaxLayerSummary, text: &BufferSnapshot) -> Ordering {
        Ord::cmp(&self.depth, &cursor_location.max_depth)
            .then_with(|| self.position.cmp(&cursor_location.range.end, text))
    }
}

impl<'a> SeekTarget<'a, SyntaxLayerSummary, SyntaxLayerSummary>
    for SyntaxLayerPositionBeforeChange
{
    fn cmp(&self, cursor_location: &SyntaxLayerSummary, buffer: &BufferSnapshot) -> Ordering {
        if self.change.cmp(cursor_location, buffer).is_le() {
            return Ordering::Less;
        } else {
            self.position.cmp(cursor_location, buffer)
        }
    }
}

impl sum_tree::Item for SyntaxLayer {
    type Summary = SyntaxLayerSummary;

    fn summary(&self) -> Self::Summary {
        SyntaxLayerSummary {
            min_depth: self.depth,
            max_depth: self.depth,
            range: self.range.clone(),
            last_layer_range: self.range.clone(),
            last_layer_language: self.content.language_id(),
            contains_unknown_injections: matches!(self.content, SyntaxLayerContent::Pending { .. }),
        }
    }
}

impl std::fmt::Debug for SyntaxLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxLayer")
            .field("depth", &self.depth)
            .field("range", &self.range)
            .field("tree", &self.content.tree())
            .finish()
    }
}

impl<'a> tree_sitter::TextProvider<'a> for TextProvider<'a> {
    type I = ByteChunks<'a>;

    fn text(&mut self, node: tree_sitter::Node) -> Self::I {
        ByteChunks(self.0.chunks_in_range(node.byte_range()))
    }
}

impl<'a> Iterator for ByteChunks<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(str::as_bytes)
    }
}

impl QueryCursorHandle {
    pub(crate) fn new() -> Self {
        let mut cursor = QUERY_CURSORS.lock().pop().unwrap_or_else(QueryCursor::new);
        cursor.set_match_limit(64);
        QueryCursorHandle(Some(cursor))
    }
}

impl Deref for QueryCursorHandle {
    type Target = QueryCursor;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl DerefMut for QueryCursorHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}

impl Drop for QueryCursorHandle {
    fn drop(&mut self) {
        let mut cursor = self.0.take().unwrap();
        cursor.set_byte_range(0..usize::MAX);
        cursor.set_point_range(Point::zero().to_ts_point()..Point::MAX.to_ts_point());
        QUERY_CURSORS.lock().push(cursor)
    }
}

pub(crate) trait ToTreeSitterPoint {
    fn to_ts_point(self) -> tree_sitter::Point;
    fn from_ts_point(point: tree_sitter::Point) -> Self;
}

impl ToTreeSitterPoint for Point {
    fn to_ts_point(self) -> tree_sitter::Point {
        tree_sitter::Point::new(self.row as usize, self.column as usize)
    }

    fn from_ts_point(point: tree_sitter::Point) -> Self {
        Point::new(point.row as u32, point.column as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LanguageConfig;
    use rand::rngs::StdRng;
    use std::env;
    use text::Buffer;
    use unindent::Unindent as _;
    use util::test::marked_text_ranges;

    #[test]
    fn test_splice_included_ranges() {
        let ranges = vec![ts_range(20..30), ts_range(50..60), ts_range(80..90)];

        let new_ranges = splice_included_ranges(
            ranges.clone(),
            &[54..56, 58..68],
            &[ts_range(50..54), ts_range(59..67)],
        );
        assert_eq!(
            new_ranges,
            &[
                ts_range(20..30),
                ts_range(50..54),
                ts_range(59..67),
                ts_range(80..90),
            ]
        );

        let new_ranges = splice_included_ranges(ranges.clone(), &[70..71, 91..100], &[]);
        assert_eq!(
            new_ranges,
            &[ts_range(20..30), ts_range(50..60), ts_range(80..90)]
        );

        let new_ranges =
            splice_included_ranges(ranges.clone(), &[], &[ts_range(0..2), ts_range(70..75)]);
        assert_eq!(
            new_ranges,
            &[
                ts_range(0..2),
                ts_range(20..30),
                ts_range(50..60),
                ts_range(70..75),
                ts_range(80..90)
            ]
        );

        let new_ranges = splice_included_ranges(ranges.clone(), &[30..50], &[ts_range(25..55)]);
        assert_eq!(new_ranges, &[ts_range(25..55), ts_range(80..90)]);

        fn ts_range(range: Range<usize>) -> tree_sitter::Range {
            tree_sitter::Range {
                start_byte: range.start,
                start_point: tree_sitter::Point {
                    row: 0,
                    column: range.start,
                },
                end_byte: range.end,
                end_point: tree_sitter::Point {
                    row: 0,
                    column: range.end,
                },
            }
        }
    }

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
    fn test_dynamic_language_injection() {
        let registry = Arc::new(LanguageRegistry::test());
        let markdown = Arc::new(markdown_lang());
        registry.add(markdown.clone());
        registry.add(Arc::new(rust_lang()));
        registry.add(Arc::new(ruby_lang()));

        let mut buffer = Buffer::new(
            0,
            0,
            r#"
                This is a code block:

                ```rs
                fn foo() {}
                ```
            "#
            .unindent(),
        );

        let mut syntax_map = SyntaxMap::new();
        syntax_map.set_language_registry(registry.clone());
        syntax_map.reparse(markdown.clone(), &buffer);
        assert_layers_for_range(
            &syntax_map,
            &buffer,
            Point::new(3, 0)..Point::new(3, 0),
            &[
                "...(fenced_code_block (fenced_code_block_delimiter) (info_string (language)) (code_fence_content) (fenced_code_block_delimiter...",
                "...(function_item name: (identifier) parameters: (parameters) body: (block)...",
            ],
        );

        // Replace Rust with Ruby in code block.
        let macro_name_range = range_for_text(&buffer, "rs");
        buffer.edit([(macro_name_range, "ruby")]);
        syntax_map.interpolate(&buffer);
        syntax_map.reparse(markdown.clone(), &buffer);
        assert_layers_for_range(
            &syntax_map,
            &buffer,
            Point::new(3, 0)..Point::new(3, 0),
            &[
                "...(fenced_code_block (fenced_code_block_delimiter) (info_string (language)) (code_fence_content) (fenced_code_block_delimiter...",
                "...(call method: (identifier) arguments: (argument_list (call method: (identifier) arguments: (argument_list) block: (block)...",
            ],
        );

        // Replace Ruby with a language that hasn't been loaded yet.
        let macro_name_range = range_for_text(&buffer, "ruby");
        buffer.edit([(macro_name_range, "html")]);
        syntax_map.interpolate(&buffer);
        syntax_map.reparse(markdown.clone(), &buffer);
        assert_layers_for_range(
            &syntax_map,
            &buffer,
            Point::new(3, 0)..Point::new(3, 0),
            &[
                "...(fenced_code_block (fenced_code_block_delimiter) (info_string (language)) (code_fence_content) (fenced_code_block_delimiter..."
            ],
        );
        assert!(syntax_map.contains_unknown_injections());

        registry.add(Arc::new(html_lang()));
        syntax_map.reparse(markdown.clone(), &buffer);
        assert_layers_for_range(
            &syntax_map,
            &buffer,
            Point::new(3, 0)..Point::new(3, 0),
            &[
                "...(fenced_code_block (fenced_code_block_delimiter) (info_string (language)) (code_fence_content) (fenced_code_block_delimiter...",
                "(fragment (text))",
            ],
        );
        assert!(!syntax_map.contains_unknown_injections());
    }

    #[gpui::test]
    fn test_typing_multiple_new_injections() {
        let (buffer, syntax_map) = test_edit_sequence(
            "Rust",
            &[
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
            ],
        );

        assert_capture_ranges(
            &syntax_map,
            &buffer,
            &["field"],
            "fn a() { dbg!(b.«c»(vec![d.«e»])) }",
        );
    }

    #[gpui::test]
    fn test_pasting_new_injection_line_between_others() {
        let (buffer, syntax_map) = test_edit_sequence(
            "Rust",
            &[
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
            ],
        );

        assert_capture_ranges(
            &syntax_map,
            &buffer,
            &["struct"],
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
        let (buffer, syntax_map) = test_edit_sequence(
            "Rust",
            &[
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
            ],
        );

        assert_capture_ranges(
            &syntax_map,
            &buffer,
            &["field"],
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
        test_edit_sequence(
            "Rust",
            &[
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
            ],
        );
    }

    #[gpui::test]
    fn test_edits_preceding_and_intersecting_injection() {
        test_edit_sequence(
            "Rust",
            &[
                //
                "const aaaaaaaaaaaa: B = c!(d(e.f));",
                "const aˇa: B = c!(d(eˇ));",
            ],
        );
    }

    #[gpui::test]
    fn test_non_local_changes_create_injections() {
        test_edit_sequence(
            "Rust",
            &[
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
            ],
        );
    }

    #[gpui::test]
    fn test_creating_many_injections_in_one_edit() {
        test_edit_sequence(
            "Rust",
            &[
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
            ],
        );
    }

    #[gpui::test]
    fn test_editing_across_injection_boundary() {
        test_edit_sequence(
            "Rust",
            &[
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
            ],
        );
    }

    #[gpui::test]
    fn test_removing_injection_by_replacing_across_boundary() {
        test_edit_sequence(
            "Rust",
            &[
                "
                    fn one() {
                        two!(
                            three.four,
                        );
                    }
                ",
                "
                    fn one() {
                        t«en
                            .eleven(
                            twelve,
                        »
                            three.four,
                        );
                    }
                ",
            ],
        );
    }

    #[gpui::test]
    fn test_combined_injections() {
        let (buffer, syntax_map) = test_edit_sequence(
            "ERB",
            &[
                "
                    <body>
                        <% if @one %>
                            <div class=one>
                        <% else %>
                            <div class=two>
                        <% end %>
                        </div>
                    </body>
                ",
                "
                    <body>
                        <% if @one %>
                            <div class=one>
                        ˇ else ˇ
                            <div class=two>
                        <% end %>
                        </div>
                    </body>
                ",
                "
                    <body>
                        <% if @one «;» end %>
                        </div>
                    </body>
                ",
            ],
        );

        assert_capture_ranges(
            &syntax_map,
            &buffer,
            &["tag", "ivar"],
            "
                <«body»>
                    <% if «@one» ; end %>
                    </«div»>
                </«body»>
            ",
        );
    }

    #[gpui::test]
    fn test_combined_injections_empty_ranges() {
        test_edit_sequence(
            "ERB",
            &[
                "
                    <% if @one %>
                    <% else %>
                    <% end %>
                ",
                "
                    <% if @one %>
                    ˇ<% end %>
                ",
            ],
        );
    }

    #[gpui::test]
    fn test_combined_injections_edit_edges_of_ranges() {
        let (buffer, syntax_map) = test_edit_sequence(
            "ERB",
            &[
                "
                    <%= one @two %>
                    <%= three @four %>
                ",
                "
                    <%= one @two %ˇ
                    <%= three @four %>
                ",
                "
                    <%= one @two %«>»
                    <%= three @four %>
                ",
            ],
        );

        assert_capture_ranges(
            &syntax_map,
            &buffer,
            &["tag", "ivar"],
            "
                <%= one «@two» %>
                <%= three «@four» %>
            ",
        );
    }

    #[gpui::test]
    fn test_combined_injections_splitting_some_injections() {
        let (_buffer, _syntax_map) = test_edit_sequence(
            "ERB",
            &[
                r#"
                      <%A if b(:c) %>
                        d
                      <% end %>
                      eee
                      <% f %>
                "#,
                r#"
                      <%« AAAAAAA %>
                        hhhhhhh
                      <%=» if b(:c) %>
                        d
                      <% end %>
                      eee
                      <% f %>
                "#,
            ],
        );
    }

    #[gpui::test(iterations = 50)]
    fn test_random_syntax_map_edits(mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let text = r#"
            fn test_something() {
                let vec = vec![5, 1, 3, 8];
                assert_eq!(
                    vec
                        .into_iter()
                        .map(|i| i * 2)
                        .collect::<Vec<usize>>(),
                    vec![
                        5 * 2, 1 * 2, 3 * 2, 8 * 2
                    ],
                );
            }
        "#
        .unindent()
        .repeat(2);

        let registry = Arc::new(LanguageRegistry::test());
        let language = Arc::new(rust_lang());
        registry.add(language.clone());
        let mut buffer = Buffer::new(0, 0, text);

        let mut syntax_map = SyntaxMap::new();
        syntax_map.set_language_registry(registry.clone());
        syntax_map.reparse(language.clone(), &buffer);

        let mut reference_syntax_map = SyntaxMap::new();
        reference_syntax_map.set_language_registry(registry.clone());

        log::info!("initial text:\n{}", buffer.text());

        for _ in 0..operations {
            let prev_buffer = buffer.snapshot();
            let prev_syntax_map = syntax_map.snapshot();

            buffer.randomly_edit(&mut rng, 3);
            log::info!("text:\n{}", buffer.text());

            syntax_map.interpolate(&buffer);
            check_interpolation(&prev_syntax_map, &syntax_map, &prev_buffer, &buffer);

            syntax_map.reparse(language.clone(), &buffer);

            reference_syntax_map.clear();
            reference_syntax_map.reparse(language.clone(), &buffer);
        }

        for i in 0..operations {
            let i = operations - i - 1;
            buffer.undo();
            log::info!("undoing operation {}", i);
            log::info!("text:\n{}", buffer.text());

            syntax_map.interpolate(&buffer);
            syntax_map.reparse(language.clone(), &buffer);

            reference_syntax_map.clear();
            reference_syntax_map.reparse(language.clone(), &buffer);
            assert_eq!(
                syntax_map.layers(&buffer).len(),
                reference_syntax_map.layers(&buffer).len(),
                "wrong number of layers after undoing edit {i}"
            );
        }

        let layers = syntax_map.layers(&buffer);
        let reference_layers = reference_syntax_map.layers(&buffer);
        for (edited_layer, reference_layer) in layers.into_iter().zip(reference_layers.into_iter())
        {
            assert_eq!(edited_layer.node.to_sexp(), reference_layer.node.to_sexp());
            assert_eq!(edited_layer.node.range(), reference_layer.node.range());
        }
    }

    #[gpui::test(iterations = 50)]
    fn test_random_syntax_map_edits_with_combined_injections(mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let text = r#"
          <div id="main">
            <% if one?(:two) %>
              <p class="three" four>
                <%= yield :five %>
              </p>
            <% elsif Six.seven(8) %>
              <p id="three" four>
                <%= yield :five %>
              </p>
            <% else %>
              <span>Ok</span>
            <% end %>
          </div>
        "#
        .unindent()
        .repeat(8);

        let registry = Arc::new(LanguageRegistry::test());
        let language = Arc::new(erb_lang());
        registry.add(language.clone());
        registry.add(Arc::new(ruby_lang()));
        registry.add(Arc::new(html_lang()));
        let mut buffer = Buffer::new(0, 0, text);

        let mut syntax_map = SyntaxMap::new();
        syntax_map.set_language_registry(registry.clone());
        syntax_map.reparse(language.clone(), &buffer);

        let mut reference_syntax_map = SyntaxMap::new();
        reference_syntax_map.set_language_registry(registry.clone());

        log::info!("initial text:\n{}", buffer.text());

        for _ in 0..operations {
            let prev_buffer = buffer.snapshot();
            let prev_syntax_map = syntax_map.snapshot();

            buffer.randomly_edit(&mut rng, 3);
            log::info!("text:\n{}", buffer.text());

            syntax_map.interpolate(&buffer);
            check_interpolation(&prev_syntax_map, &syntax_map, &prev_buffer, &buffer);

            syntax_map.reparse(language.clone(), &buffer);

            reference_syntax_map.clear();
            reference_syntax_map.reparse(language.clone(), &buffer);
        }

        for i in 0..operations {
            let i = operations - i - 1;
            buffer.undo();
            log::info!("undoing operation {}", i);
            log::info!("text:\n{}", buffer.text());

            syntax_map.interpolate(&buffer);
            syntax_map.reparse(language.clone(), &buffer);

            reference_syntax_map.clear();
            reference_syntax_map.reparse(language.clone(), &buffer);
            assert_eq!(
                syntax_map.layers(&buffer).len(),
                reference_syntax_map.layers(&buffer).len(),
                "wrong number of layers after undoing edit {i}"
            );
        }

        let layers = syntax_map.layers(&buffer);
        let reference_layers = reference_syntax_map.layers(&buffer);
        for (edited_layer, reference_layer) in layers.into_iter().zip(reference_layers.into_iter())
        {
            assert_eq!(edited_layer.node.to_sexp(), reference_layer.node.to_sexp());
            assert_eq!(edited_layer.node.range(), reference_layer.node.range());
        }
    }

    fn check_interpolation(
        old_syntax_map: &SyntaxSnapshot,
        new_syntax_map: &SyntaxSnapshot,
        old_buffer: &BufferSnapshot,
        new_buffer: &BufferSnapshot,
    ) {
        let edits = new_buffer
            .edits_since::<usize>(&old_buffer.version())
            .collect::<Vec<_>>();

        for (old_layer, new_layer) in old_syntax_map
            .layers
            .iter()
            .zip(new_syntax_map.layers.iter())
        {
            assert_eq!(old_layer.range, new_layer.range);
            let Some(old_tree) = old_layer.content.tree() else { continue };
            let Some(new_tree) = new_layer.content.tree() else { continue };
            let old_start_byte = old_layer.range.start.to_offset(old_buffer);
            let new_start_byte = new_layer.range.start.to_offset(new_buffer);
            let old_start_point = old_layer.range.start.to_point(old_buffer).to_ts_point();
            let new_start_point = new_layer.range.start.to_point(new_buffer).to_ts_point();
            let old_node = old_tree.root_node_with_offset(old_start_byte, old_start_point);
            let new_node = new_tree.root_node_with_offset(new_start_byte, new_start_point);
            check_node_edits(
                old_layer.depth,
                &old_layer.range,
                old_node,
                new_node,
                old_buffer,
                new_buffer,
                &edits,
            );
        }

        fn check_node_edits(
            depth: usize,
            range: &Range<Anchor>,
            old_node: Node,
            new_node: Node,
            old_buffer: &BufferSnapshot,
            new_buffer: &BufferSnapshot,
            edits: &[text::Edit<usize>],
        ) {
            assert_eq!(old_node.kind(), new_node.kind());

            let old_range = old_node.byte_range();
            let new_range = new_node.byte_range();

            let is_edited = edits
                .iter()
                .any(|edit| edit.new.start < new_range.end && edit.new.end > new_range.start);
            if is_edited {
                assert!(
                    new_node.has_changes(),
                    concat!(
                        "failed to mark node as edited.\n",
                        "layer depth: {}, old layer range: {:?}, new layer range: {:?},\n",
                        "node kind: {}, old node range: {:?}, new node range: {:?}",
                    ),
                    depth,
                    range.to_offset(old_buffer),
                    range.to_offset(new_buffer),
                    new_node.kind(),
                    old_range,
                    new_range,
                );
            }

            if !new_node.has_changes() {
                assert_eq!(
                    old_buffer
                        .text_for_range(old_range.clone())
                        .collect::<String>(),
                    new_buffer
                        .text_for_range(new_range.clone())
                        .collect::<String>(),
                    concat!(
                        "mismatched text for node\n",
                        "layer depth: {}, old layer range: {:?}, new layer range: {:?},\n",
                        "node kind: {}, old node range:{:?}, new node range:{:?}",
                    ),
                    depth,
                    range.to_offset(old_buffer),
                    range.to_offset(new_buffer),
                    new_node.kind(),
                    old_range,
                    new_range,
                );
            }

            for i in 0..new_node.child_count() {
                check_node_edits(
                    depth,
                    range,
                    old_node.child(i).unwrap(),
                    new_node.child(i).unwrap(),
                    old_buffer,
                    new_buffer,
                    edits,
                )
            }
        }
    }

    fn test_edit_sequence(language_name: &str, steps: &[&str]) -> (Buffer, SyntaxMap) {
        let registry = Arc::new(LanguageRegistry::test());
        registry.add(Arc::new(rust_lang()));
        registry.add(Arc::new(ruby_lang()));
        registry.add(Arc::new(html_lang()));
        registry.add(Arc::new(erb_lang()));
        registry.add(Arc::new(markdown_lang()));
        let language = registry.language_for_name(language_name).unwrap();
        let mut buffer = Buffer::new(0, 0, Default::default());

        let mut mutated_syntax_map = SyntaxMap::new();
        mutated_syntax_map.set_language_registry(registry.clone());
        mutated_syntax_map.reparse(language.clone(), &buffer);

        for (i, marked_string) in steps.into_iter().enumerate() {
            buffer.edit_via_marked_text(&marked_string.unindent());

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
                    edited_layer.node.to_sexp(),
                    reference_layer.node.to_sexp(),
                    "different layer at step {i}"
                );
                assert_eq!(
                    edited_layer.node.range(),
                    reference_layer.node.range(),
                    "different layer at step {i}"
                );
            }
        }

        (buffer, mutated_syntax_map)
    }

    fn html_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "HTML".into(),
                path_suffixes: vec!["html".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_html::language()),
        )
        .with_highlights_query(
            r#"
                (tag_name) @tag
                (erroneous_end_tag_name) @tag
                (attribute_name) @property
            "#,
        )
        .unwrap()
    }

    fn ruby_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Ruby".into(),
                path_suffixes: vec!["rb".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_ruby::language()),
        )
        .with_highlights_query(
            r#"
                ["if" "do" "else" "end"] @keyword
                (instance_variable) @ivar
            "#,
        )
        .unwrap()
    }

    fn erb_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "ERB".into(),
                path_suffixes: vec!["erb".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_embedded_template::language()),
        )
        .with_highlights_query(
            r#"
                ["<%" "%>"] @keyword
            "#,
        )
        .unwrap()
        .with_injection_query(
            r#"
                ((code) @content
                 (#set! "language" "ruby")
                 (#set! "combined"))

                 ((content) @content
                 (#set! "language" "html")
                 (#set! "combined"))
            "#,
        )
        .unwrap()
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
        .with_highlights_query(
            r#"
                (field_identifier) @field
                (struct_expression) @struct
            "#,
        )
        .unwrap()
        .with_injection_query(
            r#"
                (macro_invocation
                    (token_tree) @content
                    (#set! "language" "rust"))
            "#,
        )
        .unwrap()
    }

    fn markdown_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Markdown".into(),
                path_suffixes: vec!["md".into()],
                ..Default::default()
            },
            Some(tree_sitter_markdown::language()),
        )
        .with_injection_query(
            r#"
                (fenced_code_block
                    (info_string
                        (language) @language)
                    (code_fence_content) @content)
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
        let layers = syntax_map
            .layers_for_range(range, &buffer)
            .collect::<Vec<_>>();
        assert_eq!(
            layers.len(),
            expected_layers.len(),
            "wrong number of layers"
        );
        for (i, (SyntaxLayerInfo { node, .. }, expected_s_exp)) in
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

    fn assert_capture_ranges(
        syntax_map: &SyntaxMap,
        buffer: &BufferSnapshot,
        highlight_query_capture_names: &[&str],
        marked_string: &str,
    ) {
        let mut actual_ranges = Vec::<Range<usize>>::new();
        let captures = syntax_map.captures(0..buffer.len(), buffer, |grammar| {
            grammar.highlights_query.as_ref()
        });
        let queries = captures
            .grammars()
            .iter()
            .map(|grammar| grammar.highlights_query.as_ref().unwrap())
            .collect::<Vec<_>>();
        for capture in captures {
            let name = &queries[capture.grammar_index].capture_names()[capture.index as usize];
            if highlight_query_capture_names.contains(&name.as_str()) {
                actual_ranges.push(capture.node.byte_range());
            }
        }

        let (text, expected_ranges) = marked_text_ranges(&marked_string.unindent(), false);
        assert_eq!(text, buffer.text());
        assert_eq!(actual_ranges, expected_ranges);
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
