use std::ops::Range;

use theme::{AccentColors, ActiveTheme as _};

use clock::Global;
use collections::{HashMap, HashSet};
use gpui::{Context, HighlightStyle, Window};
use language::{Bias, BufferSnapshot, Point, QueryCapture, RainbowConfig};
use multi_buffer::{Anchor, ExcerptId, MultiBufferSnapshot};
use text::BufferId;

use crate::{DisplayPoint, DisplayRow, Editor};

const ROW_OVERSCAN: u32 = 64;

pub(super) struct RainbowHighlightState {
    cache: RainbowHighlightCache,
    active_color_keys: Vec<usize>,
}

impl Default for RainbowHighlightState {
    fn default() -> Self {
        Self {
            cache: RainbowHighlightCache::default(),
            active_color_keys: Vec::new(),
        }
    }
}

impl RainbowHighlightState {
    pub(super) fn invalidate_all(&mut self) {
        self.cache.clear();
        self.active_color_keys.clear();
    }

    pub(super) fn invalidate_buffer(&mut self, buffer_id: BufferId) {
        self.cache.invalidate_buffer(buffer_id);
    }

    pub(super) fn invalidate_excerpts(&mut self, excerpt_ids: &[ExcerptId]) {
        self.cache.invalidate_excerpts(excerpt_ids);
    }
}

#[derive(Default)]
struct RainbowHighlightCache {
    entries: HashMap<ExcerptId, RainbowCacheEntry>,
}

impl RainbowHighlightCache {
    fn get(
        &self,
        excerpt_id: &ExcerptId,
        buffer_id: BufferId,
        version: &Global,
        visible_ranges: &[Range<usize>],
        color_count: usize,
    ) -> Option<&RainbowCacheEntry> {
        let entry = self.entries.get(excerpt_id)?;
        (entry.buffer_id == buffer_id
            && entry.buffer_version == *version
            && entry.color_count == color_count
            && entry.visible_ranges == visible_ranges)
            .then_some(entry)
    }

    fn insert(&mut self, excerpt_id: ExcerptId, entry: RainbowCacheEntry) {
        self.entries.insert(excerpt_id, entry);
    }

    fn invalidate_buffer(&mut self, buffer_id: BufferId) {
        self.entries.retain(|_, entry| entry.buffer_id != buffer_id);
    }

    fn invalidate_excerpts(&mut self, excerpt_ids: &[ExcerptId]) {
        for excerpt_id in excerpt_ids {
            self.entries.remove(excerpt_id);
        }
    }

    fn clear(&mut self) {
        self.entries.clear();
    }
}

struct RainbowCacheEntry {
    buffer_id: BufferId,
    buffer_version: Global,
    visible_ranges: Vec<Range<usize>>,
    color_count: usize,
    ranges_by_color: Vec<Vec<Range<Anchor>>>,
}

impl RainbowCacheEntry {
    fn new(
        buffer_id: BufferId,
        buffer_version: Global,
        visible_ranges: Vec<Range<usize>>,
        color_count: usize,
        ranges_by_color: Vec<Vec<Range<Anchor>>>,
    ) -> Self {
        Self {
            buffer_id,
            buffer_version,
            visible_ranges,
            color_count,
            ranges_by_color,
        }
    }

    fn has_highlights(&self) -> bool {
        self.ranges_by_color.iter().any(|ranges| !ranges.is_empty())
    }
}

struct QueryExcerpt<'a> {
    buffer_snapshot: &'a BufferSnapshot,
    range: Range<usize>,
}

struct ActiveScope {
    end_byte: usize,
    level: usize,
    node_id: Option<usize>,
}

struct RainbowViewport {
    buffer_snapshot: MultiBufferSnapshot,
    visible_points: Range<Point>,
    query_points: Range<Point>,
}

pub(super) struct RainbowBracketHighlight;

