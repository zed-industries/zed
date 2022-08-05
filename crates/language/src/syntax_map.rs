use crate::{
    Grammar, Language, LanguageRegistry, QueryCursorHandle, TextProvider, ToTreeSitterPoint,
};
use collections::VecDeque;
use gpui::executor::Background;
use std::{borrow::Cow, cell::RefCell, cmp::Ordering, ops::Range, sync::Arc};
use sum_tree::{SeekTarget, SumTree};
use text::{Anchor, BufferSnapshot, Point, Rope, ToOffset};
use tree_sitter::{Parser, Tree};
use util::post_inc;

thread_local! {
    static PARSER: RefCell<Parser> = RefCell::new(Parser::new());
}

#[derive(Default)]
pub struct SyntaxMap {
    next_layer_id: usize,
    snapshot: SyntaxMapSnapshot,
}

#[derive(Clone, Default)]
pub struct SyntaxMapSnapshot {
    version: clock::Global,
    layers: SumTree<SyntaxLayer>,
}

#[derive(Clone)]
struct SyntaxLayer {
    id: usize,
    parent_id: Option<usize>,
    range: SyntaxLayerRange,
    tree: tree_sitter::Tree,
    language: Arc<Language>,
}

#[derive(Debug, Clone)]
struct SyntaxLayerSummary {
    range: Range<Anchor>,
    last_layer_range: Range<Anchor>,
}

#[derive(Clone, Debug)]
struct SyntaxLayerRange(Range<Anchor>);

impl SyntaxMap {
    pub fn new(
        executor: Arc<Background>,
        registry: Arc<LanguageRegistry>,
        language: Arc<Language>,
        text: BufferSnapshot,
        prev_set: Option<Self>,
    ) -> Self {
        let mut next_layer_id = 0;
        let mut layers = Vec::new();
        let mut injections = VecDeque::<(Option<usize>, _, Vec<tree_sitter::Range>)>::new();

        injections.push_back((None, language, vec![]));
        while let Some((parent_id, language, ranges)) = injections.pop_front() {
            if let Some(grammar) = &language.grammar.as_deref() {
                let id = post_inc(&mut next_layer_id);
                let range = if let Some((first, last)) = ranges.first().zip(ranges.last()) {
                    text.anchor_before(first.start_byte)..text.anchor_after(last.end_byte)
                } else {
                    Anchor::MIN..Anchor::MAX
                };
                let tree = Self::parse_text(grammar, text.as_rope(), None, ranges);
                Self::get_injections(grammar, &text, &tree, id, &registry, &mut injections);
                layers.push(SyntaxLayer {
                    id,
                    parent_id,
                    range: SyntaxLayerRange(range),
                    tree,
                    language,
                });
            }
        }

        layers.sort_unstable_by(|a, b| SeekTarget::cmp(&a.range, &b.range, &text));

        Self {
            next_layer_id,
            snapshot: SyntaxMapSnapshot {
                layers: SumTree::from_iter(layers, &text),
                version: text.version,
            },
        }
    }

    pub fn snapshot(&self) -> SyntaxMapSnapshot {
        self.snapshot.clone()
    }

    fn interpolate(&mut self, text: &BufferSnapshot) {
        let edits = text
            .edits_since::<(Point, usize)>(&self.version)
            .map(|edit| {
                let (lines, bytes) = edit.flatten();
                tree_sitter::InputEdit {
                    start_byte: bytes.new.start,
                    old_end_byte: bytes.new.start + bytes.old.len(),
                    new_end_byte: bytes.new.end,
                    start_position: lines.new.start.to_ts_point(),
                    old_end_position: (lines.new.start + (lines.old.end - lines.old.start))
                        .to_ts_point(),
                    new_end_position: lines.new.end.to_ts_point(),
                }
            })
            .collect::<Vec<_>>();
        if edits.is_empty() {
            return;
        }
    }

