#[cfg(test)]
mod syntax_map_tests;

use crate::{Grammar, InjectionConfig, Language, LanguageId, LanguageRegistry};
use collections::HashMap;
use futures::FutureExt;
use parking_lot::Mutex;
use std::{
    borrow::Cow,
    cmp::{self, Ordering, Reverse},
    collections::BinaryHeap,
    fmt, iter,
    ops::{Deref, DerefMut, Range},
    sync::Arc,
};
use sum_tree::{Bias, SeekTarget, SumTree};
use text::{Anchor, BufferSnapshot, OffsetRangeExt, Point, Rope, ToOffset, ToPoint};
use tree_sitter::{Node, Query, QueryCapture, QueryCaptures, QueryCursor, QueryMatches, Tree};

use super::PARSER;

static QUERY_CURSORS: Mutex<Vec<QueryCursor>> = Mutex::new(vec![]);

#[derive(Default)]
pub struct SyntaxMap {
    snapshot: SyntaxSnapshot,
    language_registry: Option<Arc<LanguageRegistry>>,
}

#[derive(Clone, Default)]
pub struct SyntaxSnapshot {
    layers: SumTree<SyntaxLayerEntry>,
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
    captures: QueryCaptures<'a, 'a, TextProvider<'a>, &'a [u8]>,
    next_capture: Option<QueryCapture<'a>>,
    grammar_index: usize,
    _query_cursor: QueryCursorHandle,
}

struct SyntaxMapMatchesLayer<'a> {
    depth: usize,
    next_pattern_index: usize,
    next_captures: Vec<QueryCapture<'a>>,
    has_next: bool,
    matches: QueryMatches<'a, 'a, TextProvider<'a>, &'a [u8]>,
    grammar_index: usize,
    _query_cursor: QueryCursorHandle,
}