impl Editor {
    pub fn refresh_rainbow_bracket_highlights(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(viewport) = self.build_rainbow_viewport(cx) else {
            self.reset_rainbow_state(cx);
            return;
        };

        let visible_ranges_by_excerpt = collect_visible_ranges_by_excerpt(
            &viewport.buffer_snapshot,
            viewport.visible_points.clone(),
        );
        if visible_ranges_by_excerpt.is_empty() {
            self.clear_rainbow_bracket_highlights(cx);
            return;
        }

        let query_excerpts =
            collect_query_excerpts(&viewport.buffer_snapshot, viewport.query_points.clone());
        if query_excerpts.is_empty() {
            self.clear_rainbow_bracket_highlights(cx);
            return;
        }

        let accent_colors = cx.theme().accents().clone();
        let color_count = accent_colors.0.len();
        if color_count == 0 {
            self.reset_rainbow_state(cx);
            return;
        }

        let (mut aggregated_ranges, had_any_highlights) = self.aggregate_highlight_ranges(
            query_excerpts,
            &visible_ranges_by_excerpt,
            color_count,
        );

        if !had_any_highlights {
            self.clear_rainbow_bracket_highlights(cx);
            return;
        }

        let used_keys = self.apply_rainbow_highlights(&mut aggregated_ranges, &accent_colors, cx);
        self.reconcile_rainbow_highlight_keys(used_keys, cx);
    }

    pub fn clear_rainbow_bracket_highlights(&mut self, cx: &mut Context<Self>) {
        let keys = std::mem::take(&mut self.rainbow_highlight_state.active_color_keys);
        for key in keys {
            self.clear_highlight_key::<RainbowBracketHighlight>(key, cx);
        }
    }

    fn build_rainbow_viewport(&mut self, cx: &mut Context<Self>) -> Option<RainbowViewport> {
        let display_snapshot = self.display_snapshot(cx);
        if display_snapshot.is_empty() {
            return None;
        }

        let Some(visible_line_count) = self.visible_line_count() else {
            return None;
        };

        let buffer_snapshot = self.buffer().read(cx).snapshot(cx);
        let scroll_position = self
            .scroll_manager
            .anchor()
            .scroll_position(&display_snapshot);
        let max_display_row = display_snapshot.max_point().row().0;
        let start_row = if scroll_position.y.is_sign_negative() {
            0
        } else {
            scroll_position.y.floor() as u32
        };
        let mut end_row = (scroll_position.y + visible_line_count).ceil() as u32;
        if end_row < start_row {
            end_row = start_row;
        }
        end_row = end_row.min(max_display_row);

        let query_start_row = start_row.saturating_sub(ROW_OVERSCAN);
        let query_end_row = (end_row + ROW_OVERSCAN).min(max_display_row);

        let visible_start_point = display_snapshot
            .display_point_to_point(DisplayPoint::new(DisplayRow(start_row), 0), Bias::Left);
        let visible_end_point = display_snapshot
            .display_point_to_point(DisplayPoint::new(DisplayRow(end_row), 0), Bias::Right);
        if visible_start_point == visible_end_point {
            return None;
        }

        let query_start_point = display_snapshot.display_point_to_point(
            DisplayPoint::new(DisplayRow(query_start_row), 0),
            Bias::Left,
        );
        let query_end_point = display_snapshot
            .display_point_to_point(DisplayPoint::new(DisplayRow(query_end_row), 0), Bias::Right);

        Some(RainbowViewport {
            buffer_snapshot,
            visible_points: visible_start_point..visible_end_point,
            query_points: query_start_point..query_end_point,
        })
    }