    fn get_injections(
        grammar: &Grammar,
        text: &BufferSnapshot,
        tree: &Tree,
        id: usize,
        registry: &Arc<LanguageRegistry>,
        output: &mut VecDeque<(Option<usize>, Arc<Language>, Vec<tree_sitter::Range>)>,
    ) {
        let config = if let Some(config) = &grammar.injection_config {
            config
        } else {
            return;
        };

        let mut query_cursor = QueryCursorHandle::new();
        for mat in query_cursor.matches(
            &config.query,
            tree.root_node(),
            TextProvider(text.as_rope()),
        ) {
            let content_ranges = mat
                .nodes_for_capture_index(config.content_capture_ix)
                .map(|node| node.range())
                .collect::<Vec<_>>();
            if content_ranges.is_empty() {
                continue;
            }
            let language_name = config.languages_by_pattern_ix[mat.pattern_index]
                .as_ref()
                .map(|s| Cow::Borrowed(s.as_ref()))
                .or_else(|| {
                    let ix = config.language_capture_ix?;
                    let node = mat.nodes_for_capture_index(ix).next()?;
                    Some(Cow::Owned(text.text_for_range(node.byte_range()).collect()))
                });
            if let Some(language_name) = language_name {
                if let Some(language) = registry.get_language(language_name.as_ref()) {
                    output.push_back((Some(id), language, content_ranges))
                }
            }
        }
    }

    fn parse_text(
        grammar: &Grammar,
        text: &Rope,
        old_tree: Option<Tree>,
        ranges: Vec<tree_sitter::Range>,
    ) -> Tree {
        PARSER.with(|parser| {
            let mut parser = parser.borrow_mut();
            let mut chunks = text.chunks_in_range(0..text.len());
            parser
                .set_included_ranges(&ranges)
                .expect("overlapping ranges");
            parser
                .set_language(grammar.ts_language)
                .expect("incompatible grammar");
            parser
                .parse_with(
                    &mut move |offset, _| {
                        chunks.seek(offset);
                        chunks.next().unwrap_or("").as_bytes()
                    },
                    old_tree.as_ref(),
                )
                .expect("invalid language")
        })
    }
}

impl SyntaxMapSnapshot {
    pub fn layers_for_range<'a, T: ToOffset>(
        &self,
        range: Range<T>,
        buffer: &BufferSnapshot,
    ) -> Vec<(Tree, &Grammar)> {
        let start = buffer.anchor_before(range.start.to_offset(buffer));
        let end = buffer.anchor_after(range.end.to_offset(buffer));

        let mut cursor = self.layers.filter::<_, ()>(|summary| {
            let is_before_start = summary.range.end.cmp(&start, buffer).is_lt();
            let is_after_end = summary.range.start.cmp(&end, buffer).is_gt();
            !is_before_start && !is_after_end
        });

        let mut result = Vec::new();
        cursor.next(buffer);
        while let Some(item) = cursor.item() {
            if let Some(grammar) = &item.language.grammar {
                result.push((item.tree.clone(), grammar.as_ref()));
            }
            cursor.next(buffer)
        }

        result
    }
}

impl std::ops::Deref for SyntaxMap {
    type Target = SyntaxMapSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl Default for SyntaxLayerSummary {
    fn default() -> Self {
        Self {
            range: Anchor::MAX..Anchor::MIN,
            last_layer_range: Anchor::MIN..Anchor::MAX,
        }
    }
}

impl sum_tree::Summary for SyntaxLayerSummary {
    type Context = BufferSnapshot;