#[derive(Clone)]
struct SyntaxLayerEntry {
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
    fn language_id(&self) -> Option<LanguageId> {
        match self {
            SyntaxLayerContent::Parsed { language, .. } => Some(language.id),
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

/// A layer of syntax highlighting, corresponding to a single syntax
/// tree in a particular language.
#[derive(Debug)]
pub struct SyntaxLayer<'a> {
    /// The language for this layer.
    pub language: &'a Arc<Language>,
    depth: usize,
    tree: &'a Tree,
    offset: (usize, tree_sitter::Point),
}

/// A layer of syntax highlighting. Like [SyntaxLayer], but holding
/// owned data instead of references.
#[derive(Clone)]
pub struct OwnedSyntaxLayer {
    /// The language for this layer.
    pub language: Arc<Language>,
    tree: tree_sitter::Tree,
    offset: (usize, tree_sitter::Point),
}

#[derive(Debug, Clone)]
struct SyntaxLayerSummary {
    min_depth: usize,
    max_depth: usize,
    range: Range<Anchor>,
    last_layer_range: Range<Anchor>,
    last_layer_language: Option<LanguageId>,
    contains_unknown_injections: bool,
}

#[derive(Clone, Debug)]
struct SyntaxLayerPosition {
    depth: usize,
    range: Range<Anchor>,
    language: Option<LanguageId>,
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

#[derive(Debug)]
enum ParseStepLanguage {
    Loaded { language: Arc<Language> },
    Pending { name: Arc<str> },
}

impl ParseStepLanguage {
    fn id(&self) -> Option<LanguageId> {
        match self {
            ParseStepLanguage::Loaded { language } => Some(language.id),
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
                if target.cmp(cursor.start(), text).is_gt() {
                    let slice = cursor.slice(&target, Bias::Left, text);
                    layers.append(slice, text);
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
                layers.append(slice, text);
                continue;
            };

            let Some(layer) = cursor.item() else { break };
            let (start_byte, start_point) = layer.range.start.summary::<(usize, Point)>(text);

            // Ignore edits that end before the start of this layer, and don't consider them
            // for any subsequent layers at this same depth.
            loop {
                let Some((_, edit_range)) = edits.get(first_edit_ix_for_depth) else {
                    continue 'outer;
                };
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

        layers.append(cursor.suffix(text), text);
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
                    let SyntaxLayerContent::Pending { language_name } = &layer.content else {
                        unreachable!()
                    };
                    if registry
                        .language_for_name_or_extension(language_name)
                        .now_or_never()
                        .and_then(|language| language.ok())
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
        log::trace!("reparse. invalidated ranges:{:?}", invalidated_ranges);

        let max_depth = self.layers.summary().max_depth;
        let mut cursor = self.layers.cursor::<SyntaxLayerSummary>();
        cursor.next(text);
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
            while !done && position.cmp(&cursor.end(text), text).is_gt() {
                done = true;

                let bounded_position = SyntaxLayerPositionBeforeChange {
                    position: position.clone(),
                    change: changed_regions.start_position(),
                };
                if bounded_position.cmp(cursor.start(), text).is_gt() {
                    let slice = cursor.slice(&bounded_position, Bias::Left, text);
                    if !slice.is_empty() {
                        layers.append(slice, text);
                        if changed_regions.prune(cursor.end(text), text) {
                            done = false;
                        }
                    }
                }

                while position.cmp(&cursor.end(text), text).is_gt() {
                    let Some(layer) = cursor.item() else { break };

                    if changed_regions.intersects(layer, text) {
                        if let SyntaxLayerContent::Parsed { language, .. } = &layer.content {
                            log::trace!(
                                "discard layer. language:{}, range:{:?}. changed_regions:{:?}",
                                language.name(),
                                LogAnchorRange(&layer.range, text),
                                LogChangedRegions(&changed_regions, text),
                            );
                        }

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
                    cursor.next(text);
                } else {
                    old_layer = None;
                }
            }

            let content = match step.language {
                ParseStepLanguage::Loaded { language } => {
                    let Some(grammar) = language.grammar() else {
                        continue;
                    };
                    let tree;
                    let changed_ranges;

                    let mut included_ranges = step.included_ranges;
                    for range in &mut included_ranges {
                        range.start_byte -= step_start_byte;
                        range.end_byte -= step_start_byte;
                        range.start_point = (Point::from_ts_point(range.start_point)
                            - step_start_point)
                            .to_ts_point();
                        range.end_point = (Point::from_ts_point(range.end_point)
                            - step_start_point)
                            .to_ts_point();
                    }

                    if let Some((SyntaxLayerContent::Parsed { tree: old_tree, .. }, layer_start)) =
                        old_layer.map(|layer| (&layer.content, layer.range.start))
                    {
                        log::trace!(
                            "existing layer. language:{}, start:{:?}, ranges:{:?}",
                            language.name(),
                            LogPoint(layer_start.to_point(text)),
                            LogIncludedRanges(&old_tree.included_ranges())
                        );

                        if let ParseMode::Combined {
                            mut parent_layer_changed_ranges,
                            ..
                        } = step.mode
                        {
                            for range in &mut parent_layer_changed_ranges {
                                range.start = range.start.saturating_sub(step_start_byte);
                                range.end = range.end.saturating_sub(step_start_byte);
                            }

                            let changed_indices;
                            (included_ranges, changed_indices) = splice_included_ranges(
                                old_tree.included_ranges(),
                                &parent_layer_changed_ranges,
                                &included_ranges,
                            );
                            insert_newlines_between_ranges(
                                changed_indices,
                                &mut included_ranges,
                                text,
                                step_start_byte,
                                step_start_point,
                            );
                        }

                        if included_ranges.is_empty() {
                            included_ranges.push(tree_sitter::Range {
                                start_byte: 0,
                                end_byte: 0,
                                start_point: Default::default(),
                                end_point: Default::default(),
                            });
                        }

                        log::trace!(
                            "update layer. language:{}, start:{:?}, included_ranges:{:?}",
                            language.name(),
                            LogAnchorRange(&step.range, text),
                            LogIncludedRanges(&included_ranges),
                        );

                        tree = parse_text(
                            grammar,
                            text.as_rope(),
                            step_start_byte,
                            included_ranges,
                            Some(old_tree.clone()),
                        );
                        changed_ranges = join_ranges(
                            invalidated_ranges
                                .iter()
                                .filter(|&range| {
                                    range.start <= step_end_byte && range.end >= step_start_byte
                                })
                                .cloned(),
                            old_tree.changed_ranges(&tree).map(|r| {
                                step_start_byte + r.start_byte..step_start_byte + r.end_byte
                            }),
                        );
                    } else {
                        if matches!(step.mode, ParseMode::Combined { .. }) {
                            insert_newlines_between_ranges(
                                0..included_ranges.len(),
                                &mut included_ranges,
                                text,
                                step_start_byte,
                                step_start_point,
                            );
                        }

                        if included_ranges.is_empty() {
                            included_ranges.push(tree_sitter::Range {
                                start_byte: 0,
                                end_byte: 0,
                                start_point: Default::default(),
                                end_point: Default::default(),
                            });
                        }

                        log::trace!(
                            "create layer. language:{}, range:{:?}, included_ranges:{:?}",
                            language.name(),
                            LogAnchorRange(&step.range, text),
                            LogIncludedRanges(&included_ranges),
                        );

                        tree = parse_text(
                            grammar,
                            text.as_rope(),
                            step_start_byte,
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
                            step.range.clone(),
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
                SyntaxLayerEntry {
                    depth: step.depth,
                    range: step.range,
                    content,
                },
                text,
            );
        }

        drop(cursor);
        self.layers = layers;
        self.interpolated_version = text.version.clone();
        self.parsed_version = text.version.clone();
        #[cfg(debug_assertions)]
        self.check_invariants(text);
    }

    #[cfg(debug_assertions)]
    fn check_invariants(&self, text: &BufferSnapshot) {
        let mut max_depth = 0;
        let mut prev_range: Option<Range<Anchor>> = None;
        for layer in self.layers.iter() {
            if layer.depth == max_depth {
                if let Some(prev_range) = prev_range {
                    match layer.range.start.cmp(&prev_range.start, text) {
                        Ordering::Less => panic!("layers out of order"),
                        Ordering::Equal => {
                            assert!(layer.range.end.cmp(&prev_range.end, text).is_ge())
                        }
                        Ordering::Greater => {}
                    }
                }
            } else if layer.depth < max_depth {
                panic!("layers out of order")
            }
            max_depth = layer.depth;
            prev_range = Some(layer.range.clone());
        }
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
            [SyntaxLayer {
                language,
                tree,
                depth: 0,
                offset: (0, tree_sitter::Point::new(0, 0)),
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
            self.layers_for_range(range, buffer),
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
            self.layers_for_range(range, buffer),
            query,
        )
    }

    #[cfg(test)]
    pub fn layers<'a>(&'a self, buffer: &'a BufferSnapshot) -> Vec<SyntaxLayer> {
        self.layers_for_range(0..buffer.len(), buffer).collect()
    }

    pub fn layers_for_range<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
        buffer: &'a BufferSnapshot,
    ) -> impl 'a + Iterator<Item = SyntaxLayer> {
        let start_offset = range.start.to_offset(buffer);
        let end_offset = range.end.to_offset(buffer);
        let start = buffer.anchor_before(start_offset);
        let end = buffer.anchor_after(end_offset);

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
                let mut info = None;
                if let SyntaxLayerContent::Parsed { tree, language } = &layer.content {
                    let layer_start_offset = layer.range.start.to_offset(buffer);
                    let layer_start_point = layer.range.start.to_point(buffer).to_ts_point();

                    info = Some(SyntaxLayer {
                        tree,
                        language,
                        depth: layer.depth,
                        offset: (layer_start_offset, layer_start_point),
                    });
                }
                cursor.next(buffer);
                if info.is_some() {
                    return info;
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
        layers: impl Iterator<Item = SyntaxLayer<'a>>,
        query: fn(&Grammar) -> Option<&Query>,
    ) -> Self {
        let mut result = Self {
            layers: Vec::new(),
            grammars: Vec::new(),
            active_layer_count: 0,
        };
        for layer in layers {
            let grammar = match &layer.language.grammar {
                Some(grammar) => grammar,
                None => continue,
            };
            let query = match query(grammar) {
                Some(query) => query,
                None => continue,
            };

            let mut query_cursor = QueryCursorHandle::new();

            // TODO - add a Tree-sitter API to remove the need for this.
            let cursor = unsafe {
                std::mem::transmute::<_, &'static mut QueryCursor>(query_cursor.deref_mut())
            };

            cursor.set_byte_range(range.clone());
            let captures = cursor.captures(query, layer.node(), TextProvider(text));
            let grammar_index = result
                .grammars
                .iter()
                .position(|g| g.id == grammar.id())
                .unwrap_or_else(|| {
                    result.grammars.push(grammar);
                    result.grammars.len() - 1
                });
            let mut layer = SyntaxMapCapturesLayer {
                depth: layer.depth,
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
        layers: impl Iterator<Item = SyntaxLayer<'a>>,
        query: fn(&Grammar) -> Option<&Query>,
    ) -> Self {
        let mut result = Self::default();
        for layer in layers {
            let grammar = match &layer.language.grammar {
                Some(grammar) => grammar,
                None => continue,
            };
            let query = match query(grammar) {
                Some(query) => query,
                None => continue,
            };

            let mut query_cursor = QueryCursorHandle::new();

            // TODO - add a Tree-sitter API to remove the need for this.
            let cursor = unsafe {
                std::mem::transmute::<_, &'static mut QueryCursor>(query_cursor.deref_mut())
            };

            cursor.set_byte_range(range.clone());
            let matches = cursor.matches(query, layer.node(), TextProvider(text));
            let grammar_index = result
                .grammars
                .iter()
                .position(|g| g.id == grammar.id())
                .unwrap_or_else(|| {
                    result.grammars.push(grammar);
                    result.grammars.len() - 1
                });
            let mut layer = SyntaxMapMatchesLayer {
                depth: layer.depth,
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
        } else if self.active_layer_count != 0 {
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
            self.next_captures.extend_from_slice(mat.captures);
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
    ranges: Vec<tree_sitter::Range>,
    old_tree: Option<Tree>,
) -> Tree {
    PARSER.with(|parser| {
        let mut parser = parser.borrow_mut();
        let mut chunks = text.chunks_in_range(start_byte..text.len());
        parser
            .set_included_ranges(&ranges)
            .expect("overlapping ranges");
        parser
            .set_language(&grammar.ts_language)
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

#[allow(clippy::too_many_arguments)]
fn get_injections(
    config: &InjectionConfig,
    text: &BufferSnapshot,
    outer_range: Range<Anchor>,
    node: Node,
    language_registry: &Arc<LanguageRegistry>,
    depth: usize,
    changed_ranges: &[Range<usize>],
    combined_injection_ranges: &mut HashMap<Arc<Language>, Vec<tree_sitter::Range>>,
    queue: &mut BinaryHeap<ParseStep>,
) {
    let mut query_cursor = QueryCursorHandle::new();
    let mut prev_match = None;

    // Ensure that a `ParseStep` is created for every combined injection language, even
    // if there currently no matches for that injection.
    combined_injection_ranges.clear();
    for pattern in &config.patterns {
        if let (Some(language_name), true) = (pattern.language.as_ref(), pattern.combined) {
            if let Some(language) = language_registry
                .language_for_name_or_extension(language_name)
                .now_or_never()
                .and_then(|language| language.ok())
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

            let content_range =
                content_ranges.first().unwrap().start_byte..content_ranges.last().unwrap().end_byte;

            // Avoid duplicate matches if two changed ranges intersect the same injection.
            if let Some((prev_pattern_ix, prev_range)) = &prev_match {
                if mat.pattern_index == *prev_pattern_ix && content_range == *prev_range {
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
                let language = language_registry
                    .language_for_name_or_extension(&language_name)
                    .now_or_never()
                    .and_then(|language| language.ok());
                let range = text.anchor_before(step_range.start)..text.anchor_after(step_range.end);
                if let Some(language) = language {
                    if combined {
                        combined_injection_ranges
                            .entry(language.clone())
                            .or_default()
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
        included_ranges.sort_unstable_by(|a, b| {
            Ord::cmp(&a.start_byte, &b.start_byte).then_with(|| Ord::cmp(&a.end_byte, &b.end_byte))
        });
        queue.push(ParseStep {
            depth,
            language: ParseStepLanguage::Loaded { language },
            range: outer_range.clone(),
            included_ranges,
            mode: ParseMode::Combined {
                parent_layer_range: node.start_byte()..node.end_byte(),
                parent_layer_changed_ranges: changed_ranges.to_vec(),
            },
        })
    }
}

/// Updates the given list of included `ranges`, removing any ranges that intersect
/// `removed_ranges`, and inserting the given `new_ranges`.
///
/// Returns a new vector of ranges, and the range of the vector that was changed,
/// from the previous `ranges` vector.
pub(crate) fn splice_included_ranges(
    mut ranges: Vec<tree_sitter::Range>,
    removed_ranges: &[Range<usize>],
    new_ranges: &[tree_sitter::Range],
) -> (Vec<tree_sitter::Range>, Range<usize>) {
    let mut removed_ranges = removed_ranges.iter().cloned().peekable();
    let mut new_ranges = new_ranges.into_iter().cloned().peekable();
    let mut ranges_ix = 0;
    let mut changed_portion = usize::MAX..0;
    loop {
        let next_new_range = new_ranges.peek();
        let next_removed_range = removed_ranges.peek();

        let (remove, insert) = match (next_removed_range, next_new_range) {
            (None, None) => break,
            (Some(_), None) => (removed_ranges.next().unwrap(), None),
            (Some(next_removed_range), Some(next_new_range)) => {
                if next_removed_range.end < next_new_range.start_byte {
                    (removed_ranges.next().unwrap(), None)
                } else {
                    let mut start = next_new_range.start_byte;
                    let mut end = next_new_range.end_byte;

                    while let Some(next_removed_range) = removed_ranges.peek() {
                        if next_removed_range.start > next_new_range.end_byte {
                            break;
                        }
                        let next_removed_range = removed_ranges.next().unwrap();
                        start = cmp::min(start, next_removed_range.start);
                        end = cmp::max(end, next_removed_range.end);
                    }

                    (start..end, Some(new_ranges.next().unwrap()))
                }
            }
            (None, Some(next_new_range)) => (
                next_new_range.start_byte..next_new_range.end_byte,
                Some(new_ranges.next().unwrap()),
            ),
        };

        let mut start_ix = ranges_ix
            + match ranges[ranges_ix..].binary_search_by_key(&remove.start, |r| r.end_byte) {
                Ok(ix) => ix,
                Err(ix) => ix,
            };
        let mut end_ix = ranges_ix
            + match ranges[ranges_ix..].binary_search_by_key(&remove.end, |r| r.start_byte) {
                Ok(ix) => ix + 1,
                Err(ix) => ix,
            };

        // If there are empty ranges, then there may be multiple ranges with the same
        // start or end. Expand the splice to include any adjacent ranges that touch
        // the changed range.
        while start_ix > 0 {
            if ranges[start_ix - 1].end_byte == remove.start {
                start_ix -= 1;
            } else {
                break;
            }
        }
        while let Some(range) = ranges.get(end_ix) {
            if range.start_byte == remove.end {
                end_ix += 1;
            } else {
                break;
            }
        }

        changed_portion.start = changed_portion.start.min(start_ix);
        changed_portion.end = changed_portion.end.max(if insert.is_some() {
            start_ix + 1
        } else {
            start_ix
        });

        ranges.splice(start_ix..end_ix, insert);
        ranges_ix = start_ix;
    }

    if changed_portion.end < changed_portion.start {
        changed_portion = 0..0;
    }

    (ranges, changed_portion)
}

/// Ensure there are newline ranges in between content range that appear on
/// different lines. For performance, only iterate through the given range of
/// indices. All of the ranges in the array are relative to a given start byte
/// and point.
fn insert_newlines_between_ranges(
    indices: Range<usize>,
    ranges: &mut Vec<tree_sitter::Range>,
    text: &text::BufferSnapshot,
    start_byte: usize,
    start_point: Point,
) {
    let mut ix = indices.end + 1;
    while ix > indices.start {
        ix -= 1;
        if 0 == ix || ix == ranges.len() {
            continue;
        }

        let range_b = ranges[ix];
        let range_a = &mut ranges[ix - 1];
        if range_a.end_point.column == 0 {
            continue;
        }

        if range_a.end_point.row < range_b.start_point.row {
            let end_point = start_point + Point::from_ts_point(range_a.end_point);
            let line_end = Point::new(end_point.row, text.line_len(end_point.row));
            if end_point.column >= line_end.column {
                range_a.end_byte += 1;
                range_a.end_point.row += 1;
                range_a.end_point.column = 0;
            } else {
                let newline_offset = text.point_to_offset(line_end);
                ranges.insert(
                    ix,
                    tree_sitter::Range {
                        start_byte: newline_offset - start_byte,
                        end_byte: newline_offset - start_byte + 1,
                        start_point: (line_end - start_point).to_ts_point(),
                        end_point: ((line_end - start_point) + Point::new(1, 0)).to_ts_point(),
                    },
                )
            }
        }
    }
}

impl OwnedSyntaxLayer {
    /// Returns the root syntax node for this layer.
    pub fn node(&self) -> Node {
        self.tree
            .root_node_with_offset(self.offset.0, self.offset.1)
    }
}

impl<'a> SyntaxLayer<'a> {
    /// Returns an owned version of this layer.
    pub fn to_owned(&self) -> OwnedSyntaxLayer {
        OwnedSyntaxLayer {
            tree: self.tree.clone(),
            offset: self.offset,
            language: self.language.clone(),
        }
    }

    /// Returns the root node for this layer.
    pub fn node(&self) -> Node<'a> {
        self.tree
            .root_node_with_offset(self.offset.0, self.offset.1)
    }

    pub(crate) fn override_id(&self, offset: usize, text: &text::BufferSnapshot) -> Option<u32> {
        let text = TextProvider(text.as_rope());
        let config = self.language.grammar.as_ref()?.override_config.as_ref()?;

        let mut query_cursor = QueryCursorHandle::new();
        query_cursor.set_byte_range(offset..offset);

        let mut smallest_match: Option<(u32, Range<usize>)> = None;
        for mat in query_cursor.matches(&config.query, self.node(), text) {
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
        Some(self.cmp(other))
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

    fn intersects(&self, layer: &SyntaxLayerEntry, text: &BufferSnapshot) -> bool {
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
            if self.range == (Anchor::MAX..Anchor::MAX) {
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

impl sum_tree::Item for SyntaxLayerEntry {
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

impl std::fmt::Debug for SyntaxLayerEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxLayer")
            .field("depth", &self.depth)
            .field("range", &self.range)
            .field("tree", &self.content.tree())
            .finish()
    }
}

impl<'a> tree_sitter::TextProvider<&'a [u8]> for TextProvider<'a> {
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

struct LogIncludedRanges<'a>(&'a [tree_sitter::Range]);
struct LogPoint(Point);
struct LogAnchorRange<'a>(&'a Range<Anchor>, &'a text::BufferSnapshot);
struct LogChangedRegions<'a>(&'a ChangeRegionSet, &'a text::BufferSnapshot);

impl<'a> fmt::Debug for LogIncludedRanges<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.0.iter().map(|range| {
                let start = range.start_point;
                let end = range.end_point;
                (start.row, start.column)..(end.row, end.column)
            }))
            .finish()
    }
}

impl<'a> fmt::Debug for LogAnchorRange<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let range = self.0.to_point(self.1);
        (LogPoint(range.start)..LogPoint(range.end)).fmt(f)
    }
}

impl<'a> fmt::Debug for LogChangedRegions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(
                self.0
                     .0
                    .iter()
                    .map(|region| LogAnchorRange(&region.range, self.1)),
            )
            .finish()
    }
}

impl fmt::Debug for LogPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0.row, self.0.column).fmt(f)
    }
}