    fn aggregate_highlight_ranges<'a>(
        &mut self,
        query_excerpts: HashMap<ExcerptId, QueryExcerpt<'a>>,
        visible_ranges_by_excerpt: &HashMap<ExcerptId, Vec<Range<usize>>>,
        color_count: usize,
    ) -> (Vec<Vec<Range<Anchor>>>, bool) {
        let mut aggregated_ranges = vec![Vec::new(); color_count];
        let mut had_any_highlights = false;

        for (excerpt_id, query_entry) in query_excerpts {
            let Some(visible_slices) = visible_ranges_by_excerpt.get(&excerpt_id) else {
                continue;
            };
            if visible_slices.is_empty() {
                continue;
            }

            let buffer_id = query_entry.buffer_snapshot.remote_id();
            let buffer_version = query_entry.buffer_snapshot.version().clone();

            if let Some(entry) = self.rainbow_highlight_state.cache.get(
                &excerpt_id,
                buffer_id,
                &buffer_version,
                visible_slices,
                color_count,
            ) {
                extend_color_ranges(&mut aggregated_ranges, &entry.ranges_by_color);
                had_any_highlights |= entry.has_highlights();
                continue;
            }

            let Some(ranges_by_color) = compute_excerpt_highlights(
                query_entry.buffer_snapshot,
                query_entry.range.clone(),
                visible_slices,
                excerpt_id,
                color_count,
            ) else {
                self.rainbow_highlight_state
                    .cache
                    .invalidate_excerpts(&[excerpt_id]);
                continue;
            };

            let entry = RainbowCacheEntry::new(
                buffer_id,
                buffer_version,
                visible_slices.clone(),
                color_count,
                ranges_by_color,
            );
            had_any_highlights |= entry.has_highlights();
            extend_color_ranges(&mut aggregated_ranges, &entry.ranges_by_color);
            self.rainbow_highlight_state.cache.insert(excerpt_id, entry);
        }

        (aggregated_ranges, had_any_highlights)
    }

    fn apply_rainbow_highlights(
        &mut self,
        aggregated_ranges: &mut [Vec<Range<Anchor>>],
        accent_colors: &AccentColors,
        cx: &mut Context<Self>,
    ) -> Vec<usize> {
        let mut used_keys = Vec::new();
        for (color_idx, ranges) in aggregated_ranges.iter_mut().enumerate() {
            if ranges.is_empty() {
                continue;
            }
            used_keys.push(color_idx);
            let style = HighlightStyle {
                color: Some(accent_colors.color_for_index(color_idx as u32)),
                ..Default::default()
            };
            let highlight_ranges = std::mem::take(ranges);
            self.highlight_text_key::<RainbowBracketHighlight>(
                color_idx,
                highlight_ranges,
                style,
                cx,
            );
        }
        used_keys
    }

    fn reconcile_rainbow_highlight_keys(&mut self, used_keys: Vec<usize>, cx: &mut Context<Self>) {
        let previous_keys: HashSet<_> = self
            .rainbow_highlight_state
            .active_color_keys
            .iter()
            .copied()
            .collect();
        let current_keys: HashSet<_> = used_keys.iter().copied().collect();
        for key in previous_keys.difference(&current_keys) {
            self.clear_highlight_key::<RainbowBracketHighlight>(*key, cx);
        }
        self.rainbow_highlight_state.active_color_keys = used_keys;
    }

    fn reset_rainbow_state(&mut self, cx: &mut Context<Self>) {
        self.clear_rainbow_bracket_highlights(cx);
        self.rainbow_highlight_state.invalidate_all();
    }
}

fn collect_visible_ranges_by_excerpt(
    buffer_snapshot: &MultiBufferSnapshot,
    visible_points: Range<Point>,
) -> HashMap<ExcerptId, Vec<Range<usize>>> {
    let mut ranges_by_excerpt: HashMap<ExcerptId, Vec<Range<usize>>> = HashMap::default();
    for (_, range, excerpt_id) in buffer_snapshot.range_to_buffer_ranges(visible_points) {
        if range.is_empty() {
            continue;
        }
        ranges_by_excerpt.entry(excerpt_id).or_default().push(range);
    }
    ranges_by_excerpt
}

fn collect_query_excerpts<'a>(
    buffer_snapshot: &'a MultiBufferSnapshot,
    query_points: Range<Point>,
) -> HashMap<ExcerptId, QueryExcerpt<'a>> {
    let mut query_excerpts: HashMap<ExcerptId, QueryExcerpt<'a>> = HashMap::default();
    for (buffer, range, excerpt_id) in buffer_snapshot.range_to_buffer_ranges(query_points) {
        if range.is_empty() {
            continue;
        }
        query_excerpts
            .entry(excerpt_id)
            .and_modify(|entry| {
                entry.range.start = entry.range.start.min(range.start);
                entry.range.end = entry.range.end.max(range.end);
            })
            .or_insert(QueryExcerpt {
                buffer_snapshot: buffer,
                range,
            });
    }
    query_excerpts
}

fn extend_color_ranges(target: &mut [Vec<Range<Anchor>>], source: &[Vec<Range<Anchor>>]) {
    for (idx, ranges) in source.iter().enumerate() {
        if ranges.is_empty() {
            continue;
        }
        if let Some(destination) = target.get_mut(idx) {
            destination.extend(ranges.iter().cloned());
        }
    }
}