    fn add_summary(&mut self, other: &Self, buffer: &Self::Context) {
        if other.range.start.cmp(&self.range.start, buffer).is_lt() {
            self.range.start = other.range.start;
        }
        if other.range.end.cmp(&self.range.end, buffer).is_gt() {
            self.range.end = other.range.end;
        }
        self.last_layer_range = other.last_layer_range.clone();
    }
}

impl Default for SyntaxLayerRange {
    fn default() -> Self {
        Self(Anchor::MIN..Anchor::MAX)
    }
}

impl<'a> SeekTarget<'a, SyntaxLayerSummary, SyntaxLayerRange> for SyntaxLayerRange {
    fn cmp(&self, cursor_location: &Self, buffer: &BufferSnapshot) -> Ordering {
        self.0
            .start
            .cmp(&cursor_location.0.start, buffer)
            .then_with(|| cursor_location.0.end.cmp(&self.0.end, buffer))
    }
}

impl<'a> sum_tree::Dimension<'a, SyntaxLayerSummary> for SyntaxLayerRange {
    fn add_summary(
        &mut self,
        summary: &'a SyntaxLayerSummary,
        _: &<SyntaxLayerSummary as sum_tree::Summary>::Context,
    ) {
        self.0 = summary.last_layer_range.clone();
    }
}

impl sum_tree::Item for SyntaxLayer {
    type Summary = SyntaxLayerSummary;

    fn summary(&self) -> Self::Summary {
        SyntaxLayerSummary {
            range: self.range.0.clone(),
            last_layer_range: self.range.0.clone(),
        }
    }
}

impl std::fmt::Debug for SyntaxLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxLayer")
            .field("id", &self.id)
            .field("parent_id", &self.parent_id)
            .field("range", &self.range)
            .field("tree", &self.tree)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LanguageConfig;
    use gpui::MutableAppContext;
    use text::{Buffer, Point};
    use unindent::Unindent as _;

    #[gpui::test]
    fn test_syntax_map(cx: &mut MutableAppContext) {
        let buffer = Buffer::new(
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

        let executor = cx.background().clone();
        let registry = Arc::new(LanguageRegistry::test());
        let language = Arc::new(rust_lang());
        let snapshot = buffer.snapshot();
        registry.add(language.clone());

        let syntax_map = SyntaxMap::new(executor, registry, language, snapshot.clone(), None);

        let layers = syntax_map.layers_for_range(Point::new(0, 0)..Point::new(0, 1), &snapshot);
        assert_layers(
            &layers,
            &["(source_file (function_item name: (identifier)..."],
        );

        let layers = syntax_map.layers_for_range(Point::new(2, 0)..Point::new(2, 0), &snapshot);
        assert_layers(
            &layers,
            &[
                "...(function_item ... (block (expression_statement (macro_invocation...",
                "...(tuple_expression (call_expression ... arguments: (arguments (macro_invocation...",
            ],
        );

        let layers = syntax_map.layers_for_range(Point::new(2, 14)..Point::new(2, 16), &snapshot);
        assert_layers(
            &layers,
            &[
                "...(function_item ...",
                "...(tuple_expression (call_expression ... arguments: (arguments (macro_invocation...",
                "...(array_expression (struct_expression ...",
            ],
        );

        let layers = syntax_map.layers_for_range(Point::new(3, 14)..Point::new(3, 16), &snapshot);
        assert_layers(
            &layers,
            &[
                "...(function_item ...",
                "...(tuple_expression (call_expression ... arguments: (arguments (macro_invocation...",
                "...(array_expression (field_expression ...",
            ],
        );

        let layers = syntax_map.layers_for_range(Point::new(5, 12)..Point::new(5, 16), &snapshot);
        assert_layers(
            &layers,
            &[
                "...(function_item ...",
                "...(call_expression ... (arguments (closure_expression ...",
            ],
        );
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

    fn assert_layers(layers: &[(Tree, &Grammar)], expected_layers: &[&str]) {
        assert_eq!(
            layers.len(),
            expected_layers.len(),
            "wrong number of layers"
        );
        for (i, (layer, expected_s_exp)) in layers.iter().zip(expected_layers.iter()).enumerate() {
            let actual_s_exp = layer.0.root_node().to_sexp();
            assert!(
                string_contains_sequence(
                    &actual_s_exp,
                    &expected_s_exp.split("...").collect::<Vec<_>>()
                ),
                "layer {i}:\n\nexpected: {expected_s_exp}\nactual:   {actual_s_exp}",
            );
        }
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