fn compute_excerpt_highlights(
    buffer_snapshot: &BufferSnapshot,
    query_range: Range<usize>,
    visible_ranges: &[Range<usize>],
    excerpt_id: ExcerptId,
    color_count: usize,
) -> Option<Vec<Vec<Range<Anchor>>>> {
    if color_count == 0 || visible_ranges.is_empty() || query_range.is_empty() {
        return Some(vec![Vec::new(); color_count]);
    }

    let mut matches = buffer_snapshot.matches(query_range.clone(), |grammar| {
        grammar.rainbow_config().map(|config| &config.query)
    });
    let configs = matches
        .grammars()
        .iter()
        .map(|grammar| grammar.rainbow_config())
        .collect::<Vec<_>>();
    if !has_any_rainbow_configs(&configs) {
        return None;
    }

    let mut level_ranges = vec![Vec::new(); color_count];
    let mut scope_stack = Vec::<ActiveScope>::new();

    while let Some(mat) = matches.peek() {
        let Some(config) = config_for_match(&configs, mat.grammar_index) else {
            matches.advance();
            continue;
        };

        let capture_start = capture_start_byte(mat.captures, query_range.end);
        pop_completed_scopes(&mut scope_stack, capture_start);

        push_scope_captures(
            mat.captures,
            config.scope_capture_ix,
            config.patterns[mat.pattern_index].include_children,
            &mut scope_stack,
        );

        collect_bracket_captures(
            mat.captures,
            config,
            &scope_stack,
            buffer_snapshot,
            visible_ranges,
            excerpt_id,
            color_count,
            &mut level_ranges,
        );

        matches.advance();
    }

    Some(level_ranges)
}

fn is_range_visible(range: &Range<usize>, visible_ranges: &[Range<usize>]) -> bool {
    visible_ranges.iter().any(|visible| {
        range.start >= visible.start && range.end <= visible.end && range.start < range.end
    })
}

fn has_any_rainbow_configs(configs: &[Option<&RainbowConfig>]) -> bool {
    configs.iter().any(|config| config.is_some())
}

fn config_for_match<'a>(
    configs: &[Option<&'a RainbowConfig>],
    grammar_index: usize,
) -> Option<&'a RainbowConfig> {
    configs
        .get(grammar_index)
        .and_then(|config| config.as_ref().copied())
}

fn capture_start_byte(captures: &[QueryCapture<'_>], fallback: usize) -> usize {
    captures
        .iter()
        .map(|capture| capture.node.byte_range().start)
        .min()
        .unwrap_or(fallback)
}

fn pop_completed_scopes(scope_stack: &mut Vec<ActiveScope>, capture_start: usize) {
    while let Some(scope) = scope_stack.last() {
        if capture_start < scope.end_byte {
            break;
        }
        scope_stack.pop();
    }
}

fn push_scope_captures(
    captures: &[QueryCapture<'_>],
    scope_capture_ix: Option<u32>,
    include_children: bool,
    scope_stack: &mut Vec<ActiveScope>,
) {
    let Some(scope_capture_ix) = scope_capture_ix else {
        return;
    };

    for capture in captures
        .iter()
        .filter(|capture| capture.index == scope_capture_ix)
    {
        let node = capture.node;
        let level = scope_stack.last().map_or(0, |scope| scope.level + 1);
        scope_stack.push(ActiveScope {
            end_byte: node.end_byte(),
            level,
            node_id: include_children.then(|| node.id()),
        });
    }
}

fn collect_bracket_captures(
    captures: &[QueryCapture<'_>],
    config: &RainbowConfig,
    scope_stack: &[ActiveScope],
    buffer_snapshot: &BufferSnapshot,
    visible_ranges: &[Range<usize>],
    excerpt_id: ExcerptId,
    color_count: usize,
    level_ranges: &mut [Vec<Range<Anchor>>],
) {
    for capture in captures
        .iter()
        .filter(|capture| capture.index == config.bracket_capture_ix)
    {
        let scope = scope_stack.last();
        if scope.is_none() && config.scope_capture_ix.is_some() {
            continue;
        }
        let (scope_level, scope_node_id) = scope
            .map(|scope| (scope.level, scope.node_id))
            .unwrap_or((0, None));

        let node = capture.node;
        if let Some(scope_node_id) = scope_node_id {
            let Some(parent) = node.parent() else {
                continue;
            };
            if parent.id() != scope_node_id {
                continue;
            }
        }

        let byte_range = node.byte_range();
        if byte_range.is_empty() || !is_range_visible(&byte_range, visible_ranges) {
            continue;
        }

        let color_index = scope_level % color_count;
        let anchor_range = Anchor::range_in_buffer(
            excerpt_id,
            buffer_snapshot.remote_id(),
            buffer_snapshot.anchor_after(byte_range.start)
                ..buffer_snapshot.anchor_before(byte_range.end),
        );

        level_ranges[color_index].push(anchor_range);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, TestAppContext};
    use indoc::indoc;
    use language::{Buffer, Language, LanguageConfig, LanguageMatcher, LanguageQueries, Point};
    use multi_buffer::MultiBuffer;
    use std::{borrow::Cow, sync::Arc};
    use text::Rope;

    fn javascript_language_with_rainbow_query(query: Cow<'static, str>) -> Language {
        Language::new(
            LanguageConfig {
                name: "JavaScript".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["js".into()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
        )
        .with_queries(LanguageQueries {
            rainbow: Some(query),
            ..LanguageQueries::default()
        })
        .expect("failed to load rainbow query")
    }

    fn javascript_test_language() -> Language {
        javascript_language_with_rainbow_query(Cow::from(include_str!(
            "../../languages/src/javascript/brackets.scm"
        )))
    }

    fn javascript_bracket_only_language() -> Language {
        javascript_language_with_rainbow_query(Cow::from(indoc! {r#"
            [
              "("
              ")"
              "["
              "]"
              "{"
              "}"
            ] @rainbow.bracket
        "#}))
    }

    fn build_snapshot_for_test(
        cx: &mut TestAppContext,
        language: Arc<Language>,
        rope: Rope,
    ) -> BufferSnapshot {
        let mut app = cx.app.borrow_mut();
        Buffer::build_snapshot_sync(rope, Some(language), None, &mut *app)
    }

    #[gpui::test]
    async fn computes_bracket_levels(cx: &mut TestAppContext) {
        let language = Arc::new(javascript_test_language());
        let rope = Rope::from(indoc! {
            r#"
            function demo() {
                const data = [{ value: (items[0]) }];
            }
            "#
        });

        let snapshot = build_snapshot_for_test(cx, language, rope);

        let excerpt_id = ExcerptId::min();
        let visible = vec![0..snapshot.len()];
        let ranges =
            compute_excerpt_highlights(&snapshot, 0..snapshot.len(), &visible, excerpt_id, 4)
                .expect("missing rainbow config");

        let total_highlights: usize = ranges.iter().map(|r| r.len()).sum();
        assert!(
            total_highlights >= 8,
            "expected multiple highlighted brackets"
        );
        assert!(ranges[0].len() >= ranges[1].len());
    }

    #[gpui::test]
    async fn highlights_without_scope_captures(cx: &mut TestAppContext) {
        let language = Arc::new(javascript_bracket_only_language());
        let rope = Rope::from(indoc! {
            r#"
            (() => {
                return ({ nested: [value()] });
            })();
            "#
        });

        let snapshot = build_snapshot_for_test(cx, language, rope);
        let excerpt_id = ExcerptId::min();
        let visible = vec![0..snapshot.len()];
        let ranges =
            compute_excerpt_highlights(&snapshot, 0..snapshot.len(), &visible, excerpt_id, 4)
                .expect("missing rainbow config");

        let total_highlights: usize = ranges.iter().map(|r| r.len()).sum();
        assert!(
            total_highlights > 0,
            "expected brackets to be highlighted even without scopes"
        );
        assert!(
            ranges.iter().skip(1).all(|r| r.is_empty()),
            "without scopes, only the first color should be used"
        );
    }

    #[gpui::test]
    async fn nested_levels_cycle_colors(cx: &mut TestAppContext) {
        let language = Arc::new(javascript_test_language());
        let rope = Rope::from(indoc! {r#"
            function colors() {
                return [{ call: wrapper((alpha[beta()] + gamma)) }];
            }
        "#});

        let snapshot = build_snapshot_for_test(cx, language, rope);
        let excerpt_id = ExcerptId::min();
        let visible = vec![0..snapshot.len()];
        let ranges =
            compute_excerpt_highlights(&snapshot, 0..snapshot.len(), &visible, excerpt_id, 2)
                .expect("missing rainbow config");

        assert!(
            ranges[0].len() > 0 && ranges[1].len() > 0,
            "expected nested brackets to map to multiple colors"
        );
    }

    #[gpui::test]
    async fn filters_highlights_to_visible_ranges(cx: &mut TestAppContext) {
        let language = Arc::new(javascript_test_language());
        let rope = Rope::from("({})");
        let snapshot = build_snapshot_for_test(cx, language, rope);
        let excerpt_id = ExcerptId::min();
        let visible = vec![0..1];

        let ranges =
            compute_excerpt_highlights(&snapshot, 0..snapshot.len(), &visible, excerpt_id, 4)
                .expect("missing rainbow config");
        assert_eq!(
            ranges.iter().map(|r| r.len()).sum::<usize>(),
            1,
            "only the opening parenthesis should be emitted"
        );
    }

    #[gpui::test]
    async fn collects_visible_ranges_for_multiple_excerpts(cx: &mut TestAppContext) {
        let mut app = cx.app.borrow_mut();
        let multi = MultiBuffer::build_multi(
            [
                ("alpha()\n", vec![Point::new(0, 0)..Point::new(1, 0)]),
                ("beta[]\n", vec![Point::new(0, 0)..Point::new(1, 0)]),
            ],
            &mut *app,
        );
        drop(app);

        let snapshot = cx.read_entity(&multi, |multi, app| multi.snapshot(app));
        let visible_ranges =
            collect_visible_ranges_by_excerpt(&snapshot, Point::new(0, 0)..Point::MAX);
        let expected: Vec<_> = snapshot
            .excerpts()
            .map(|(id, buffer, _)| (id, buffer.len()))
            .collect();
        assert_eq!(visible_ranges.len(), expected.len());

        for (excerpt_id, buffer_len) in expected.iter().copied() {
            let ranges = visible_ranges
                .get(&excerpt_id)
                .expect("missing ranges for excerpt");
            let total_len: usize = ranges.iter().map(|r| r.end - r.start).sum();
            assert_eq!(total_len, buffer_len);
        }

        let query_entries = collect_query_excerpts(&snapshot, Point::new(0, 0)..Point::MAX);
        assert_eq!(query_entries.len(), visible_ranges.len());
        for (excerpt_id, entry) in query_entries {
            let ranges = visible_ranges
                .get(&excerpt_id)
                .expect("missing ranges for excerpt");
            let min_start = ranges.iter().map(|r| r.start).min().unwrap();
            let max_end = ranges.iter().map(|r| r.end).max().unwrap();
            assert_eq!(entry.range.start, min_start);
            assert_eq!(entry.range.end, max_end);
        }
    }

    #[gpui::test]
    async fn deep_nesting_uses_first_six_colors(cx: &mut TestAppContext) {
        let language = Arc::new(javascript_test_language());
        let rope = Rope::from(indoc! {r#"
            const deeplyNested = ((((((value))))));
        "#});

        let snapshot = build_snapshot_for_test(cx, language, rope);
        let excerpt_id = ExcerptId::min();
        let visible = vec![0..snapshot.len()];
        let ranges =
            compute_excerpt_highlights(&snapshot, 0..snapshot.len(), &visible, excerpt_id, 6)
                .expect("missing rainbow config");

        for (color_index, color_ranges) in ranges.iter().take(6).enumerate() {
            assert!(
                !color_ranges.is_empty(),
                "expected color {color_index} to have highlights at depth >= 6"
            );
        }
    }

    #[gpui::test]
    async fn deep_nesting_wraps_color_indices(cx: &mut TestAppContext) {
        let language = Arc::new(javascript_test_language());
        let rope = Rope::from(indoc! {r#"
            const wrapped = ((((((value))))));
        "#});

        let snapshot = build_snapshot_for_test(cx, language, rope);
        let excerpt_id = ExcerptId::min();
        let visible = vec![0..snapshot.len()];
        let ranges =
            compute_excerpt_highlights(&snapshot, 0..snapshot.len(), &visible, excerpt_id, 3)
                .expect("missing rainbow config");

        assert!(
            ranges[0].len() >= 2,
            "expected deepest levels to wrap back to the first color"
        );
        assert!(
            ranges.iter().flat_map(|r| r).count() > 3,
            "expected more highlights than available colors to verify wrapping"
        );
    }
}
